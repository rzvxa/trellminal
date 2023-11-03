use toml;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Database {
    pub first_load: bool,
    pub users: HashMap<String, String>,
}

fn create_default_db() -> Database {
    return Database {
        first_load: true,
        users: HashMap::new(),
    }
}

pub fn load_database(path: &str) -> Database {
    let path = Path::new(path);
    if path.exists() {
        let raw = fs::read_to_string(path).unwrap();
        return toml::from_str(&raw).unwrap();
    } else {
        return create_default_db();
    }
}
