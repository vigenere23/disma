use std::collections::HashSet;

use crate::{
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole, Role},
};

#[derive(Debug, Clone)]
pub struct PermissionsOverwrites<T>
where
    T: Role,
{
    pub role: T,
    pub allow: PermissionsList,
    pub deny: PermissionsList,
}

impl<T, U> PartialEq<PermissionsOverwrites<T>> for PermissionsOverwrites<U>
where
    T: Role,
    U: Role,
{
    fn eq(&self, other: &PermissionsOverwrites<T>) -> bool {
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
    items: Vec<PermissionsOverwrites<R>>,
}

impl<R: Role> PermissionsOverwritesList<R> {
    pub fn find_by_role_name(&self, name: &str) -> Option<&PermissionsOverwrites<R>> {
        self.items
            .iter()
            .find(|overwrite| overwrite.role.name() == name)
    }

    pub fn to_list(&self) -> &Vec<PermissionsOverwrites<R>> {
        &self.items
    }
}

impl PartialEq<PermissionsOverwritesList<AwaitingRole>>
    for PermissionsOverwritesList<ExistingRole>
{
    fn eq(&self, other: &PermissionsOverwritesList<AwaitingRole>) -> bool {
        if self.to_list().len() != other.to_list().len() {
            return false;
        }

        let mut overwrites = self.to_list().clone();
        overwrites.sort_by(|a, b| a.role.name().cmp(&b.role.name()));

        let mut other_overwrited = other.to_list().clone();
        other_overwrited.sort_by(|a, b| a.role.name().cmp(&b.role.name()));

        for (overwrite, other_overwrite) in overwrites.iter().zip(other_overwrited.iter()) {
            if overwrite != other_overwrite {
                return false;
            }
        }

        true
    }
}

impl<R: Role> From<Vec<PermissionsOverwrites<R>>> for PermissionsOverwritesList<R> {
    fn from(overwrites: Vec<PermissionsOverwrites<R>>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for overwrite in overwrites.iter() {
            if role_names.contains(&overwrite.role.name()) {
                panic!("All overwrites must have unique roles.");
            }

            role_names.insert(overwrite.role.name().clone());
        }

        Self { items: overwrites }
    }
}
