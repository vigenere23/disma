use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PermissionOverwriteType {
    Role = 0,
    // Member = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwritesDto {
    #[serde(rename = "id")]
    pub role_or_member_id: String,
    #[serde(rename = "type")]
    pub _type: PermissionOverwriteType,
    pub allow: String,
    pub deny: String,
}
