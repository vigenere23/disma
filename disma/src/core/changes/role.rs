use crate::{
    core::{
        diffs::{Diff, Differ},
        ListComparison,
    },
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, ExistingRole},
};

pub enum RoleChange {
    Create(AwaitingRole),
    Update(ExistingRole, AwaitingRole, Vec<Diff>),
    Delete(ExistingRole),
}

pub struct RoleChangesService {}

impl RoleChangesService {
    pub fn list_changes(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<RoleChange> {
        let ListComparison {
            extra_self: extra_awaiting,
            extra_other: extra_existing,
            same,
        } = awaiting_guild
            .roles
            .items
            .compare_by_name(existing_guild.roles());

        let to_create = extra_awaiting
            .into_iter()
            .map(|awaiting| RoleChange::Create(awaiting.clone()));

        let to_update = same.into_iter().filter_map(|(awaiting, existing)| {
            let diffs = existing.diffs_with(awaiting);
            match diffs.is_empty() {
                true => None,
                false => Some(RoleChange::Update(
                    existing.clone(),
                    awaiting.clone(),
                    diffs,
                )),
            }
        });

        let mut to_delete: Vec<RoleChange> = Vec::new();
        for existing in extra_existing.into_iter() {
            awaiting_guild
                .roles
                .extra_items_strategy
                .handle_extra_role(existing, &mut to_delete)
        }

        to_create.chain(to_update).chain(to_delete).collect()
    }
}
