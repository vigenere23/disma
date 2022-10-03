use std::collections::HashSet;

use crate::diff::base::{diffs_between, option_diffs_between, Diff};

use super::permission::PermissionsList;

pub trait Role: Clone {
    fn name(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ExistingRole {
    pub id: String,
    pub name: String,
    pub permissions: PermissionsList,
    pub color: Option<String>,
    pub is_mentionable: bool,
    pub show_in_sidebar: bool,
}

impl ExistingRole {
    pub fn diffs_with(&self, awaiting_role: &AwaitingRole) -> Vec<Diff> {
        let mut diffs = vec![];

        let permissions_diffs = self.permissions.diffs_with(&awaiting_role.permissions);

        if !permissions_diffs.is_empty() {
            diffs.push(Diff::Update("permissions".into(), permissions_diffs));
        }

        let is_mentionable_diffs = diffs_between(self.is_mentionable, awaiting_role.is_mentionable);

        if !is_mentionable_diffs.is_empty() {
            diffs.push(Diff::Update("is_mentionable".into(), is_mentionable_diffs));
        }

        let show_in_sidebar_diffs =
            diffs_between(self.show_in_sidebar, awaiting_role.show_in_sidebar);

        if !show_in_sidebar_diffs.is_empty() {
            diffs.push(Diff::Update(
                "show_in_sidebar".into(),
                show_in_sidebar_diffs,
            ));
        }

        let color_diffs = option_diffs_between(self.color.as_ref(), awaiting_role.color.as_ref());

        if !color_diffs.is_empty() {
            diffs.push(Diff::Update("color".into(), color_diffs));
        }

        diffs
    }
}

impl Role for ExistingRole {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl PartialEq<AwaitingRole> for ExistingRole {
    fn eq(&self, other: &AwaitingRole) -> bool {
        self.name == other.name
            && self.permissions == other.permissions
            && self.color == other.color
            && self.is_mentionable == other.is_mentionable
            && self.show_in_sidebar == other.show_in_sidebar
    }
}

#[derive(Debug, Clone)]
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

impl PartialEq<ExistingRole> for AwaitingRole {
    fn eq(&self, other: &ExistingRole) -> bool {
        self.name == other.name
            && self.is_mentionable == other.is_mentionable
            && self.color == other.color
            && self.show_in_sidebar == other.show_in_sidebar
            && self.permissions == other.permissions
    }
}

#[derive(Debug, Clone)]
pub struct RolesList<T>
where
    T: Role,
{
    roles: Vec<T>,
}

impl<T: Role> RolesList<T> {
    pub fn find_by_name(&self, name: &str) -> Option<&T> {
        self.roles.iter().find(|role| role.name() == name)
    }

    pub fn items(&self) -> &Vec<T> {
        &self.roles
    }
}

impl RolesList<ExistingRole> {
    pub fn find_by_id(&self, id: &str) -> &ExistingRole {
        self.roles
            .iter()
            .find(|role| role.id == id)
            .unwrap_or_else(|| panic!("Could not find role with id {}", &id))
    }
}

impl<T: Role> From<Vec<T>> for RolesList<T> {
    fn from(roles: Vec<T>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for role in roles.iter() {
            if role_names.contains(&role.name()) {
                panic!("All roles must have unique names.");
            }

            role_names.insert(role.name().clone());
        }

        Self { roles }
    }
}
