use std::collections::HashSet;

use super::permission::PermissionsList;

pub trait Role: Clone {
    fn name(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ExistingRole {
    pub id: String,
    pub name: String,
    pub permissions: PermissionsList,
    pub is_mentionalbe: bool,
    pub show_in_sidebar: bool,
}

impl Role for ExistingRole {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq<AwaitingRole> for ExistingRole {
    fn eq(&self, other: &AwaitingRole) -> bool {
        self.name == other.name
            && self.is_mentionalbe == other.is_mentionalbe
            && self.show_in_sidebar == other.show_in_sidebar
            && self.permissions == other.permissions
    }
}

#[derive(Debug, Clone)]
pub struct AwaitingRole {
    pub name: String,
    pub permissions: PermissionsList,
    pub is_mentionalbe: bool,
    pub show_in_sidebar: bool,
}

impl Role for AwaitingRole {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq<ExistingRole> for AwaitingRole {
    fn eq(&self, other: &ExistingRole) -> bool {
        self.name == other.name
            && self.is_mentionalbe == other.is_mentionalbe
            && self.show_in_sidebar == other.show_in_sidebar
            && self.permissions == other.permissions
    }
}

#[derive(Debug)]
pub struct RolesList<T>
where
    T: Role,
{
    roles: Vec<T>,
    role_names: HashSet<String>,
}

impl<T: Role> RolesList<T> {
    fn new(roles: Vec<T>, role_names: HashSet<String>) -> Self {
        Self { roles, role_names }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&T> {
        self.roles.iter().find(|role| role.name() == name)
    }

    pub fn items(&self) -> &Vec<T> {
        &self.roles
    }
}

impl<T: Role> From<&Vec<T>> for RolesList<T> {
    fn from(roles: &Vec<T>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for role in roles.iter() {
            if role_names.contains(&role.name()) {
                panic!("All roles must have unique names.");
            }

            role_names.insert(role.name().clone());
        }

        Self {
            roles: roles.clone(),
            role_names,
        }
    }
}
