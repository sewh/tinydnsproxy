use std::fs;

use serde::{Deserialize, Serialize};
use toml;

use crate::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralOptions,
    pub http_block_list: Option<Vec<HttpBlockList>>,
    pub file_block_list: Option<Vec<FileBlockList>>,
    pub dot_provider: Vec<DotProvider>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralOptions {
    pub bind_ip: String,
    pub bind_port: u16,
    pub refresh_blocklists_after: u64,
    pub worker_threads: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpBlockList {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileBlockList {
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DotProvider {
    pub ip: String,
    pub port: u16,
    pub hostname: String,
}

impl Config {
    pub fn from_toml_file(path: &str) -> Result<Self, Error> {
        let parsed_toml: Config = toml::from_str(fs::read_to_string(path)?.as_str())?;
        if parsed_toml.dot_provider.is_empty() {
            return Err(Error::no_dot_providers());
        }

        Ok(parsed_toml)
    }
}
