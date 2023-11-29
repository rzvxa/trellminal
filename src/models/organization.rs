use serde::{Deserialize, Serialize};

pub type OrganizationId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub desc: String,
    pub url: String,
    #[serde(rename = "teamType")]
    pub team_type: String,
}
