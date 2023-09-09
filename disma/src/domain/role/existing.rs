use crate::permission::PermissionsList;

use super::Role;

#[derive(Debug, Clone, PartialEq)]
pub struct ExistingRole {
    pub id: String,
    pub name: String,
    pub permissions: PermissionsList,
    pub color: Option<String>,
    pub is_mentionable: bool,
    pub show_in_sidebar: bool,
}

impl Role for ExistingRole {
    fn name(&self) -> &str {
        &self.name
    }
}
