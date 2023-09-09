use std::sync::Arc;

use crate::{
    api::params::guild::GuildParams,
    core::{
        changes::{
            category::{CategoryChange, CategoryChangesService},
            channel::{ChannelChange, ChannelChangesService},
            role::{RoleChange, RoleChangesService},
        },
        commands::{
            category::{AddCategory, UpdateCategory},
            channel::{AddChannel, UpdateChannel},
            role::{AddRole, UpdateRole},
            CommandRef,
        },
        events::ChangeEventListenerRef,
    },
    guild::{AwaitingGuild, ExistingGuild, GuildCommanderRef, GuildQuerierRef},
};

pub struct ApplyChangesUseCase {
    querier: GuildQuerierRef,
    commander: GuildCommanderRef,
    change_event_listener: ChangeEventListenerRef,
    role_changes_service: Arc<RoleChangesService>,
    category_changes_service: Arc<CategoryChangesService>,
    channel_changes_service: Arc<ChannelChangesService>,
}

impl ApplyChangesUseCase {
    pub fn new(
        querier: GuildQuerierRef,
        commander: GuildCommanderRef,
        change_event_listener: ChangeEventListenerRef,
        role_changes_service: Arc<RoleChangesService>,
        category_changes_service: Arc<CategoryChangesService>,
        channel_changes_service: Arc<ChannelChangesService>,
    ) -> Self {
        Self {
            querier,
            commander,
            change_event_listener,
            role_changes_service,
            category_changes_service,
            channel_changes_service,
        }
    }

    pub fn execute(&self, guild_id: &str, params: GuildParams) {
        let awaiting_guild: AwaitingGuild = params.into();
        let mut existing_guild = self.querier.get_guild(guild_id);

        self.apply_role_commands(&awaiting_guild, &mut existing_guild);
        self.apply_category_commands(&awaiting_guild, &mut existing_guild);
        self.apply_channel_commands(&awaiting_guild, &mut existing_guild);
    }

    fn apply_role_commands(
        &self,
        awaiting_guild: &AwaitingGuild,
        existing_guild: &mut ExistingGuild,
    ) {
        let mut commands = Vec::<CommandRef>::new();

        let role_changes = self
            .role_changes_service
            .list_changes(existing_guild, awaiting_guild);

        for role_change in role_changes {
            match role_change {
                RoleChange::Create(awaiting) => commands.push(Arc::from(AddRole::new(awaiting))),
                RoleChange::Update(existing, awaiting, _) => {
                    commands.push(Arc::from(UpdateRole::new(existing, awaiting)))
                }
                RoleChange::Delete(existing) => awaiting_guild
                    .roles
                    .extra_items_strategy
                    .handle_extra_role(&existing, &mut commands),
            }
        }

        self.execute_commands(commands, existing_guild);
    }

    fn apply_category_commands(
        &self,
        awaiting_guild: &AwaitingGuild,
        existing_guild: &mut ExistingGuild,
    ) {
        let mut commands = Vec::<CommandRef>::new();

        let category_changes = self
            .category_changes_service
            .list_changes(existing_guild, awaiting_guild);

        for category_change in category_changes {
            match category_change {
                CategoryChange::Create(awaiting) => {
                    commands.push(Arc::from(AddCategory::new(awaiting)))
                }
                CategoryChange::Update(existing, awaiting, _) => commands.push(Arc::from(
                    UpdateCategory::new(existing.clone(), awaiting.clone()),
                )),
                CategoryChange::Delete(existing) => awaiting_guild
                    .categories
                    .extra_items_strategy
                    .handle_extra_category(&existing, &mut commands),
            }
        }

        self.execute_commands(commands, existing_guild);
    }

