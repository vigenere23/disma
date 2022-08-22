use serde::{Deserialize, Serialize};

use super::permissions::PermissionOverwritesDto;

#[derive(Debug, Serialize)]
pub struct ChannelRequest {
    pub id: String,
    pub name: String,
    pub topic: String,
    #[serde(alias = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
}

#[derive(Debug, Deserialize)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    #[serde(alias = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
}
