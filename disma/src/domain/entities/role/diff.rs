use super::{AwaitingRole, AwaitingRolesList, ExistingRole, ExtraRolesStrategy};
use std::sync::Arc;

use crate::{
    diff::{
        base::{Diff, DiffCommandFactory, DiffCommandRef, Differ},
        roles::{AddRole, DeleteRole, UpdateRole},
    },
    guild::ExistingGuild,
    utils::misc::IfThen,
};
impl Differ<AwaitingRole> for ExistingRole {
    fn diffs_with(&self, awaiting: &AwaitingRole) -> Vec<Diff> {
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

impl PartialEq<AwaitingRole> for ExistingRole {
    fn eq(&self, other: &AwaitingRole) -> bool {
        self.name == other.name
            && self.permissions == other.permissions
            && self.color == other.color
            && self.is_mentionable == other.is_mentionable
            && self.show_in_sidebar == other.show_in_sidebar
    }
}

impl DiffCommandFactory for AwaitingRolesList {
    fn diff_commands_for(&self, existing_guild: &ExistingGuild) -> Vec<DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

        for awaiting_role in self.items.to_list() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(existing_role) => {
                    if existing_role != awaiting_role {
                        let command = UpdateRole::new(
                            existing_role.clone(),
                            awaiting_role.clone(),
                            existing_role.diffs_with(awaiting_role),
                        );
                        diffs.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddRole::new(awaiting_role.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        if self.extra_items.strategy == ExtraRolesStrategy::Remove {
            for existing_role in existing_guild.roles.to_list() {
                if self.items.find_by_name(&existing_role.name).is_none() {
                    let command = DeleteRole::new(existing_role.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        diffs
    }
}
