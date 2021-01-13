use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str;
use zstd;
use serde::{Deserialize, Serialize};

use crate::crypto;

#[derive(Deserialize, Serialize, Debug)]
pub struct EncryptedFile {
    path: String,
    key: crypto::Key,
}
impl EncryptedFile {

    pub fn new(path: &str, key: crypto::Key) -> EncryptedFile {
        EncryptedFile {
            path: String::from(path),
            key,
        }
    }

    pub fn create(&self) {
        if Path::new(&self.path).exists() == false {
            let splitted: Vec<&str> = self.path.split("/").collect();
            if splitted.len() > 1 {
                let folders = &self.path.replace(&splitted[&splitted.len() - 1], "");
                fs::create_dir_all(folders).unwrap();
                File::create(&self.path).expect("Unable to create file");
            } else {
                File::create(&self.path).expect("Unable to create file");
            }
        }
    }

    pub fn read(&self) -> Result<Vec<u8>, String> {
        let mut compressed_data: Vec<u8> = vec![];
        let file;
        match File::open(&self.path) {
            Ok(x) => {file = x},
            Err(e) => return Err(format!("Could not open file at {} ({})", self.path, e)),
        }
        let mut buf_reader = BufReader::new(&file);
        match buf_reader.read_to_end(&mut compressed_data) {
            Ok(_) => {},
            Err(e) => return Err(format!("Could not read file at {} ({})", self.path, e)),
        }
        match uncompress_decrypt(&compressed_data,&self.key) {
            Ok(x) => Ok(x),
            Err(e) => Err(format!("{} at: {}", e, self.path)),
        }
    }

    pub fn write(&self, data: &[u8]) {
        let compressed_data: Vec<u8> = compress_encrypt(data, &self.key);
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .unwrap()
            .write_all(&compressed_data)
            .unwrap();
    }
}

pub fn compress_encrypt(data: &[u8], key: &crypto::Key) -> Vec<u8> {
    let mut raw_compressed_data: Vec<u8> = vec![];
    zstd::stream::copy_encode(&mut &*data, &mut raw_compressed_data, 21).unwrap();
    let encrypted_data = crypto::encrypt(&raw_compressed_data, key);
    let mut compressed_data: Vec<u8> = vec![];
    zstd::stream::copy_encode(&mut &*encrypted_data, &mut compressed_data, 21).unwrap();
    compressed_data
}

pub fn uncompress_decrypt(encrypted_compressed_data: &[u8], key: &crypto::Key) -> Result<Vec<u8>, String> {
    let mut encrypted_data = vec![];
    match zstd::stream::copy_decode(&mut &*encrypted_compressed_data, &mut encrypted_data) {
        Ok(()) => (),
        Err(e) => return Err(format!("{} ({})", "Could not uncompress file before decryption", e.to_string())),
    }
    match crypto::decrypt(&encrypted_data, key) {
        Ok(x) => {
            let mut raw_data = vec![];
            match zstd::stream::copy_decode(&mut &*x, &mut raw_data) {
                Ok(()) => Ok(raw_data),
                Err(e) => Err(format!("{} ({})", "Could not uncompress file after decryption", e.to_string())),
            }
        },
        Err(e) => Err(format!("{} ({})", "Could not decrypt file", e.to_string())),
    }
}
