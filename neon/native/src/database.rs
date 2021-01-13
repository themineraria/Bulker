use crate::encrypted_file::EncryptedFile;
use std::fs;
use std::str;

pub fn new(conf_file: &EncryptedFile) -> String {
    let blank_conf = json::stringify(json::JsonValue::new_array());
    conf_file.write(blank_conf.as_bytes());
    String::from(blank_conf)
}

pub fn read(conf_file: &EncryptedFile) -> Result<String, String> {
    match conf_file.read() {
        Ok (x) => Ok(String::from(str::from_utf8(&x).unwrap())),
        Err(e) => Err(e),
    }
}

pub fn append(conf_file: &EncryptedFile, data: &str) {
    let mut db = json::parse(&read(conf_file).unwrap()).unwrap();
    let db_size = db.len();
    db[db_size] = json::parse(data).unwrap();
    conf_file.write(json::stringify(db).as_bytes());
}

pub fn remove(conf_file: &EncryptedFile, index: &str) {
    let mut db = json::parse(&read(conf_file).unwrap()).unwrap();
    let db_size = db.len();
    let mut modified_flag = false;
    for x in 0..db_size-1 {
        if db[x]["value"] == index {
            db.array_remove(x);
            modified_flag = true;
        }
    }
    if modified_flag == true
    {
        conf_file.write(json::stringify(db).as_bytes());
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
