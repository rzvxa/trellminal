use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, error::Error, path::Path};
use toml;

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub first_load: bool,
    pub users: HashMap<String, String>,
    path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        Database {
            first_load: true,
            users: HashMap::new(),
            path,
        }
    }
    pub fn load(path: &str) -> Self {
        let p = Path::new(path);
        if p.exists() {
            let raw = fs::read_to_string(p).unwrap();
            return toml::from_str(&raw).unwrap();
        } else {
            return Database::new(path.to_string());
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}
