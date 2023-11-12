use serde::{Deserialize, Serialize};

pub type UserId = String;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    #[serde(rename = "idBoards")]
    pub id_boards: Vec<String>,
    #[serde(rename = "idOrganizations")]
    pub id_organizations: Vec<String>,
}
