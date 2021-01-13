use curl::easy::{Easy, List};
use json::{JsonValue, JsonError};
use std::str;
use std::fs;
use std::io::prelude::*;
use std::string::String;

pub fn get_info(token: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://www.googleapis.com/drive/v3/about?fields=user").unwrap();

    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    header.append("Accept: application/json").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();

    conn.http_headers(header).unwrap();

    {
        let mut transfer = conn.transfer();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    match json::parse(str::from_utf8(&response).unwrap())
        {
            Ok(v) => v,
            Err(_) => json::JsonValue::Null,
        }
}

pub fn list_files(token: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://www.googleapis.com/drive/v3/files").unwrap();

    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    header.append("Accept: application/json").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();

    conn.http_headers(header).unwrap();

    {
        let mut transfer = conn.transfer();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    match json::parse(str::from_utf8(&response).unwrap())
        {
            Ok(v) => v,
            Err(_) => json::JsonValue::Null,
        }
}

pub fn download(token: &str, id: &str,file_name_in_gdrive: &str, local_path: &str) -> bool{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    let file_name = format!("{}/{}", local_path, file_name_in_gdrive);
    let mut file = fs::File::create(file_name.replace("\\", "/")).unwrap();

    let url = "https://www.googleapis.com/drive/v3/files/".to_owned() + id + "?alt=media";

    conn.url(&url).unwrap();

    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    header.append("Accept: application/json").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();

    conn.http_headers(header).unwrap();

    {
        let mut transfer = conn.transfer();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    file.write_all(&response).expect("Unable to write data");
    true
}
