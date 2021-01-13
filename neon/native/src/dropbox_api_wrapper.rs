use curl::easy::{Easy, List};
use json::{JsonValue, JsonError};
use std::str;
use std::fs;
use std::io::prelude::*;
use std::string::String;

pub fn get_token(code: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let grant_type = format!("grant_type=authorization_code&code={}&redirect_uri=http://localhost:65000/callback", code);
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropbox.com/1/oauth2/token").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    header.append("Content-Type: application/x-www-form-urlencoded").unwrap();

    conn.http_headers(header).unwrap();

    conn.username("2ezekn40iurcs2i").unwrap();
    conn.password("xnb3suyo584qkrv").unwrap();

	conn.post_field_size(grant_type.len() as u64).unwrap();
    conn.post_fields_copy(grant_type.as_bytes()).unwrap();

	//conn.verbose(true).unwrap();

    {
        let mut x = grant_type.as_bytes();
        let mut transfer = conn.transfer();
        transfer.read_function(|into| {
            Ok(x.read(into).unwrap())
        }).unwrap();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    drop(conn);
    match json::parse(str::from_utf8(&response).unwrap())
    {
        Ok(v) => v,
        Err(_) => json::JsonValue::Null,
    }
}

pub fn get_info(token: &str) -> Result<JsonValue, JsonError>{

    let mut conn = Easy::new();
    let mut header = List::new();
    let data = "null";
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/users/get_current_account").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();

    conn.http_headers(header).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    {
        let mut transfer = conn.transfer();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    json::parse(str::from_utf8(&response).unwrap())
}

pub fn get_space(token: &str) -> Result<JsonValue, JsonError>{

    let mut conn = Easy::new();
    let mut header = List::new();
    let data = "null";
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/users/get_space_usage").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

	// conn.verbose(true).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();

    conn.http_headers(header).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    {
        let mut transfer = conn.transfer();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    json::parse(str::from_utf8(&response).unwrap())
}

pub fn list_files(token: &str, mut path: &str, recursive: &str) -> Result<JsonValue, JsonError>{

    let mut conn = Easy::new();
    let mut header = List::new();

    if path == "\\"
    {
        path = "";
    }
    let data = format!("{{\"path\": \"{}\",\"recursive\": {},\"include_media_info\": false,\"include_deleted\": false,\"include_has_explicit_shared_members\": false,\"include_mounted_folders\": true,\"include_non_downloadable_files\": true}}", path, recursive);
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/files/list_folder").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

	// conn.verbose(true).unwrap();


    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
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
    json::parse(str::from_utf8(&response).unwrap())
}

pub fn list_files_continue(token: &str, cursor: &str) -> Result<JsonValue, JsonError>{

    let mut conn = Easy::new();
    let mut header = List::new();

    let data = format!("{{\"cursor\": \"{}\"}}", cursor);

    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/files/list_folder/continue").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

	// conn.verbose(true).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
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
    json::parse(str::from_utf8(&response).unwrap())
}


pub fn create_dir(token: &str, mut path: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();

    if path == "\\"
    {
        path = "";
    }
    let data = format!("{{\"path\": \"{}\"}}", path);
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/files/create_folder").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
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

pub fn delete(token: &str, mut path: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();

    if path == "\\"
    {
        path = "";
    }
    let data = format!("{{\"path\": \"{}\"}}", path);
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/files/delete").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
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

pub fn r#move(token: &str, mut from_path: &str, mut to_path: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();

    if from_path == "\\"
    {
        from_path = "";
    }
    if to_path == "\\"
    {
        to_path = "";
    }
    let data = format!("{{\"from_path\": \"{}\",\"to_path\": \"{}\",\"allow_shared_folder\": false,\"autorename\": false,\"allow_ownership_transfer\": false}}", from_path, to_path);
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://api.dropboxapi.com/2/files/move").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

    header.append("Content-Type: application/json; charset=utf-8").unwrap();
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

pub fn download(token: &str, mut remote_path: &str, local_path: &str) -> bool{

    let mut conn = Easy::new();
    let mut header = List::new();
    let data = "";
    let mut response: Vec<u8> = Vec::new();
    let file_name = format!("{}/{}", local_path, &remote_path[remote_path.rfind("/").unwrap()+1..remote_path.len()]);
    let mut file = fs::File::create(file_name.replace("\\", "/")).unwrap();

    if remote_path == "\\"
    {
        remote_path = "";
    }

    conn.url("https://content.dropboxapi.com/2/files/download").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

	// conn.verbose(true).unwrap();

    header.append("Content-Type: application/octet-stream; charset=utf-8").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();
    header.append(&format!("Dropbox-API-Arg: {{\"path\": \"{}\"}}", remote_path)).unwrap();

    conn.http_headers(header).unwrap();

    conn.post_field_size(data.len() as u64).unwrap();
    conn.post_fields_copy(data.as_bytes()).unwrap();

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

pub fn upload(token: &str, mut remote_path: &str, data: &[u8]) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    if remote_path == "\\"
    {
        remote_path = "";
    }

    conn.url("https://content.dropboxapi.com/2/files/upload").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    //conn.verbose(true).unwrap();

	header.append("Transfer-Encoding: chunked").unwrap();
    header.append("Content-Type: application/octet-stream").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();
    header.append(&format!("Dropbox-API-Arg: {{\"path\": \"{}\",\"mode\": \"add\",\"autorename\": true,\"mute\": false,\"strict_conflict\": false}}", remote_path)).unwrap();

    conn.http_headers(header).unwrap();

    {
        let mut x = &*data;
        let mut transfer = conn.transfer();
        transfer.read_function(|into| {
            Ok(x.read(into).unwrap())
        }).unwrap();
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

pub fn upload_session_start(token: &str, data: &[u8]) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://content.dropboxapi.com/2/files/upload_session/start").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    // conn.verbose(true).unwrap();

	header.append("Transfer-Encoding: chunked").unwrap();
    header.append("Content-Type: application/octet-stream").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();
    header.append(&format!("Dropbox-API-Arg: {{\"close\": false}}")).unwrap();

    conn.http_headers(header).unwrap();

    {
        let mut x = &*data;
        let mut transfer = conn.transfer();
        transfer.read_function(|into| {
            Ok(x.read(into).unwrap())
        }).unwrap();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let size = format!("\"data_size\": \"{}\"}}",data.len().to_string());
    let mut to_parse: String = String::from(std::str::from_utf8(&response).unwrap());
    //to_parse.remove(0);
    to_parse = to_parse.replace("}", ", ");
    to_parse = format!("{}{}", to_parse, size);

    match json::parse(&to_parse)
        {
            Ok(v) => v,
            Err(_) => json::JsonValue::Null,
        }
}

pub fn upload_session_finish(token: &str, remote_path: &str, session_id: &str, data_size: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://content.dropboxapi.com/2/files/upload_session/finish").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    // conn.verbose(true).unwrap();

	header.append("Transfer-Encoding: chunked").unwrap();
    header.append("Content-Type: application/octet-stream").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();
    header.append(&format!("Dropbox-API-Arg: {{\"cursor\": {{\"session_id\": \"{}\",\"offset\": {}}},\"commit\": {{\"path\": \"{}\",\"mode\": \"add\",\"autorename\": true,\"mute\": false,\"strict_conflict\": false}}}}", session_id,data_size, remote_path)).unwrap();

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

pub fn upload_session_append(token: &str, data: &[u8], session_id: &str, data_size: &str) -> JsonValue{

    let mut conn = Easy::new();
    let mut header = List::new();
    let mut response: Vec<u8> = Vec::new();

    conn.url("https://content.dropboxapi.com/2/files/upload_session/append_v2").unwrap();

    conn.post(true).unwrap();
    conn.ssl_verify_peer(true).unwrap();
    conn.follow_location(true).unwrap();

    // conn.verbose(true).unwrap();

	header.append("Transfer-Encoding: chunked").unwrap();
    header.append("Content-Type: application/octet-stream").unwrap();
    header.append(&format!("Authorization: Bearer {}", token)).unwrap();
    header.append(&format!("Dropbox-API-Arg: {{\"cursor\": {{\"session_id\": \"{}\",\"offset\": {}}},\"close\": false}}", session_id, data_size)).unwrap();

    conn.http_headers(header).unwrap();

    {
        let mut x = &*data;
        let mut transfer = conn.transfer();
        transfer.read_function(|into| {
            Ok(x.read(into).unwrap())
        }).unwrap();
        transfer.write_function(|new_data| {
            response.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let size = format!("\"data_size\": \"{}\"}}",data.len().to_string());
    let mut to_parse: String = String::from(std::str::from_utf8(&response).unwrap());
    //to_parse.remove(0);
    to_parse = to_parse.replace("}", ", ");
    to_parse = format!("{}{}", to_parse, size);

    match json::parse(&to_parse)
        {
            Ok(v) => v,
            Err(_) => json::JsonValue::Null,
        }
}
