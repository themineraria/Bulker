use neon::prelude::*;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::fs;
use json::JsonValue;

use crate::dropbox_api_wrapper;
use crate::encrypted_file;
use crate::database;
use crate::crypto;
use crate::neon_async;

pub fn wait_until_dropbox_redirect(mut cx: FunctionContext) -> JsResult<JsNull> {
    let callback = cx.argument::<JsFunction>(0)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            let mut reply = Err(String::new());
            let listener = TcpListener::bind("127.0.0.1:65000").unwrap();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 1024];
                let reply_content = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\ncontent-length: 263\r\n\r\n<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>Bulker</title></head><body style=\"background-color: #282c34;color: white;text-align:center;\"><br><h1>Successfully connected !</h1><br><p>You can now close this page and go back to Bulker ..</p></body></html>";

                stream.read(&mut buffer).unwrap();

                let reply_status = stream.write(reply_content);
                stream.flush().unwrap();


                match reply_status {
                    Ok(_) => {
                        let line = String::from_utf8_lossy(&buffer[..]);
                        let start_bytes = line.find("Request: GET /callback?").unwrap_or(0); //index where "pattern" starts
                        let end_bytes = line.find("HTTP/").unwrap_or(line.len());
                        reply = Ok(String::from(&line[start_bytes + 14..end_bytes]));
                        break;
                    }
                    Err(e) => {
                        reply = Err(e.to_string());
                        break;
                    }
                }
            }
            reply
        }
    )}.schedule(callback);
    Ok(cx.null())
}

pub fn get_token(mut cx: FunctionContext) -> JsResult<JsString> {
    let oauth_token = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(format!("{}", dropbox_api_wrapper::get_token(&oauth_token))))
}

pub fn get_info(mut cx: FunctionContext) -> JsResult<JsNull> {
    let token = cx.argument::<JsString>(0)?.value();
    let callback = cx.argument::<JsFunction>(1)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            match dropbox_api_wrapper::get_info(&token) {
                Ok(x) => Ok(x.to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
    )}.schedule(callback);

    Ok(cx.null())
}

pub fn get_space(mut cx: FunctionContext) -> JsResult<JsNull> {
    let token = cx.argument::<JsString>(0)?.value();
    let callback = cx.argument::<JsFunction>(1)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            match dropbox_api_wrapper::get_space(&token) {
                Ok(x) => Ok(x.to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
    )}.schedule(callback);
    Ok(cx.null())
}

pub fn list_files(mut cx: FunctionContext) -> JsResult<JsNull> {
    let token = cx.argument::<JsString>(0)?.value();
    let path = cx.argument::<JsString>(1)?.value();
    let recursivity = cx.argument::<JsString>(2)?.value();
    let callback = cx.argument::<JsFunction>(3)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            let mut first_tree;
            match dropbox_api_wrapper::list_files(&token, &path, &recursivity) {
                Ok(x) => first_tree = x,
                Err(e) => return Err(e.to_string()),
            }
            if first_tree["has_more"] == true
            {
                let mut next_tree: JsonValue = first_tree.clone();
                loop {
                    match dropbox_api_wrapper::list_files_continue(&token, &json::stringify(next_tree["cursor"].clone()).replace("\"", "")) {
                        Ok(x) => next_tree = x,
                        Err(e) => return Err(e.to_string()),
                    }
                    for x in 0..next_tree["entries"].len() {
                        first_tree["entries"].push(next_tree["entries"][x].clone()).unwrap();
                    }
                    if next_tree["has_more"] == false {break;}
                }
                Ok(format!("{}", first_tree["entries"]))
            }
            else {
                Ok(format!("{}", first_tree["entries"]))
            }
        }
    )}.schedule(callback);
    Ok(cx.null())
}

