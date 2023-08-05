use std::sync::Arc;

use crate::{
    commands::{CommandDescription, CommandEntity},
    core::changes::role::{RoleChange, RoleChangesService},
    diff::Differ,
    guild::{AwaitingGuild, GuildQuerier},
    params::guild::GuildParams,
};

pub struct ListChangesUseCase {
    querier: Arc<dyn GuildQuerier>,
    role_changes_service: Arc<RoleChangesService>,
}

impl ListChangesUseCase {
    pub fn execute(&self, guild_id: &str, params: GuildParams) -> Vec<CommandDescription> {
        let awaiting_guild: AwaitingGuild = params.into();
        let existing_guild = self.querier.get_guild(guild_id);

        let role_changes = self
            .role_changes_service
            .list_changes(&existing_guild, &awaiting_guild);

        return role_changes
            .into_iter()
            .map(|change| match change {
                RoleChange::Create(awaiting) => {
                    CommandDescription::Create(CommandEntity::Role, awaiting.name)
                }
                RoleChange::Update(existing, awaiting) => CommandDescription::Update(
                    CommandEntity::Role,
                    existing.name.clone(),
                    existing.diffs_with(&awaiting),
                ),
                RoleChange::Delete(existing) => {
                    CommandDescription::Delete(CommandEntity::Role, existing.name)
                }
            })
            .collect();
    }
}
