use serde::{Deserialize, Serialize};

pub type LabelId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: LabelId,
    #[serde(rename = "idBoard")]
    pub id_board: String,
    pub name: String,
    pub color: String,
    pub uses: u32,
}
