use serde::{Deserialize, Serialize};

pub type ListId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: ListId,
    #[serde(rename = "idBoard")]
    pub id_board: String,
    #[serde(rename = "idOrganization")]
    pub id_organization: String,
    pub name: String,
}

