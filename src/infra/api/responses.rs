use serde::Deserialize;

use crate::domain::role::ExistingRole;

#[derive(Debug, Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
}

impl Into<ExistingRole> for RoleResponse {
    fn into(self) -> ExistingRole {
        ExistingRole {
            id: self.id,
            name: self.name,
            is_mentionalbe: self.mentionable,
            show_in_sidebar: self.hoist,
        }
    }
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
