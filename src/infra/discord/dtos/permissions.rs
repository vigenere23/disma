use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesDto {
    #[serde(alias = "id")]
    pub role_id: String, // can be user_id too
    #[serde(alias = "type")]
    pub _type: u8, // 0 = role, 1 = member
    pub allow: String,
    pub deny: String,
}
