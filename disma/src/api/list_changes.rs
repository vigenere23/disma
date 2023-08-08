use std::sync::Arc;

use crate::{
    channel::Channel,
    commands::{CommandDescription, CommandEntity},
    core::changes::{
        category::{CategoryChange, CategoryChangesService},
        channel::{ChannelChange, ChannelChangesService},
        role::{RoleChange, RoleChangesService},
    },
    guild::{AwaitingGuild, ExistingGuild, GuildQuerier},
    params::guild::GuildParams,
};

pub struct ListChangesUseCase {
    querier: Arc<dyn GuildQuerier>,
    role_changes_service: Arc<RoleChangesService>,
    category_changes_service: Arc<CategoryChangesService>,
    channel_changes_service: Arc<ChannelChangesService>,
}

impl ListChangesUseCase {
    #[allow(dead_code)]
    pub fn execute(&self, guild_id: &str, params: GuildParams) -> Vec<CommandDescription> {
        let awaiting_guild: AwaitingGuild = params.into();
        let existing_guild = self.querier.get_guild(guild_id);

        self.list_role_changes(&existing_guild, &awaiting_guild)
            .chain(self.list_category_changes(&existing_guild, &awaiting_guild))
            .chain(self.list_channel_changes(&existing_guild, &awaiting_guild))
            .collect()
    }

