use super::Label;
use serde::{Deserialize, Serialize};

pub type CardId = String;

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    #[serde(rename = "idBoard")]
    pub id_board: String,
    #[serde(rename = "idList")]
    pub id_list: String,
    #[serde(rename = "idOrganization")]
    pub id_organization: String,
    pub name: String,
    pub labels: Vec<Label>,
    pub url: String,
}
