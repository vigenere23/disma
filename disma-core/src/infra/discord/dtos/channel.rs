use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::domain::entities::{
    category::AwaitingCategory,
    role::{ExistingRole, RolesList},
};

use super::permissions::{PermissionOverwriteType, PermissionOverwritesDto};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
    Text = 0,
    Voice = 2,
    Category = 4,
}

#[derive(Debug, Serialize)]
pub struct ChannelRequest {
    pub name: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub _type: ChannelType,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwritesDto>,
}

impl ChannelRequest {
    pub fn from(category: &AwaitingCategory, roles: &RolesList<ExistingRole>) -> Self {
        let permission_overwrites = category
            .overwrites
            .items()
            .iter()
            .map(|permission| PermissionOverwritesDto {
                role_or_member_id: roles
                    .find_by_name(&permission.role.name)
                    .unwrap()
                    .id
                    .clone(),
                allow: permission.allow.code(),
                deny: permission.deny.code(),
                _type: PermissionOverwriteType::Role,
            })
            .collect();

        Self {
            name: category.name.clone(),
            topic: String::new(),
            _type: ChannelType::Category,
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
    pub permission_overwrites: Vec<PermissionOverwritesDto>,
}
