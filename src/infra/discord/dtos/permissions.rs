use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesDto {
    #[serde(alias = "id")]
    pub role_id: String,
    #[serde(alias = "type")]
    pub _type: u8,
    pub allow: String,
    pub deny: String,
}