pub fn download(mut cx: FunctionContext) -> JsResult<JsString> {
    let token = cx.argument::<JsString>(0)?.value();
    let remote_path = cx.argument::<JsString>(1)?.value();
    let local_path = cx.argument::<JsString>(2)?.value();

    dropbox_api_wrapper::download(&token, &remote_path, &local_path);

    let encrypted_name = remote_path.split("/").collect::<Vec<_>>().last().copied().unwrap();
    let real_name = cx.argument::<JsString>(3)?.value();

    if real_name != ""{
        let key_username = cx.argument::<JsString>(4)?.value();
        let key_pass = cx.argument::<JsString>(5)?.value();
        let encrypted_database_handler = cx.argument::<JsString>(6)?.value();
        let database_handler: encrypted_file::EncryptedFile = serde_json::from_str(&encrypted_database_handler).unwrap();
        let database_content = json::parse(&database::read(&database_handler).unwrap()).unwrap();
        for i in 0..database_content.len() {
            if &database_content[i]["encrypted_name"].as_str().unwrap().to_lowercase() == encrypted_name {
                if database_content[i]["chunks"].is_empty() == false {
                    let chunks = json::parse(&database_content[i]["chunks"].as_str().unwrap()).unwrap();
                    let mut file = fs::OpenOptions::new().create_new(true).append(true).open(format!("{}/{}", local_path, real_name)).unwrap();
                    let mut encrypted_file = fs::OpenOptions::new().read(true).open(format!("{}/{}", &local_path, encrypted_name)).unwrap();
                    for j in 0..chunks.len() {
                        let mut buffer = vec![0; chunks[j].as_usize().unwrap()];
                        encrypted_file.read(&mut buffer).expect("Unable to read file");
                        file.write(&encrypted_file::uncompress_decrypt(&buffer, &crypto::derive_key(&key_username,&key_pass)).unwrap()).unwrap();
                    }
                    fs::remove_file(format!("{}/{}", &local_path, encrypted_name)).unwrap();
                }
                else {
                    let mut file_data = fs::read(format!("{}/{}", &local_path, encrypted_name)).expect("Unable to read file");
                    file_data = encrypted_file::uncompress_decrypt(&file_data, &crypto::derive_key(&key_username,&key_pass)).unwrap();
                    fs::write(format!("{}/{}", &local_path, &real_name), file_data).expect("Unable to write file");
                    fs::remove_file(format!("{}/{}", &local_path, encrypted_name)).unwrap();
                }
            }
        }
    }

    Ok(cx.string(format!("")))
}
pub fn upload(mut cx: FunctionContext) -> JsResult<JsString> {
    let token = cx.argument::<JsString>(0)?.value();
    let local_path = cx.argument::<JsString>(1)?.value();
    let remote_path = cx.argument::<JsString>(2)?.value();
    let encrypted_name = cx.argument::<JsString>(3)?.value();

    let max_upload_size = 100000000; //

    let file_name = local_path.split("/").collect::<Vec<_>>().last().copied().unwrap();
    let metadata = fs::metadata(local_path.clone()).expect("Unable to read file's metadate");

    let reply;

    //File doesn't need to be split
    if metadata.len() <= max_upload_size {
        let mut file_data = fs::read(local_path.clone()).expect("Unable to read file");
        if encrypted_name == ""{ //not encrypted
            reply = dropbox_api_wrapper::upload(&token, &format!("{}{}", &remote_path, &file_name), &file_data);
        } else { //encrypted
            let key_username = cx.argument::<JsString>(4)?.value();
            let key_pass = cx.argument::<JsString>(5)?.value();
            println!("OK !");
            let encrypted_database_handler = cx.argument::<JsString>(6)?.value();
            println!("OK2 !");
            let encrypted: encrypted_file::EncryptedFile = serde_json::from_str(&encrypted_database_handler).unwrap();
            println!("OK3 !");
            file_data = encrypted_file::compress_encrypt(&file_data, &crypto::derive_key(&key_username,&key_pass));
            reply = dropbox_api_wrapper::upload(&token, &format!("{}{}", &remote_path, &encrypted_name), &file_data);
            let json_to_append = format!("{{\"encrypted_name\":\"{}\",\"real_name\":\"{}\",\"remote_path\":\"{}\",\"password\":\"{}\"}}", &encrypted_name, &file_name, &remote_path, &key_pass);
            database::append(&encrypted, &json_to_append);
        }
    }
    //File exceed max size & need to be split
    else{
        let mut to_upload:usize = metadata.len() as usize;
        let mut file = fs::File::open(local_path.clone()).unwrap();
        let mut buffer = vec![0; max_upload_size as usize];
        let upload_session_id;
        let upload_session;

        if encrypted_name == ""{ //not encrypted
            let mut uploaded_data_size = 0;
            let mut nb_bytes_read = file.read(&mut buffer).expect("Unable to read file");
            upload_session = dropbox_api_wrapper::upload_session_start(&token, &buffer[..nb_bytes_read]);
            uploaded_data_size += nb_bytes_read;
            upload_session_id = upload_session["session_id"].as_str().unwrap();
            to_upload -= nb_bytes_read;
            loop {
                nb_bytes_read = file.read(&mut buffer).expect("Unable to read file");
                to_upload -= nb_bytes_read;
                if to_upload == 0{
                    dropbox_api_wrapper::upload_session_append(&token, &buffer[..nb_bytes_read], &upload_session_id.clone(), &uploaded_data_size.to_string());
                    uploaded_data_size += nb_bytes_read;
                    reply = dropbox_api_wrapper::upload_session_finish(&token, &format!("{}{}", &remote_path, &file_name), &upload_session_id.clone(), &uploaded_data_size.to_string());
                    if reply == json::JsonValue::Null
                    {
                        panic!("Error while uploading file(s) to dropbox!");
                    }
                    break;
                }
                else {
                    dropbox_api_wrapper::upload_session_append(&token, &buffer[..nb_bytes_read], &upload_session_id.clone(), &uploaded_data_size.to_string());
                    uploaded_data_size += nb_bytes_read;
                }
            }
        }
        else{
            let key_username = cx.argument::<JsString>(4)?.value();
            let key_pass = cx.argument::<JsString>(5)?.value();
            let encrypted_database_handler = cx.argument::<JsString>(6)?.value();
            let database_handler: encrypted_file::EncryptedFile = serde_json::from_str(&encrypted_database_handler).unwrap();
            let mut nb_bytes_read = file.read(&mut buffer).expect("Unable to read file");
            let mut encrypted_chunk = encrypted_file::compress_encrypt(&buffer[..nb_bytes_read], &crypto::derive_key(&key_username,&key_pass));
            let mut encrypted_chunks_size: Vec<i32> = Vec::new();
            encrypted_chunks_size.push(encrypted_chunk.len() as i32);
            upload_session = dropbox_api_wrapper::upload_session_start(&token, &encrypted_chunk);
            upload_session_id = upload_session["session_id"].as_str().unwrap();
            to_upload -= nb_bytes_read;
            loop {
                nb_bytes_read = file.read(&mut buffer).expect("Unable to read file");
                encrypted_chunk = encrypted_file::compress_encrypt(&buffer[..nb_bytes_read], &crypto::derive_key(&key_username,&key_pass));
                to_upload -= nb_bytes_read;
                if to_upload == 0{
                    dropbox_api_wrapper::upload_session_append(&token, &encrypted_chunk, &upload_session_id.clone(), &encrypted_chunks_size.iter().sum::<i32>().to_string());
                    encrypted_chunks_size.push(encrypted_chunk.len() as i32);
                    reply = dropbox_api_wrapper::upload_session_finish(&token, &format!("{}{}", &remote_path, &encrypted_name), &upload_session_id.clone(), &encrypted_chunks_size.iter().sum::<i32>().to_string());
                    if reply == json::JsonValue::Null
                    {
                        panic!("Error while uploading file(s) to dropbox!");
                    }
                    else
                    {
                        let json_to_append = format!("{{\"encrypted_name\":\"{}\",\"real_name\":\"{}\",\"remote_path\":\"{}\",\"password\":\"{}\",\"chunks\":\"{}\"}}", &encrypted_name, &file_name, &remote_path, &key_pass, &json::stringify(encrypted_chunks_size));
                        database::append(&database_handler, &json_to_append);
                    }
                    break;
                }
                else {
                    dropbox_api_wrapper::upload_session_append(&token, &encrypted_chunk, &upload_session_id.clone(), &encrypted_chunks_size.iter().sum::<i32>().to_string());
                    encrypted_chunks_size.push(encrypted_chunk.len() as i32);
                }
            }
        }
    }
    Ok(cx.string(format!("{}", &reply)))
}

pub fn delete(mut cx: FunctionContext) -> JsResult<JsString> {
    let token = cx.argument::<JsString>(0)?.value();
    let path = cx.argument::<JsString>(1)?.value();
    Ok(cx.string(format!("{}", dropbox_api_wrapper::delete(&token, &path))))
}
