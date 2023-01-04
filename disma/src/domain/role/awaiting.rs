use crate::permission::PermissionsList;

use super::{Role, RolesList};

#[derive(Clone, Debug, PartialEq)]
pub struct AwaitingRolesList {
    pub items: RolesList<AwaitingRole>,
    pub extra_items: ExtraRolesOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtraRolesOptions {
    pub strategy: ExtraRolesStrategy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtraRolesStrategy {
    Keep,
    Remove,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AwaitingRole {
    pub name: String,
    pub permissions: PermissionsList,
    pub color: Option<String>,
    pub is_mentionable: bool,
    pub show_in_sidebar: bool,
}

impl Role for AwaitingRole {
    fn name(&self) -> String {
        self.name.clone()
    }
}
