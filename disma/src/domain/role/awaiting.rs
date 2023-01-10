use std::sync::Arc;

use crate::permission::PermissionsList;

use super::{ExtraRolesStrategy, Role, RolesList};

#[derive(Debug)]
pub struct AwaitingRolesList {
    pub items: RolesList<AwaitingRole>,
    pub extra_items_strategy: Arc<dyn ExtraRolesStrategy>,
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
