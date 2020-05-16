use std::io::prelude::*;
use std::io::{Cursor, Seek, SeekFrom};

use byteorder::{NetworkEndian, ReadBytesExt};

use crate::error::Error;

pub fn get_requested_domain(packet: &[u8]) -> Result<String, Error> {
    let mut cursor = Cursor::new(packet);

    // Jump to the questions section
    cursor.seek(SeekFrom::Start(4))?;

    let questions = cursor.read_u16::<NetworkEndian>()?;

    // We don't want to have to deal with extra parsing, so only support
    // DNS packets with one question in it
    if questions != 1 {
        return Err(Error::dns_too_many_questions());
    }

    // Jump to the questions section
    cursor.seek(SeekFrom::Start(12))?;

    // Now read the string in DNS format
    let mut hostname_buff: Vec<u8> = Vec::new();
    loop {
        let mut size_buff = vec![0; 1];
        if let Ok(result) = cursor.read(&mut size_buff) {
            if result != 1 {
                let e = Error::dns_parsing();
                return Err(e);
            }
        }

        if size_buff[0] == 0x0 {
            // Remove last dot and then bail
            let new_size = hostname_buff.len() - 1;
            hostname_buff.truncate(new_size);
            break;
        }

        for _ in 0..size_buff[0] {
            let mut byte_buff = vec![0; 1];
            if let Ok(result) = cursor.read(&mut byte_buff) {
                if result != 1 {
                    let e = Error::dns_parsing();
                    return Err(e);
                }
            }
            hostname_buff.push(byte_buff[0]);
        }

        hostname_buff.push('.' as u8);
    }

    let as_string = String::from_utf8(hostname_buff)?;

    Ok(as_string)
}

pub fn tweak_to_nxdomain(packet: &mut Vec<u8>) -> Result<(), Error> {
    let mut cursor = Cursor::new(packet);
    let response_bytes = vec![0x81, 0x83];

    // We replace the flags to make it look like NXDomain response
    cursor.seek(SeekFrom::Start(2))?;

    cursor.write(&response_bytes)?;

    Ok(())
}