    fn apply_channel_commands(
        &self,
        awaiting_guild: &AwaitingGuild,
        existing_guild: &mut ExistingGuild,
    ) {
        let mut commands = Vec::<CommandRef>::new();

        let channel_changes = self
            .channel_changes_service
            .list_changes(existing_guild, awaiting_guild);

        for channel_change in channel_changes {
            match channel_change {
                ChannelChange::Create(awaiting) => {
                    commands.push(Arc::from(AddChannel::new(awaiting)))
                }
                ChannelChange::Update(existing, awaiting, _) => commands.push(Arc::from(
                    UpdateChannel::new(existing.clone(), awaiting.clone()),
                )),
                ChannelChange::Delete(existing) => awaiting_guild
                    .channels
                    .extra_items_strategy
                    .handle_extra_channel(
                        &existing,
                        &mut commands,
                        awaiting_guild.categories.items.find_by_name(&existing.name),
                    ),
            }
        }

        self.execute_commands(commands, existing_guild);
    }

    fn execute_commands(&self, commands: Vec<CommandRef>, existing_guild: &mut ExistingGuild) {
        commands.into_iter().for_each(|command| {
            command.execute(
                self.commander.as_ref(),
                self.change_event_listener.as_ref(),
                existing_guild,
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mock_it::{any, eq};

    use crate::{
        api::params::permission::PermissionsOverwriteParams,
        core::{
            changes::{
                category::CategoryChangesService, channel::ChannelChangesService,
                role::RoleChangesService,
            },
            events::ChangeEventListenerMock,
        },
        guild::{AwaitingGuild, GuildCommanderMock, GuildQuerierMock},
        tests::fixtures::{
            existing::{
                ExistingCategoryFixture, ExistingChannelFixture, ExistingGuildFixture,
                ExistingRoleFixture,
            },
            params::{
                CategoryParamsFixture, ChannelParamsFixture, GuildParamsFixture, RoleParamsFixture,
            },
        },
    };

    use super::ApplyChangesUseCase;

    static GUILD_ID: &str = "abc";
    static A_ROLE_NAME: &str = "role";
    static A_CATEGORY_NAME: &str = "a_category";

    fn create_usecase(
        querier: &GuildQuerierMock,
        commander: &GuildCommanderMock,
    ) -> ApplyChangesUseCase {
        let change_event_listener = ChangeEventListenerMock::new();
        change_event_listener
            .when_handle(any())
            .will_return_default();

        ApplyChangesUseCase::new(
            Arc::from(querier.clone()),
            Arc::from(commander.clone()),
            Arc::from(change_event_listener),
            Arc::from(RoleChangesService {}),
            Arc::from(CategoryChangesService {}),
            Arc::from(ChannelChangesService {}),
        )
    }

    fn prepare_commander_for_roles(commander: &GuildCommanderMock) {
        commander
            .when_add_role(any())
            .will_return(Ok(ExistingRoleFixture::new().build()));
        commander
            .when_update_role(any(), any())
            .will_return(Ok(ExistingRoleFixture::new().build()));
        commander.when_delete_role(any()).will_return(Ok(()));
    }

    fn prepare_commander_for_categories(commander: &GuildCommanderMock) {
        commander
            .when_add_category(any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));
        commander
            .when_update_category(any(), any(), any())
            .will_return(Ok(ExistingCategoryFixture::new().build()));
        commander.when_delete_category(any()).will_return(Ok(()));
    }

    fn prepare_commander_for_channels(commander: &GuildCommanderMock) {
        commander
            .when_add_channel(any(), any(), any())
            .will_return(Ok(ExistingChannelFixture::new().build()));
        commander
            .when_update_channel(any(), any(), any(), any())
            .will_return(Ok(ExistingChannelFixture::new().build()));
        commander.when_delete_channel(any()).will_return(Ok(()));
    }

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
        prepare_commander_for_roles(&commander);

        let usecase = create_usecase(&querier, &commander);

        usecase.execute(
            GUILD_ID,
            GuildParamsFixture::new()
                .with_role(role_to_add_params.clone())
                .with_role(role_to_update_params.clone())
                .with_role(role_not_to_update_params.clone())
                .build(),
        );

        commander.expect_add_role(eq(&role_to_add_params.into()));
        commander.expect_update_role(eq(&role_to_update.id), eq(&role_to_update_params.into()));
        commander.expect_delete_role(eq(&role_to_remove.id));
    }

    #[test]
    fn can_apply_category_changes() {
        let querier = GuildQuerierMock::new();
        let commander = GuildCommanderMock::new();

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

        let existing_guild = ExistingGuildFixture::new()
            .with_role(ExistingRoleFixture::new().with_name(A_ROLE_NAME).build())
            .with_category(category_to_remove.clone())
            .with_category(category_to_update.clone())
            .with_category(category_not_to_update.clone())
            .build();
        querier
            .when_get_guild(eq(GUILD_ID))
            .will_return(existing_guild.clone());
        prepare_commander_for_categories(&commander);

        let usecase = create_usecase(&querier, &commander);

        let params = GuildParamsFixture::new()
            .with_role(RoleParamsFixture::new().with_name(A_ROLE_NAME).build())
            .with_category(category_to_add_params.clone())
            .with_category(category_to_update_params.clone())
            .with_category(category_not_to_update_params.clone())
            .build();
        usecase.execute(GUILD_ID, params.clone());

        // TODO the fact that these need access to the awaiting_guild and existing_guild roles list
        // is a smell : maybe categories should not contain entire roles
        let awaiting_guild: AwaitingGuild = params.into();
        let roles_list = awaiting_guild.roles.items;
        commander.expect_add_category(
            eq(&category_to_add_params.into(&roles_list)),
            eq(existing_guild.roles()),
        );
        commander.expect_update_category(
            eq(&category_to_update.id),
            eq(&category_to_update_params.into(&roles_list)),
            eq(existing_guild.roles()),
        );
        commander.expect_delete_category(eq(&category_to_remove.id));
    }

    #[test]
    fn can_apply_channel_changes() {
        let querier = GuildQuerierMock::new();
        let commander = GuildCommanderMock::new();

        let channel_to_remove = ExistingChannelFixture::new()
            .with_name("to_remove")
            .with_id("to_remove")
            .build();
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
            .with_id("category_change")
            .build();
        let channel_to_change_category_params = ChannelParamsFixture::new()
            .with_name("category_change")
            .with_category(A_CATEGORY_NAME)
            .build();

        let existing_guild = ExistingGuildFixture::new()
            .with_category(
                ExistingCategoryFixture::new()
                    .with_name(A_CATEGORY_NAME)
                    .build(),
            )
            .with_channel(channel_to_remove.clone())
            .with_channel(channel_to_update.clone())
            .with_channel(channel_not_to_update.clone())
            .with_channel(channel_to_change_category.clone())
            .build();
        querier
            .when_get_guild(eq(GUILD_ID))
            .will_return(existing_guild.clone());
        prepare_commander_for_categories(&commander);
        prepare_commander_for_channels(&commander);

        let usecase = create_usecase(&querier, &commander);

        let params = GuildParamsFixture::new()
            .with_category(
                CategoryParamsFixture::new()
                    .with_name(A_CATEGORY_NAME)
                    .build(),
            )
            .with_channel(channel_to_add_params.clone())
            .with_channel(channel_to_update_params.clone())
            .with_channel(channel_not_to_update_params.clone())
            .with_channel(channel_to_change_category_params.clone())
            .build();
        usecase.execute(GUILD_ID, params.clone());

        // TODO the fact that these need access to the awaiting_guild and existing_guild roles list
        // is a smell : maybe categories should not contain entire roles
        // TODO does not verify that commander methods are not called
        let awaiting_guild: AwaitingGuild = params.into();
        commander.expect_add_channel(
            eq(&channel_to_add_params.into(
                &awaiting_guild.roles.items,
                &awaiting_guild.categories.items,
            )),
            eq(existing_guild.roles()),
            eq(existing_guild.categories()),
        );
        commander.expect_update_channel(
            eq(&channel_to_update.id),
            eq(&channel_to_update_params.into(
                &awaiting_guild.roles.items,
                &awaiting_guild.categories.items,
            )),
            eq(existing_guild.roles()),
            eq(existing_guild.categories()),
        );
        commander.expect_delete_channel(eq(&channel_to_remove.id));
        commander.expect_delete_channel(eq(&channel_to_change_category.id));
    }
}
