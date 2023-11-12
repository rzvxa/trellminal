use super::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub id: UserId,
    pub username: String,
    pub token: String,
}
