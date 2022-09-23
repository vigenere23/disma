use std::str::FromStr;

use serde::{Deserialize, Serialize};

use disma::{
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

impl Into<AwaitingRole> for RoleConfig {
    fn into(self) -> AwaitingRole {
        let permissions: Vec<Permission> = self
            .permissions
            .iter()
            .map(|permission| Permission::from_str(permission).unwrap())
            .collect();

        AwaitingRole {
            name: self.name,
            permissions: PermissionsList::from(&permissions),
            color: self.color.map(|color| color.to_lowercase()),
            is_mentionable: self.is_mentionable,
            show_in_sidebar: self.show_in_sidebar,
        }
    }
}
