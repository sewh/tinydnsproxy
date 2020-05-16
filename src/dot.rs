use std::io;
use std::vec;

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use rand::seq::SliceRandom;

use crate::error::Error;

pub fn read<R: io::Read>(reader: &mut R) -> Result<Vec<u8>, Error> {
    let to_read = reader.read_u16::<NetworkEndian>()?;
    let mut buff = vec![0; to_read as usize];
    reader.read_exact(&mut buff)?;

    Ok(buff)
}

pub fn write<W: io::Write>(writer: &mut W, buff: &[u8]) -> Result<(), Error> {
    let buff_size: usize = buff.len();
    writer.write_u16::<NetworkEndian>(buff_size as u16)?;
    writer.write_all(buff)?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct DotProvider {
    pub ip: String,
    pub port: u16,
    pub hostname: String,
    pub cert: Option<Vec<u8>>,
}

pub struct DotProviders {
    providers: Vec<DotProvider>,
}

impl DotProviders {
    pub fn new() -> Self {
        let providers = Vec::new();
        DotProviders { providers }
    }

    pub fn add(&mut self, ip: String, port: u16, hostname: String, cert: Option<Vec<u8>>) {
        let p = DotProvider {
            ip,
            port,
            hostname,
            cert,
        };
        self.providers.push(p);
    }

    pub fn clear(&mut self) {
        self.providers = Vec::new();
    }

    pub fn get_random(&self) -> Result<DotProvider, Error> {
        match self.providers.choose(&mut rand::thread_rng()) {
            Some(p) => Ok(p.clone()),
            None => Err(Error::no_dot_providers()),
        }
    }
}
