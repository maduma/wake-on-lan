
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn get_db_path() -> PathBuf {
    let home_dir = env::var("HOMEPATH").unwrap();
    Path::new(&home_dir).join(".wake_on_lan.json")
}

fn open_db() -> HashMap<String, String> {
    let db_file = get_db_path();
    match fs::read_to_string(db_file) {
        Ok(str) => serde_json::from_str::<HashMap<String, String>>(&str).unwrap(),
        Err(_) => HashMap::new(),
    }
}

fn close_db(db: &HashMap<String, String>) {
    if db.is_empty() {
        return;
    }
    let json = serde_json::to_string(db).unwrap();
    let db_file = get_db_path();
    fs::write(db_file, json).unwrap();
}

pub fn create_alias(alias: &str, mac: &str) {
    let db = &mut open_db();
    db.insert(alias.to_string(), mac.to_string());
    close_db(db);
}

pub fn remove_alias(_alias: &str) {
    let db = &mut open_db();
    match db.remove(_alias) {
        Some(_) => println!("Removing alias {_alias}"),
        None => println!("Cannot find alias {_alias}"),
    };
    close_db(db)
}

pub fn get_alias(_alias: &str) -> Option<String> {
    let db = &open_db();
    db.get(_alias).map(|s| s.to_string())
}
