#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

slint::include_modules!();

#[derive(Serialize, Deserialize, Debug)]
struct Database {
    data: HashMap<String, String>,
    // We skip serializing the path because we don't want the path
    // saved inside the JSON file itself!
    #[serde(skip)]
    path: PathBuf,
}

impl Database {
    fn new<P: Into<PathBuf>>(path: P) -> Self {
        Database {
            data: HashMap::new(),
            path: path.into(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        if !path.exists() {
            return Ok(Database::new(path));
        }

        let contents = fs::read_to_string(&path)?;
        let data = serde_json::from_str(&contents)?;
        Ok(Database { data, path })
    }
}

fn main() -> Result<(), slint::PlatformError> {
    // 1. Initialize UI and Database
    let ui = AppWindow::new()?;

    let db = Arc::new(Mutex::new(
        Database::load("storage.json").unwrap_or(Database::new("storage.json")),
    ));

    let ui_handle = ui.as_weak();
    let fetch_db = Arc::clone(&db);
    ui.on_fetch_data(move |key| {
        let ui = ui_handle.unwrap();
        let db = fetch_db.lock().unwrap();

        match db.get(&key) {
            Some(value) => ui.set_results(value.clone().into()),
            None => ui.set_results("Key not found!".into()),
        }
    });
    let ui_handle = ui.as_weak(); // Second weak handle (NEW CLONE)
    let save_db = Arc::clone(&db);
    ui.on_save_data(move |key, value| {
        let ui = ui_handle.unwrap();
        let mut db = save_db.lock().unwrap();

        db.insert(key.clone().to_string(), value.clone().to_string());
        match db.save() {
            Ok(_) => ui.set_status("success".to_string().into()),
            Err(_) => ui.set_status("error".to_string().into()),
        }
    });
    ui.run()
}
