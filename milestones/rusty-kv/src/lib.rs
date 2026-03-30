use anyhow::{Error, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database<V> {
    // We store the data in a HashMap for fast access
    pub map: HashMap<String, V>,

    // We skip serializing the path because we don't want the path
    // saved inside the JSON file itself!
    #[serde(skip)]
    path: PathBuf,
}

impl<V> Database<V>
where
    V: Serialize + DeserializeOwned,
{
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Database {
            map: HashMap::new(),
            path: path.into(),
        }
    }

    pub fn insert(&mut self, key: String, value: V) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.map.get(key)
    }

    pub fn save(&self) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self.map)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();

        // Check if file exists first
        if !path.exists() {
            return Ok(Database::new(path));
        }

        let contents = fs::read_to_string(&path)?;
        let map = serde_json::from_str(&contents)?;

        Ok(Self { map, path })
    }
}
