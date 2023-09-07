use std::collections::HashSet;

use crate::permission::PermissionsList;

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionsOverwrite {
    pub name: String,
    pub allow: PermissionsList,
    pub deny: PermissionsList,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionsOverwritesList {
    items: Vec<PermissionsOverwrite>,
}

impl Default for PermissionsOverwritesList {
    fn default() -> Self {
        Self::new()
    }
}

impl PermissionsOverwritesList {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&PermissionsOverwrite> {
        self.items.iter().find(|overwrite| overwrite.name == name)
    }

    pub fn to_list(&self) -> &Vec<PermissionsOverwrite> {
        &self.items
    }
}

impl From<Vec<PermissionsOverwrite>> for PermissionsOverwritesList {
    fn from(overwrites: Vec<PermissionsOverwrite>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for overwrite in overwrites.iter() {
            if role_names.contains(&overwrite.name) {
                panic!("All overwrites must have unique roles.");
            }

            role_names.insert(overwrite.name.clone());
        }

        Self { items: overwrites }
    }
}
