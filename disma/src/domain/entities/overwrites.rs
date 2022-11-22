use std::collections::HashSet;

use crate::{
    diff::base::{Diff, Differ},
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole, Role},
    utils::misc::IfThen,
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

impl Differ<PermissionsOverwrites<AwaitingRole>> for PermissionsOverwrites<ExistingRole> {
    fn diffs_with(&self, target: &PermissionsOverwrites<AwaitingRole>) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.allow.diffs_with(&target.allow).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("allow".into(), diffs)),
        );

        self.deny.diffs_with(&target.deny).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("deny".into(), diffs)),
        );

        all_diffs
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

    pub fn items(&self) -> &Vec<PermissionsOverwrites<R>> {
        &self.items
    }
}

impl PartialEq<PermissionsOverwritesList<AwaitingRole>>
    for PermissionsOverwritesList<ExistingRole>
{
    fn eq(&self, other: &PermissionsOverwritesList<AwaitingRole>) -> bool {
        if self.items().len() != other.items().len() {
            return false;
        }

        let mut overwrites = self.items().clone();
        overwrites.sort_by(|a, b| a.role.name().cmp(&b.role.name()));

        let mut other_overwrited = other.items().clone();
        other_overwrited.sort_by(|a, b| a.role.name().cmp(&b.role.name()));

        for (overwrite, other_overwrite) in overwrites.iter().zip(other_overwrited.iter()) {
            if overwrite != other_overwrite {
                return false;
            }
        }

        true
    }
}

impl Differ<PermissionsOverwritesList<AwaitingRole>> for PermissionsOverwritesList<ExistingRole> {
    fn diffs_with(&self, target: &PermissionsOverwritesList<AwaitingRole>) -> Vec<Diff> {
        let mut all_diffs = vec![];

        for existing_overwrite in self.items.iter() {
            match target.find_by_role_name(&existing_overwrite.role.name) {
                Some(awaiting_overwrite) => {
                    existing_overwrite.diffs_with(awaiting_overwrite).if_then(
                        |diffs| !diffs.is_empty(),
                        |diffs| {
                            all_diffs
                                .push(Diff::Update(existing_overwrite.role.name.clone(), diffs))
                        },
                    );
                }
                None => all_diffs.push(Diff::Remove(existing_overwrite.role.name.clone())),
            }
        }

        for awaiting_role in target.items.iter() {
            if self.find_by_role_name(&awaiting_role.role.name).is_none() {
                all_diffs.push(Diff::Add(awaiting_role.role.name.clone()))
            }
        }

        all_diffs
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
