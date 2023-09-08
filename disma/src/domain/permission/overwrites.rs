use std::collections::HashSet;

use crate::{permission::PermissionsList, role::Role};

#[derive(Debug, Clone)]
pub struct PermissionsOverwrite<T>
where
    T: Role,
{
    pub role: T,
    pub allow: PermissionsList,
    pub deny: PermissionsList,
}

impl<T, U> PartialEq<PermissionsOverwrite<T>> for PermissionsOverwrite<U>
where
    T: Role,
    U: Role,
{
    fn eq(&self, other: &PermissionsOverwrite<T>) -> bool {
        self.role.name() == other.role.name()
            && self.allow == other.allow
            && self.deny == other.deny
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionsOverwritesList<R>
where
    R: Role,
{
    items: Vec<PermissionsOverwrite<R>>,
}

impl<R: Role> Default for PermissionsOverwritesList<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Role> PermissionsOverwritesList<R> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn find_by_role_name(&self, name: &str) -> Option<&PermissionsOverwrite<R>> {
        self.items
            .iter()
            .find(|overwrite| overwrite.role.name() == name)
    }

    pub fn to_list(&self) -> &Vec<PermissionsOverwrite<R>> {
        &self.items
    }
}

impl<R: Role> From<Vec<PermissionsOverwrite<R>>> for PermissionsOverwritesList<R> {
    fn from(overwrites: Vec<PermissionsOverwrite<R>>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for overwrite in overwrites.iter() {
            if role_names.contains(overwrite.role.name()) {
                panic!("All overwrites must have unique roles.");
            }

            role_names.insert(overwrite.role.name().to_string());
        }

        Self { items: overwrites }
    }
}
