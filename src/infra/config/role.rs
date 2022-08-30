use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::domain::{
    permission::{Permission, PermissionsList},
    role::{AwaitingRole, ExistingRole},
};

#[derive(Serialize, Deserialize)]
pub struct RoleConfig {
    pub name: String,
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub show_in_sidebar: bool,
    pub is_mentionable: bool,
}

impl From<&ExistingRole> for RoleConfig {
    fn from(role: &ExistingRole) -> Self {
        let permissions = role
            .permissions
            .items()
            .iter()
            .map(|permission| permission.to_string())
            .collect();

        Self {
            name: role.name.clone(),
            permissions,
            color: role.color.clone(),
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionalbe,
        }
    }
}

pub struct RoleConfigAssembler {}

impl RoleConfigAssembler {
    pub fn to_awaiting(&self, role_config: &RoleConfig) -> AwaitingRole {
        let permissions: Vec<Permission> = role_config
            .permissions
            .iter()
            .map(|permission| Permission::from_str(permission).unwrap())
            .collect();

        AwaitingRole {
            name: role_config.name.clone(),
            permissions: PermissionsList::from(&permissions),
            color: role_config.color.clone().map(|color| color.to_lowercase()),
            is_mentionable: role_config.is_mentionable,
            show_in_sidebar: role_config.show_in_sidebar,
        }
    }
}
