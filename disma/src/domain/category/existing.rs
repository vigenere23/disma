use crate::permission::PermissionsOverwritesList;

use super::{CategoriesList, Category};

#[derive(Debug, Clone, PartialEq)]
pub struct ExistingCategory {
    pub id: String,
    pub name: String,
    pub overwrites: PermissionsOverwritesList,
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
    pub fn find_by_id(&self, id: &str) -> Option<&ExistingCategory> {
        self.to_list()
            .into_iter()
            .find(|category| category.id == id)
    }
}
