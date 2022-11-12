use std::collections::HashSet;

use super::permission::PermissionsList;
use crate::{
    diff::base::{Diff, Differ},
    utils::misc::IfThen,
};

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
    pub fn diffs_with(&self, awaiting: &AwaitingRole) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.permissions.diffs_with(&awaiting.permissions).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("permissions".into(), diffs)),
        );

        self.is_mentionable
            .diffs_with(&awaiting.is_mentionable)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("is_mentionable".into(), diffs)),
            );

        self.show_in_sidebar
            .diffs_with(&awaiting.show_in_sidebar)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("show_in_sidebar".into(), diffs)),
            );

        self.color.diffs_with(&awaiting.color).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("color".into(), diffs)),
        );

        all_diffs
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

#[derive(Debug, Clone, PartialEq)]
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
pub struct RolesList<R>
where
    R: Role,
{
    items: Vec<R>,
}

impl<R: Role> RolesList<R> {
    pub fn find_by_name(&self, name: &str) -> Option<&R> {
        self.items.iter().find(|role| role.name() == name)
    }

    pub fn items(&self) -> &Vec<R> {
        &self.items
    }
}

impl RolesList<ExistingRole> {
    pub fn find_by_id(&self, id: &str) -> &ExistingRole {
        self.items
            .iter()
            .find(|role| role.id == id)
            .unwrap_or_else(|| panic!("Could not find role with id {}", &id))
    }
}

impl<R: Role> From<Vec<R>> for RolesList<R> {
    fn from(roles: Vec<R>) -> Self {
        let mut role_names: HashSet<String> = HashSet::new();

        for role in roles.iter() {
            if role_names.contains(&role.name()) {
                panic!("All roles must have unique names.");
            }

            role_names.insert(role.name().clone());
        }

        Self { items: roles }
    }
}
