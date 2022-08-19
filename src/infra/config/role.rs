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
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionalbe,
        }
    }
}

impl From<&AwaitingRole> for RoleConfig {
    fn from(role: &AwaitingRole) -> Self {
        let permissions = role
            .permissions
            .items()
            .iter()
            .map(|permission| permission.to_string())
            .collect();

        Self {
            name: role.name.clone(),
            permissions,
            show_in_sidebar: role.show_in_sidebar,
            is_mentionable: role.is_mentionalbe,
        }
    }
}

impl Into<AwaitingRole> for &RoleConfig {
    fn into(self) -> AwaitingRole {
        let permissions: Vec<Permission> = self
            .permissions
            .iter()
            .map(|permission| Permission::from_str(&permission).unwrap())
            .collect();

        AwaitingRole {
            name: self.name.clone(),
            permissions: PermissionsList::from(&permissions),
            is_mentionalbe: self.is_mentionable,
            show_in_sidebar: self.show_in_sidebar,
        }
    }
}
