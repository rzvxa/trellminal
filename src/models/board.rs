use serde::{Deserialize, Serialize};

pub type BoardId = String;

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: BoardId,
    pub name: String,
    pub desc: String,
    pub url: String,
    pub pinned: bool,
    pub starred: bool,
}
