#![allow(dead_code)] // TODO remove after dev
#![allow(unused_variables)] // TODO remove after dev

use std::env::args;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::block_lists::BlockLists;
use crate::config::Config;
use crate::dot::DotProviders;

mod block_lists;
mod config;
mod dns_parser;
mod dot;
mod error;
mod server;

fn unpack_args() -> Vec<String> {
    let mut unpacked = Vec::new();
    for arg in args() {
        unpacked.push(arg);
    }

    unpacked
}

fn sync_thread(interval_mins: u64, block_lists: Arc<RwLock<BlockLists>>) {
    thread::spawn(move || {
        let mut instant = Instant::now();
        loop {
            if instant.elapsed().as_secs() > (interval_mins * 60) {
                {
                    if let Ok(lists) = &mut block_lists.write() {
                        println!("Updating block lists...");
                        if let Err(err) = lists.sync() {
                            eprintln!("Unable to update block lists. {}", err);
                        } else {
                            println!("Finished updating block lists");
                        }
                    }
                }
            }
            instant = Instant::now();
            thread::sleep(Duration::from_secs(5));
        }
    });
}

fn main() {
    let argv = unpack_args();

    if argv.len() < 2 {
        eprintln!("Usage: tinydnsproxy <path to config.toml file>");
        return;
    }

    let config_path = argv[1].clone();
    let config = match Config::from_toml_file(config_path.as_str()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut dot_providers = DotProviders::new();
    for p in &config.dot_provider {
        dot_providers.add(p.ip.clone(), p.port, p.hostname.clone(), None);
    }

    let mut block_lists = BlockLists::new();
    if let Some(http_block_lists) = &config.http_block_list {
        for l in http_block_lists {
            block_lists.add_http_list(l.url.as_str());
        }
    }
    if let Some(file_block_lists) = &config.file_block_list {
        for l in file_block_lists {
            block_lists.add_file_list(l.path.as_str());
        }
    }
    println!("Performing first block list sync...");
    match block_lists.sync() {
        Ok(()) => println!("Block list sync complete"),
        Err(e) => {
            eprintln!("Error syncing block lists for the first time: {}", e);
            return;
        }
    };

    let wrapped_block_list = Arc::new(RwLock::new(block_lists));

    sync_thread(
        config.general.refresh_blocklists_after,
        wrapped_block_list.clone(),
    );
    server::listen_and_serve(config, dot_providers, wrapped_block_list.clone());
}
