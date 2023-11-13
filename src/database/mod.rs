mod account;

use crate::models::{User, UserId};
use account::Account;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Error as IoError, path::Path};
use toml;

#[derive(Debug)]
pub enum DatabaseError {
    IoError(IoError),
    SerializationError(toml::ser::Error),
    DeserializationError(toml::de::Error),
    KeyNotFound,
}

impl From<IoError> for DatabaseError {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

impl From<toml::ser::Error> for DatabaseError {
    fn from(err: toml::ser::Error) -> Self {
        Self::SerializationError(err)
    }
}

impl From<toml::de::Error> for DatabaseError {
    fn from(err: toml::de::Error) -> Self {
        Self::DeserializationError(err)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub first_load: bool,
    pub active_account: Option<UserId>,
    pub accounts: HashMap<UserId, Account>,
    path: String,
}

impl Database {
    pub fn new(path: String) -> Self {
        Database {
            first_load: true,
            active_account: None,
            accounts: HashMap::new(),
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

    pub fn save(&self) -> Result<(), DatabaseError> {
        let content = toml::to_string(self)?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn add_user_account(&mut self, user: User, token: String) -> Result<(), DatabaseError> {
        let user_id = user.id.clone();
        let account = Account {
            id: user.id,
            username: user.username,
            token,
        };
        self.accounts.insert(user_id, account);
        Ok(())
    }

    pub fn set_active_account(&mut self, id: UserId) -> Result<(), DatabaseError> {
        if self.accounts.contains_key(&id) {
            self.active_account = Some(id);
            Ok(())
        } else {
            Err(DatabaseError::KeyNotFound)
        }
    }
}
