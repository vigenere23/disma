use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
}

#[derive(Debug, Deserialize)]
pub struct PermissionOverwritesDto {
    pub id: String,
    #[serde(alias = "type")]
    pub _type: u8,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Deserialize)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    #[serde(alias = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
}
