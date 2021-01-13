use neon::prelude::*;
use neon::register_module;

mod gdrive;
mod gdrive_api_wrapper;
mod dropbox_api_wrapper;
mod dropbox;
mod config;
mod database;
mod encrypted_file;
mod crypto;
mod neon_async;

fn profile_exists(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(format!("{}", config::exists(&path))))
}

fn generate_encrypted_handler(mut cx: FunctionContext) -> JsResult<JsNull> {
    let username = cx.argument::<JsString>(0)?.value();
    let password = cx.argument::<JsString>(1)?.value();
    let path = cx.argument::<JsString>(2)?.value();
    let callback = cx.argument::<JsFunction>(3)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            let key = crypto::derive_key(&username, &password);
            match serde_json::to_string(&encrypted_file::EncryptedFile::new(&path, key)) {
                Ok(x) => Ok(x),
                Err(_) => Err("[E]Unable to serialize the encrypted file handler!".to_string()),
            }
        }
    )}.schedule(callback);
    Ok(cx.null())
}

fn create_profile(mut cx: FunctionContext) -> JsResult<JsString> {
    let encrypted: encrypted_file::EncryptedFile = serde_json::from_str(&cx.argument::<JsString>(0)?.value()).unwrap();
    let username = cx.argument::<JsString>(1)?.value();
    encrypted.create();
    Ok(cx.string(config::new(&encrypted, &username)))
}

fn read_profile(mut cx: FunctionContext) -> JsResult<JsNull> {
    let encrypted: encrypted_file::EncryptedFile = serde_json::from_str(&cx.argument::<JsString>(0)?.value()).unwrap();
    let callback = cx.argument::<JsFunction>(1)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            config::read(&encrypted)
        }
    )}.schedule(callback);
    Ok(cx.null())
}

fn delete_profile(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value();
    config::delete(&path);
    Ok(cx.string(""))
}

fn list_profiles(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(config::list(&path)))
}

fn update_profile(mut cx: FunctionContext) -> JsResult<JsNull> {
    let encrypted: encrypted_file::EncryptedFile = serde_json::from_str(&cx.argument::<JsString>(0)?.value()).unwrap();
    let arg1 = cx.argument::<JsString>(1)?.value();
    config::update(&encrypted, &arg1);
    Ok(cx.null())
}

fn create_database(mut cx: FunctionContext) -> JsResult<JsString> {
    let encrypted: encrypted_file::EncryptedFile = serde_json::from_str(&cx.argument::<JsString>(0)?.value()).unwrap();
    encrypted.create();
    Ok(cx.string(database::new(&encrypted)))
}

fn delete_database(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?.value();
    database::delete(&path);
    Ok(cx.string(""))
}

fn read_database(mut cx: FunctionContext) -> JsResult<JsNull> {
    let encrypted: encrypted_file::EncryptedFile = serde_json::from_str(&cx.argument::<JsString>(0)?.value()).unwrap();
    let callback = cx.argument::<JsFunction>(1)?;

    neon_async::Async{lambda: Box::new(move ||
        {
            database::read(&encrypted)
        }
    )}.schedule(callback);
    Ok(cx.null())
}

register_module!(mut m, {
    /*Encrypted file*/
    m.export_function("generateEncryptedHandler", generate_encrypted_handler)?;

    /*User profile*/
    m.export_function("profileExists", profile_exists)?;
    m.export_function("createProfile", create_profile)?;
    m.export_function("readProfile", read_profile)?;
    m.export_function("deleteProfile", delete_profile)?;
    m.export_function("listProfiles", list_profiles)?;
    m.export_function("updateProfile", update_profile)?;

    /*Database*/
    m.export_function("createDatabase", create_database)?;
    m.export_function("deleteDatabase", delete_database)?;
    m.export_function("readDatabase", read_database)?;

    /*Dropbox API*/
    m.export_function("waitUntilDropboxRedirect", dropbox::wait_until_dropbox_redirect)?;
    m.export_function("getDropboxToken", dropbox::get_token)?;
    m.export_function("getDropboxInfo", dropbox::get_info)?;
    m.export_function("getDropboxSpace", dropbox::get_space)?;
    m.export_function("getDropboxFilesList", dropbox::list_files)?;
    m.export_function("DropboxDownload", dropbox::download)?;
    m.export_function("DropboxUpload", dropbox::upload)?;
    m.export_function("DropboxDelete", dropbox::delete)?;

    /*Gdrive API*/
    m.export_function("gdrivePkceHandshake", gdrive::pkce_handshake)?;
    m.export_function("gdriveDownload", gdrive::download)?;
    m.export_function("getGdriveInfo", gdrive::get_info)?;
    m.export_function("getGdriveFilesList", gdrive::list_files)?;

    Ok(())
});

//m.export_function("throwError", throw_error)?;
/*fn throw_error(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsString>(0)?.value();
    if !arg0.contains("hello") {
        panic!("Expected you to say hello");
    }

    Ok(cx.string("hello to you too!"))
}*/
