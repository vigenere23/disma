use serde::Serialize;

use crate::domain::role::AwaitingRole;

#[derive(Debug, Serialize)]
pub struct RoleRequest {
    pub name: String,
    pub permissions: String,
    pub hoist: bool,
    pub mentionable: bool,
}

impl From<&AwaitingRole> for RoleRequest {
    fn from(role: &AwaitingRole) -> Self {
        Self {
            name: role.name.clone(),
            permissions: role.permissions.code(),
            hoist: role.show_in_sidebar.clone(),
            mentionable: role.is_mentionalbe.clone(),
        }
    }
}
