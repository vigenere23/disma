use std::sync::Arc;

use crate::{
    commands::CommandRef,
    core::changes::role::{RoleChange, RoleChangesService},
    guild::{AwaitingGuild, GuildQuerier},
    params::guild::GuildParams,
    role::{AddRole, DeleteRole, UpdateRole},
};

pub struct ApplyChangesUseCase {
    querier: Arc<dyn GuildQuerier>,
    role_changes_service: Arc<RoleChangesService>,
}

impl ApplyChangesUseCase {
    pub fn execute(&self, guild_id: &str, params: GuildParams) -> Vec<CommandRef> {
        let mut create_commands = Vec::<CommandRef>::new();
        let mut update_commands = Vec::<CommandRef>::new();
        let mut delete_commands = Vec::<CommandRef>::new();

        let awaiting_guild: AwaitingGuild = params.into();

        self.add_role_commands(
            guild_id,
            &awaiting_guild,
            &mut create_commands,
            &mut update_commands,
            &mut delete_commands,
        );

        return create_commands
            .into_iter()
            .chain(update_commands.into_iter())
            .chain(delete_commands.into_iter())
            .collect();
    }

    fn add_role_commands(
        &self,
        guild_id: &str,
        awaiting_guild: &AwaitingGuild,
        create_commands: &mut Vec<CommandRef>,
        update_commands: &mut Vec<CommandRef>,
        delete_commands: &mut Vec<CommandRef>,
    ) {
        let role_changes = self
            .role_changes_service
            .list_changes(&self.querier.get_guild(guild_id), &awaiting_guild);

        for role_change in role_changes {
            match role_change {
                RoleChange::Create(awaiting) => {
                    create_commands.push(Arc::from(AddRole::new(awaiting)))
                }
                RoleChange::Update(existing, awaiting) => update_commands.push(Arc::from(
                    UpdateRole::try_new(&existing, &awaiting).unwrap(),
                )),
                RoleChange::Delete(existing) => {
                    delete_commands.push(Arc::from(DeleteRole::new(existing)))
                }
            }
        }
    }
}
