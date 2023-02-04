use serde::{Deserialize, Serialize};

use crate::{
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole},
};

#[derive(Debug, Serialize)]
pub struct RoleRequest {
    pub name: String,
    pub permissions: String,
    pub color: Option<u32>,
    pub hoist: bool,
    pub mentionable: bool,
}

impl From<&AwaitingRole> for RoleRequest {
    fn from(role: &AwaitingRole) -> Self {
        Self {
            name: role.name.clone(),
            permissions: role.permissions.code(),
            color: role
                .color
                .clone()
                .map(|color| u32::from_str_radix(&color, 16).unwrap()),
            hoist: role.show_in_sidebar,
            mentionable: role.is_mentionable,
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
            color => Some(format!("{:0>6}", format!("{color:x}"))),
        };

        ExistingRole {
            id: self.id,
            name: self.name,
            permissions: PermissionsList::from(self.permissions.as_str()),
            color,
            is_mentionable: self.mentionable,
            show_in_sidebar: self.hoist,
        }
    }
}
