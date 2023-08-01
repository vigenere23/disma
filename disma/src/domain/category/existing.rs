use crate::{permission::PermissionsOverwritesList, role::ExistingRole};

use super::{CategoriesList, Category};

#[derive(Debug, Clone, PartialEq)]
pub struct ExistingCategory {
    pub id: String,
    pub name: String,
    pub overwrites: PermissionsOverwritesList<ExistingRole>,
}

impl ToString for ExistingCategory {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Category for ExistingCategory {
    fn name(&self) -> &str {
        &self.name
    }
}

impl CategoriesList<ExistingCategory> {
    pub fn find_by_id(&self, id: &str) -> &ExistingCategory {
        self.to_list()
            .iter()
            .find(|category| category.id == id)
            .unwrap_or_else(|| panic!("Could not find category with id {}", &id))
    }
}
