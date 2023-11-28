use serde::{Deserialize, Serialize};

pub type LabelId = String;

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub id: LabelId,
    pub id_board: String,
    pub name: String,
    pub color: String,
    pub uses: u32,
}
