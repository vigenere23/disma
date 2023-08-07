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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mock_it::eq;

    use crate::{
        category::CategoriesList,
        channel::ChannelsList,
        core::changes::role::RoleChangesService,
        guild::{ExistingGuild, GuildQuerierMock},
        params::{
            category::CategoriesParamsList, channel::ChannelsParamsList, guild::GuildParams,
            role::RolesParamsList,
        },
        role::RolesList,
    };

    use super::ListChangesUseCase;

    const GUILD_ID: &str = "abc";

    #[test]
    fn list_role_changes() {
        let querier = GuildQuerierMock::new();

        querier
            .when_get_guild(eq(GUILD_ID))
            .will_return(ExistingGuild {
                roles: RolesList::from(Vec::new()),
                categories: CategoriesList::from(Vec::new()),
                channels: ChannelsList::from(Vec::new()),
            });

        let usecase = ListChangesUseCase {
            querier: Arc::from(querier),
            role_changes_service: Arc::from(RoleChangesService {}),
        };

        let changes = usecase.execute(
            GUILD_ID,
            GuildParams {
                roles: RolesParamsList::default(),
                categories: CategoriesParamsList::default(),
                channels: ChannelsParamsList::default(),
            },
        );

        assert_eq!(changes, Vec::new());
    }
}