    fn list_role_changes(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> impl Iterator<Item = CommandDescription> {
        let role_changes = self
            .role_changes_service
            .list_changes(existing_guild, awaiting_guild);

        role_changes.into_iter().map(|change| match change {
            RoleChange::Create(awaiting) => {
                CommandDescription::Create(CommandEntity::Role, awaiting.name)
            }
            RoleChange::Update(existing, _, diffs) => {
                CommandDescription::Update(CommandEntity::Role, existing.name.clone(), diffs)
            }
            RoleChange::Delete(existing) => {
                CommandDescription::Delete(CommandEntity::Role, existing.name)
            }
        })
    }

    fn list_category_changes(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> impl Iterator<Item = CommandDescription> {
        let category_changes = self
            .category_changes_service
            .list_changes(existing_guild, awaiting_guild);

        category_changes.into_iter().map(|change| match change {
            CategoryChange::Create(awaiting) => {
                CommandDescription::Create(CommandEntity::Category, awaiting.name)
            }
            CategoryChange::Update(existing, _, diffs) => {
                CommandDescription::Update(CommandEntity::Category, existing.name.clone(), diffs)
            }
            CategoryChange::Delete(existing) => {
                CommandDescription::Delete(CommandEntity::Category, existing.name)
            }
        })
    }

    fn list_channel_changes(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> impl Iterator<Item = CommandDescription> {
        let channel_changes = self
            .channel_changes_service
            .list_changes(existing_guild, awaiting_guild);

        channel_changes.into_iter().map(|change| match change {
            ChannelChange::Create(awaiting) => CommandDescription::Create(
                CommandEntity::Channel,
                awaiting.unique_name().to_string(),
            ),
            ChannelChange::Update(existing, _, diffs) => CommandDescription::Update(
                CommandEntity::Channel,
                existing.unique_name().to_string(),
                diffs,
            ),
            ChannelChange::Delete(existing) => CommandDescription::Delete(
                CommandEntity::Channel,
                existing.unique_name().to_string(),
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mock_it::eq;

    use crate::{
        commands::{CommandDescription, CommandEntity},
        core::changes::{
            category::CategoryChangesService, channel::ChannelChangesService,
            role::RoleChangesService,
        },
        diff::Diff,
        guild::GuildQuerierMock,
        params::permission::PermissionsOverwriteParams,
        test::fixtures::{
            existing::{
                category::tests::ExistingCategoryFixture, channel::tests::ExistingChannelFixture,
                guild::tests::ExistingGuildFixture, role::tests::ExistingRoleFixture,
            },
            params::{
                category::tests::CategoryParamsFixture, channel::tests::ChannelParamsFixture,
                guild::tests::GuildParamsFixture, role::tests::RoleParamsFixture,
            },
        },
    };

    use super::ListChangesUseCase;

    static GUILD_ID: &str = "abc";
    static A_ROLE_NAME: &str = "a_role";
    static A_CATEGORY_NAME: &str = "a_category";

    fn create_usecase(querier: GuildQuerierMock) -> ListChangesUseCase {
        ListChangesUseCase {
            querier: Arc::from(querier),
            role_changes_service: Arc::from(RoleChangesService {}),
            category_changes_service: Arc::from(CategoryChangesService {}),
            channel_changes_service: Arc::from(ChannelChangesService {}),
        }
    }

    #[test]
    fn when_no_changes_it_returns_empty_list() {
        let querier = GuildQuerierMock::new();
        let empty_guild = ExistingGuildFixture::new().build();
        let params_with_no_changes = GuildParamsFixture::new().build();

        querier
            .when_get_guild(eq(GUILD_ID))
            .will_return(empty_guild);

        let usecase = create_usecase(querier);

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
        let role_not_to_update = ExistingRoleFixture::new()
            .with_name("not_to_update")
            .build();
        let role_not_to_update_params = RoleParamsFixture::new().with_name("not_to_update").build();

        querier.when_get_guild(eq(GUILD_ID)).will_return(
            ExistingGuildFixture::new()
                .with_role(role_to_remove.clone())
                .with_role(role_to_update.clone())
                .with_role(role_not_to_update.clone())
                .build(),
        );

        let usecase = create_usecase(querier);

        let changes = usecase.execute(
            GUILD_ID,
            GuildParamsFixture::new()
                .with_role(role_to_add_params.clone())
                .with_role(role_to_update_params.clone())
                .with_role(role_not_to_update_params.clone())
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

    #[test]
    fn can_list_category_changes() {
        let querier = GuildQuerierMock::new();

        let category_to_remove = ExistingCategoryFixture::new()
            .with_name("to_remove")
            .build();
        let category_to_add_params = CategoryParamsFixture::new().with_name("to_add").build();
        let category_to_update = ExistingCategoryFixture::new()
            .with_name("to_update")
            .build();
        let category_not_to_update = ExistingCategoryFixture::new()
            .with_name("not_to_update")
            .build();
        let category_to_update_params = CategoryParamsFixture::new()
            .with_name("to_update")
            .with_permissions_overwrite(PermissionsOverwriteParams {
                role: A_ROLE_NAME.to_string(),
                allow: Vec::new(),
                deny: Vec::new(),
            })
            .build();
        let category_not_to_update_params = CategoryParamsFixture::new()
            .with_name("not_to_update")
            .keep_extra_channels()
            .build();

        querier.when_get_guild(eq(GUILD_ID)).will_return(
            ExistingGuildFixture::new()
                .with_role(ExistingRoleFixture::new().with_name(A_ROLE_NAME).build())
                .with_category(category_to_remove.clone())
                .with_category(category_to_update.clone())
                .with_category(category_not_to_update.clone())
                .build(),
        );

        let usecase = create_usecase(querier);

        let changes = usecase.execute(
            GUILD_ID,
            GuildParamsFixture::new()
                .with_role(RoleParamsFixture::new().with_name(A_ROLE_NAME).build())
                .with_category(category_to_add_params.clone())
                .with_category(category_to_update_params.clone())
                .with_category(category_not_to_update_params.clone())
                .build(),
        );

        assert_eq!(
            changes,
            vec![
                CommandDescription::Create(CommandEntity::Category, category_to_add_params.name),
                CommandDescription::Update(
                    CommandEntity::Category,
                    category_to_update.name,
                    vec![Diff::Update(
                        "overwrites".to_string(),
                        vec![Diff::Add(A_ROLE_NAME.to_string())]
                    )]
                ),
                CommandDescription::Delete(CommandEntity::Category, category_to_remove.name)
            ]
        );
    }

    #[test]
    fn can_list_channel_changes() {
        let querier = GuildQuerierMock::new();

        let channel_to_remove = ExistingChannelFixture::new().with_name("to_remove").build();
        let channel_to_add_params = ChannelParamsFixture::new().with_name("to_add").build();
        let channel_to_update = ExistingChannelFixture::new().with_name("to_update").build();
        let channel_not_to_update = ExistingChannelFixture::new()
            .with_name("not_to_update")
            .build();
        let channel_to_update_params = ChannelParamsFixture::new()
            .with_name("to_update")
            .with_topic("new_topic")
            .build();
        let channel_not_to_update_params = ChannelParamsFixture::new()
            .with_name("not_to_update")
            // TODO keep like that or change that should not trigger update
            .build();
        let channel_to_change_category = ExistingChannelFixture::new()
            .with_name("category_change")
            .build();
        let channel_to_change_category_params = ChannelParamsFixture::new()
            .with_name("category_change")
            .with_category(A_CATEGORY_NAME)
            .build();

        querier.when_get_guild(eq(GUILD_ID)).will_return(
            ExistingGuildFixture::new()
                .with_category(
                    ExistingCategoryFixture::new()
                        .with_name(A_CATEGORY_NAME)
                        .build(),
                )
                .with_channel(channel_to_remove.clone())
                .with_channel(channel_to_update.clone())
                .with_channel(channel_not_to_update.clone())
                .with_channel(channel_to_change_category.clone())
                .build(),
        );

        let usecase = create_usecase(querier);

        let changes = usecase.execute(
            GUILD_ID,
            GuildParamsFixture::new()
                .with_category(
                    CategoryParamsFixture::new()
                        .with_name(A_CATEGORY_NAME)
                        .build(),
                )
                .with_channel(channel_to_add_params.clone())
                .with_channel(channel_to_update_params.clone())
                .with_channel(channel_not_to_update_params.clone())
                .with_channel(channel_to_change_category_params.clone())
                .build(),
        );

        // TODO should contain the UniqueChannelName and not the pre-computed string
        assert_eq!(
            changes,
            vec![
                CommandDescription::Create(
                    CommandEntity::Channel,
                    "a_category:category_change (TEXT)".to_string()
                ),
                CommandDescription::Create(CommandEntity::Channel, ":to_add (TEXT)".to_string()),
                CommandDescription::Update(
                    CommandEntity::Channel,
                    ":to_update (TEXT)".to_string(),
                    vec![Diff::Update(
                        "topic".to_string(),
                        vec![Diff::Add("new_topic".to_string())]
                    )]
                ),
                CommandDescription::Delete(
                    CommandEntity::Channel,
                    ":category_change (TEXT)".to_string()
                ),
                CommandDescription::Delete(CommandEntity::Channel, ":to_remove (TEXT)".to_string()),
            ]
        );
    }
}
