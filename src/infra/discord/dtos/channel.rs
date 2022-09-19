use serde::{Deserialize, Serialize};

use crate::domain::{
    category::AwaitingCategory,
    role::{ExistingRole, RolesList},
};

use super::permissions::PermissionOverwritesDto;

#[derive(Debug, Serialize)]
pub struct ChannelRequest {
    pub name: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
}

impl ChannelRequest {
    pub fn from(category: &AwaitingCategory, roles: &RolesList<ExistingRole>) -> Self {
        let permission_overwrites = category.permissions_overwrites.as_ref().map(|permissions| {
            permissions
                .iter()
                .map(|permission| PermissionOverwritesDto {
                    role_id: roles
                        .find_by_name(&permission.role.name)
                        .unwrap()
                        .id
                        .clone(),
                    allow: permission.allow.code(),
                    deny: permission.deny.code(),
                    _type: 0, // TODO make an enum
                })
                .collect()
        });

        Self {
            name: category.name.clone(),
            topic: String::new(),
            _type: 4, // TODO make an enum
            parent_id: None,
            permission_overwrites,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    #[serde(rename = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Option<Vec<PermissionOverwritesDto>>,
}
