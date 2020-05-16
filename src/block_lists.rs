use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use curl::easy::Easy;

use crate::error::Error;

pub struct BlockLists {
    blocked_domains: HashSet<String>,
    http_lists: Vec<HttpBlockList>,
    file_lists: Vec<FileBlockList>,
}

pub struct HttpBlockList {
    url: String,
}

pub struct FileBlockList {
    path: String,
}

impl BlockLists {
    pub fn new() -> Self {
        let blocked_domains = HashSet::new();
        let http_lists = Vec::new();
        let file_lists = Vec::new();

        BlockLists {
            blocked_domains,
            http_lists,
            file_lists,
        }
    }

    pub fn add_http_list(&mut self, url: &str) {
        self.http_lists.push(HttpBlockList {
            url: String::from(url),
        })
    }

    pub fn add_file_list(&mut self, path: &str) {
        self.file_lists.push(FileBlockList {
            path: String::from(path),
        })
    }

    pub fn check(&self, to_check: &str) -> bool {
        self.blocked_domains.contains(&String::from(to_check))
    }

    pub fn sync(&mut self) -> Result<(), Error> {
        self.sync_http()?;
        self.sync_file()?;
        Ok(())
    }

    fn sync_http(&mut self) -> Result<(), Error> {
        for list_details in &self.http_lists {
            if let Err(err) = do_http(list_details.url.as_str(), &mut self.blocked_domains) {
                eprintln!("Couldn't sync {}. {}", list_details.url, err);
            }
        }
        Ok(())
    }

    fn sync_file(&mut self) -> Result<(), Error> {
        for list_details in &self.file_lists {
            if let Err(err) = do_file(list_details.path.as_str(), &mut self.blocked_domains) {
                eprintln!("Couldn't sync {}. {}", list_details.path, err);
            }
        }

        Ok(())
    }
}

fn do_http(url: &str, blocked_domains: &mut HashSet<String>) -> Result<(), Error> {
    let mut handle = Easy::new();
    handle.url(url)?;
    {
        let mut buffer: Vec<u8> = Vec::new();
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            for byte in data {
                if *byte as char == '\n' {
                    if let Ok(processed_line) = process_line_bytes(buffer.as_slice()) {
                        blocked_domains.insert(processed_line);
                    }
                    buffer.truncate(0);
                } else {
                    buffer.push(*byte);
                }
            }
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    Ok(())
}

fn do_file(path: &str, blocked_domains: &mut HashSet<String>) -> Result<(), Error> {
    let file = File::open(path)?;
    let buf_reader = io::BufReader::new(file);

    for line_res in buf_reader.lines() {
        if let Ok(line) = line_res {
            if let Ok(processed_line) = process_line(line) {
                blocked_domains.insert(processed_line);
            }
        }
    }

    Ok(())
}

fn process_line_bytes(input: &[u8]) -> Result<String, Error> {
    let output = String::from_utf8(input.to_vec())?;

    process_line(output)
}

fn process_line(input: String) -> Result<String, Error> {
    let mut output = input;
    if let Some(index) = output.find('#') {
        output.truncate(index);
    }
    if output.is_empty() {
        return Err(Error::line_is_blank());
    }

    let parts = output.split_whitespace();
    if let Some(hostname) = parts.last() {
        let trimmed_hostname = hostname.trim();
        Ok(String::from(trimmed_hostname))
    } else {
        Err(Error::line_is_blank())
    }
}
