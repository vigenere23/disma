use crate::{permission::PermissionsOverwritesList, role::ExistingRole};

use super::Category;

#[derive(Debug, Clone, PartialEq)]
pub struct ExistingCategory {
    pub id: String,
    pub name: String,
    pub overwrites: PermissionsOverwritesList<ExistingRole>,
}

impl Category for ExistingCategory {
    fn name(&self) -> &str {
        &self.name
    }
}
