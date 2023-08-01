use crate::permission::PermissionsList;

use super::{Role, RolesList};

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

impl RolesList<ExistingRole> {
    pub fn find_by_id(&self, id: &str) -> &ExistingRole {
        self.to_list()
            .iter()
            .find(|role| role.id == id)
            .unwrap_or_else(|| panic!("Could not find role with id {}", &id))
    }
}
