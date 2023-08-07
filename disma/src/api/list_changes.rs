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
    #[allow(dead_code)]
    pub fn execute(&self, guild_id: &str, params: GuildParams) -> Vec<CommandDescription> {
        let awaiting_guild: AwaitingGuild = params.into();
        let existing_guild = self.querier.get_guild(guild_id);

        let role_changes = self
            .role_changes_service
            .list_changes(&existing_guild, &awaiting_guild);

        role_changes
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
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mock_it::eq;

    use crate::{
        commands::{CommandDescription, CommandEntity},
        core::changes::role::RoleChangesService,
        diff::Diff,
        guild::GuildQuerierMock,
        test::fixtures::{
            existing::{guild::tests::ExistingGuildFixture, role::tests::ExistingRoleFixture},
            params::{guild::tests::GuildParamsFixture, role::tests::RoleParamsFixture},
        },
    };

    use super::ListChangesUseCase;

    static GUILD_ID: &str = "abc";

    #[test]
    fn when_no_changes_it_returns_empty_list() {
        let querier = GuildQuerierMock::new();
        let empty_guild = ExistingGuildFixture::default();
        let params_with_no_changes = GuildParamsFixture::default();

        querier
            .when_get_guild(eq(GUILD_ID))
            .will_return(empty_guild);

        let usecase = ListChangesUseCase {
            querier: Arc::from(querier),
            role_changes_service: Arc::from(RoleChangesService {}),
        };

        let changes = usecase.execute(GUILD_ID, params_with_no_changes);

        assert_eq!(changes, Vec::new());
    }

    #[test]
    fn can_list_role_changes() {
        let querier = GuildQuerierMock::new();

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

        let usecase = ListChangesUseCase {
            querier: Arc::from(querier),
            role_changes_service: Arc::from(RoleChangesService {}),
        };

        let changes = usecase.execute(
            GUILD_ID,
            GuildParamsFixture::new()
                .with_role(role_to_add_params.clone())
                .with_role(role_to_update_params.clone())
                .build(),
        );

        assert_eq!(
            changes,
            vec![
                CommandDescription::Create(CommandEntity::Role, role_to_add_params.name),
                CommandDescription::Update(
                    CommandEntity::Role,
                    role_to_update.name,
                    vec![Diff::Update(
                        "color".to_string(),
                        vec![Diff::Add("124f5d".to_string())]
                    )]
                ),
                CommandDescription::Delete(CommandEntity::Role, role_to_remove.name)
            ]
        );
    }
}