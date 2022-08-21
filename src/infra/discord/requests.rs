use serde::Serialize;

use crate::domain::role::AwaitingRole;

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
