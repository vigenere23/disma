use serde::Deserialize;

use crate::domain::{guild::GuildSummary, permission::PermissionsList, role::ExistingRole};

#[derive(Deserialize)]
pub struct GuildResponse {
    pub name: String,
    pub id: String,
}

impl Into<GuildSummary> for GuildResponse {
    fn into(self) -> GuildSummary {
        GuildSummary {
            name: self.name,
            id: self.id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub permissions: String,
    pub color: u32,
    pub hoist: bool,
    pub mentionable: bool,
}

impl Into<ExistingRole> for RoleResponse {
    fn into(self) -> ExistingRole {
        let color = match self.color {
            0 => None,
            color => Some(format!("{:X}", color)),
        };

        ExistingRole {
            id: self.id,
            name: self.name,
            permissions: PermissionsList::from(self.permissions.as_str()),
            color,
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
