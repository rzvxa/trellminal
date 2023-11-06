use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "idBoards")]
    pub id_boards: Vec<String>,
    #[serde(rename = "idOrganizations")]
    pub id_organizations: Vec<String>,
}
