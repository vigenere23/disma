use std::sync::Arc;

use crate::{
    commands::CommandRef,
    core::changes::role::{RoleChange, RoleChangesService},
    guild::{AwaitingGuild, GuildCommander, GuildQuerier},
    params::guild::GuildParams,
    role::{AddRole, DeleteRole, UpdateRole},
};

pub struct ApplyChangesUseCase {
    querier: Arc<dyn GuildQuerier>,
    commander: Arc<dyn GuildCommander>,
    role_changes_service: Arc<RoleChangesService>,
}

impl ApplyChangesUseCase {
    #[allow(dead_code)]
    pub fn execute(&self, guild_id: &str, params: GuildParams) {
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

        create_commands
            .into_iter()
            .chain(update_commands.into_iter())
            .chain(delete_commands.into_iter())
            .for_each(|command| command.execute(&self.commander));
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
            .list_changes(&self.querier.get_guild(guild_id), awaiting_guild);

        for role_change in role_changes {
            match role_change {
                RoleChange::Create(awaiting) => {
                    create_commands.push(Arc::from(AddRole::new(awaiting)))
                }
                RoleChange::Update(existing, awaiting, _) => update_commands.push(Arc::from(
                    // No longer need to try depending on diff
                    UpdateRole::try_new(&existing, &awaiting).unwrap(),
                )),
                RoleChange::Delete(existing) => {
                    delete_commands.push(Arc::from(DeleteRole::new(existing)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mock_it::{any, eq};

    use crate::{
        core::changes::role::RoleChangesService,
        guild::{GuildCommanderMock, GuildQuerierMock},
        test::fixtures::{
            existing::{guild::tests::ExistingGuildFixture, role::tests::ExistingRoleFixture},
            params::{guild::tests::GuildParamsFixture, role::tests::RoleParamsFixture},
        },
    };

    use super::ApplyChangesUseCase;

    static GUILD_ID: &str = "abc";

    #[test]
    fn can_apply_role_changes() {
        let querier = GuildQuerierMock::new();
        let commander = GuildCommanderMock::new();

        let role_to_remove = ExistingRoleFixture::new().with_name("to_remove").build();
        let role_to_add_params = RoleParamsFixture::new().with_name("to_add").build();
        let role_to_update = ExistingRoleFixture::new().with_name("to_update").build();
        let role_to_update_params = RoleParamsFixture::new()
            .with_name("to_update")
            .with_color("124f5d")
            .build();

        querier.when_get_guild(eq(GUILD_ID)).will_return(
            ExistingGuildFixture::new()
                .with_role(role_to_remove.clone())
                .with_role(role_to_update.clone())
                .build(),
        );
        commander.when_add_role(any()).will_return_default();
        commander
            .when_update_role(any(), any())
            .will_return_default();
        commander.when_delete_role(any()).will_return_default();

        let usecase = ApplyChangesUseCase {
            querier: Arc::from(querier),
            commander: Arc::from(commander.clone()),
            role_changes_service: Arc::from(RoleChangesService {}),
        };

        usecase.execute(
            GUILD_ID,
            GuildParamsFixture::new()
                .with_role(role_to_add_params.clone())
                .with_role(role_to_update_params.clone())
                .build(),
        );

        commander.expect_add_role(eq(&role_to_add_params.into()));
        commander.expect_update_role(eq(&role_to_update.id), eq(&role_to_update_params.into()));
        commander.expect_delete_role(eq(&role_to_remove.id));
    }
}
