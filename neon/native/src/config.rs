use crate::encrypted_file::EncryptedFile;
use serde::{Deserialize, Serialize};
use std::fs;
use std::str;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub dropbox: Dropbox,
}

#[derive(Deserialize, Serialize)]
pub struct Dropbox {
    pub token: String,
}

pub fn new(conf_file: &EncryptedFile, username: &str) -> String {
    let blank_conf = format!("[general]\nusername = \"{}\"\n\n[dropbox]\ntoken = \"\"\n\n[gdrive]\ntoken = \"\"", &username);
    conf_file.write(blank_conf.as_bytes());
    String::from(blank_conf)
}

pub fn read(conf_file: &EncryptedFile) -> Result<String, String> {
    match conf_file.read() {
        Ok (x) => Ok(String::from(str::from_utf8(&x).unwrap())),
        Err(e) => Err(e),
    }

}

pub fn update(conf_file: &EncryptedFile, updated: &str) {
    conf_file.write(updated.as_bytes());
}

pub fn exists(path: &str) -> String {
    if Path::new(&String::from(path)).exists() == true {
        String::from("true")
    } else {
        String::from("false")
    }
}

pub fn delete(path: &str) {
    let splitted: Vec<&str> = path.split("/").collect();
    if splitted.len() > 1 {
        let folders = path.replace(&splitted[&splitted.len() - 1], "");
        fs::remove_dir_all(folders).unwrap();
    }
    fs::remove_file(path).unwrap();
}

pub fn list(path: &str) -> String {
    let files_paths;
    match fs::read_dir(path) {
        Ok(x)  => {files_paths = x;},
        Err(_) => {return "".to_string();},
    }
    let mut profiles = String::from("");
    let mut config: String;
    for files in files_paths {
        let splitted: Vec<&str> = path.split("/").collect();
        if splitted.len() > 1 {
            let folders = path.replace(&splitted[&splitted.len() - 1], "");
            config = (files.unwrap().path().display().to_string()).replace(&folders, "");
        } else {
            config = files.unwrap().path().display().to_string();
        }
        if profiles != "" {
            profiles = format!("{};{}", profiles, &config);
        } else {
            profiles = config;
        }
    }
    profiles
}
