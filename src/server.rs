use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::sync::{Arc, RwLock};

use native_tls::TlsConnector;
use threadpool::ThreadPool;

use crate::block_lists::BlockLists;
use crate::config::Config;
use crate::dns_parser;
use crate::dot::{read, write, DotProviders};
use crate::error::Error;

const PACKET_BUFFER_SIZE: usize = 8192;

pub fn listen_and_serve(c: Config, dot: DotProviders, block_lists: Arc<RwLock<BlockLists>>) {
    let bind_string = format!("{}:{}", c.general.bind_ip, c.general.bind_port);
    let socket = match UdpSocket::bind(bind_string) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Couldn't open listening socket. Error: {}", e);
            return;
        }
    };

    let num_threads = c.general.worker_threads.unwrap_or(num_cpus::get());
    let pool = ThreadPool::new(num_threads);
    let arc_dot = Arc::new(dot);

    let mut packet_buffer: Vec<u8> = vec![0; PACKET_BUFFER_SIZE];

    loop {
        let (amt, src) = match socket.recv_from(packet_buffer.as_mut_slice()) {
            Ok(res) => res,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Unexpected socket error. Error: {}", e);
                return;
            }
        };

        let mut dns_buff = vec![0; amt];
        for i in 0..amt {
            dns_buff[i] = packet_buffer[i];
        }

        let dot_ref = arc_dot.clone();
        let cloned_socket = match socket.try_clone() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Couldn't clone socket to handle request. Error: {}", e);
                return;
            }
        };

        let block_lists_ref = block_lists.clone();

        pool.execute(move || {
            match handle_request(dns_buff, src, cloned_socket, &dot_ref, block_lists_ref) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Couldn't handle request: {}", e);
                    return;
                }
            };
        });

        for i in 0..PACKET_BUFFER_SIZE {
            packet_buffer[i] = 0x0;
        }
    }
}

fn handle_request(
    mut msg: Vec<u8>,
    from: SocketAddr,
    socket: UdpSocket,
    dot: &DotProviders,
    block_lists: Arc<RwLock<BlockLists>>,
) -> Result<(), Error> {
    // Before we do any network stuff, let's check to see if we should
    // block the request
    let hostname = dns_parser::get_requested_domain(msg.as_slice())?;
    let should_block = {
        if let Ok(lists) = block_lists.try_read() {
            lists.check(hostname.as_str())
        } else {
            false // Default don't block if we're not able to grab the mutex
        }
    };
    if should_block {
        dns_parser::tweak_to_nxdomain(&mut msg)?;
        socket.send_to(msg.as_slice(), from)?;
        return Ok(());
    }
    let dot = dot.get_random()?;
    let conn_string = format!("{}:{}", dot.ip, dot.port);
    let tcp_connection = TcpStream::connect(conn_string)?;
    let tls_connector = TlsConnector::new()?;
    let mut tls_connection = tls_connector.connect(dot.hostname.as_str(), tcp_connection)?;

    write(&mut tls_connection, &msg)?;
    let upstream_response = read(&mut tls_connection)?;

    socket.send_to(upstream_response.as_slice(), from)?;

    Ok(())
}
