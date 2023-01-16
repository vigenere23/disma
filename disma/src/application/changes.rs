use crate::{
    commands::{
        CommandDescription, CommandEventListenerRef, CommandEventType, CommandFactory, CommandRef,
    },
    guild::{AwaitingGuild, GuildCommanderRef, GuildQuerierRef},
    params::guild::GuildParams,
};

pub struct ChangesService {
    guild_commander: GuildCommanderRef,
    guild_querier: GuildQuerierRef,
    event_listener: CommandEventListenerRef,
}

impl ChangesService {
    pub fn new(
        guild_commander: GuildCommanderRef,
        guild_querier: GuildQuerierRef,
        event_listener: CommandEventListenerRef,
    ) -> Self {
        Self {
            guild_commander,
            guild_querier,
            event_listener,
        }
    }

    pub fn list_changes(
        &self,
        guild_id: &str,
        guild_params: GuildParams,
    ) -> Vec<CommandDescription> {
        let awaiting_guild: AwaitingGuild = guild_params.into();
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let role_commands = awaiting_guild.roles.commands_for(&existing_guild);
        let category_commands = awaiting_guild.categories.commands_for(&existing_guild);
        let channel_commands = awaiting_guild.channels.commands_for(&existing_guild);

        role_commands
            .into_iter()
            .chain(category_commands.into_iter())
            .chain(channel_commands.into_iter())
            .map(|command| command.describe())
            .collect()
    }

    pub fn apply_changes(&self, guild_id: &str, guild_params: GuildParams) {
        let awaiting_guild: AwaitingGuild = guild_params.into();

        self.apply_changes_for(guild_id, &awaiting_guild.roles);
        self.apply_changes_for(guild_id, &awaiting_guild.categories);
        self.apply_changes_for(guild_id, &awaiting_guild.channels);
    }

    fn apply_changes_for(&self, guild_id: &str, command_factory: &dyn CommandFactory) {
        let existing_guild = self.guild_querier.get_guild(guild_id);
        command_factory
            .commands_for(&existing_guild)
            .into_iter()
            .for_each(|command| self.apply_command(command));
    }

    fn apply_command(&self, command: CommandRef) {
        let description = command.describe();
        self.event_listener
            .handle(CommandEventType::BeforeExecution, description.clone());

        command.execute(&self.guild_commander);

        self.event_listener
            .handle(CommandEventType::AfterExecution, description);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        category::CategoriesList,
        changes::ChangesService,
        channel::ChannelsList,
        commands::{
            CommandDescription, CommandEntity, CommandEventListenerMock, CommandEventListenerRef,
        },
        diff::Diff,
        guild::{ExistingGuild, GuildCommanderMock, GuildQuerierMock, GuildQuerierRef},
        params::{
            category::{
                CategoriesParamsList, CategoryParams, CategoryParamsExtraItems,
                CategoryParamsExtraItemsStrategy,
            },
            channel::{
                ChannelParams, ChannelParamsExtraItems, ChannelParamsExtraItemsStrategy,
                ChannelsParamsList,
            },
            guild::GuildParams,
            role::{
                RoleParams, RoleParamsExtraItems, RoleParamsExtraItemsStrategy, RolesParamsList,
            },
        },
        permission::{Permission, PermissionsList},
        role::{ExistingRole, RolesList},
    };
    use mock_it::{any, eq};

    fn given_guild_querier_for(guild_id: &str, existing_guild: ExistingGuild) -> GuildQuerierRef {
        let guild_querier = Arc::from(GuildQuerierMock::new());
        guild_querier
            .when_get_guild(eq(guild_id))
            .will_return(existing_guild);

        guild_querier
    }

    fn given_fake_diff_event_listener() -> CommandEventListenerRef {
        let event_listener = Arc::from(CommandEventListenerMock::new());
        event_listener
            .when_handle(any(), any())
            .will_return_default();
        event_listener
    }

    fn given_empty_existing_guild() -> ExistingGuild {
        ExistingGuild {
            roles: RolesList::from(vec![]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
        }
    }

    fn given_an_existing_role_with(id: &str, name: &str) -> ExistingRole {
        ExistingRole {
            id: id.to_string(),
            name: name.to_string(),
            color: Some("abcdef".to_string()),
            is_mentionable: true,
            show_in_sidebar: true,
            permissions: PermissionsList::from(vec![
                Permission::SEND_MESSAGES,
                Permission::READ_MESSAGE_HISTORY,
            ]),
        }
    }

    fn given_role_params_with(name: &str) -> RoleParams {
        RoleParams {
            name: name.to_string(),
            color: Some("abcdef".to_string()),
            is_mentionable: true,
            show_in_sidebar: true,
            permissions: vec![Permission::SEND_MESSAGES, Permission::READ_MESSAGE_HISTORY],
        }
    }

    fn given_other_role_params_with(name: &str) -> RoleParams {
        RoleParams {
            name: name.to_string(),
            color: None,
            is_mentionable: false,
            show_in_sidebar: false,
            permissions: vec![Permission::SEND_MESSAGES, Permission::CHANGE_NICKNAME],
        }
    }

    fn given_empty_guild_params() -> GuildParams {
        GuildParams {
            roles: given_roles_params_list_for(vec![]),
            categories: given_categories_params_list_for(vec![]),
            channels: given_channels_params_list_for(vec![]),
        }
    }

    fn given_roles_params_list_for(roles: Vec<RoleParams>) -> RolesParamsList {
        RolesParamsList {
            items: roles,
            extra_items: RoleParamsExtraItems {
                strategy: RoleParamsExtraItemsStrategy::REMOVE,
            },
        }
    }

    fn given_categories_params_list_for(categories: Vec<CategoryParams>) -> CategoriesParamsList {
        CategoriesParamsList {
            items: categories,
            extra_items: CategoryParamsExtraItems {
                strategy: CategoryParamsExtraItemsStrategy::REMOVE,
            },
        }
    }

    fn given_channels_params_list_for(channels: Vec<ChannelParams>) -> ChannelsParamsList {
        ChannelsParamsList {
            items: channels,
            extra_items: ChannelParamsExtraItems {
                strategy: ChannelParamsExtraItemsStrategy::REMOVE,
            },
        }
    }

    const GUILD_ID: &str = "1234";

    #[test]
    fn given_no_difference_when_listing_changes_then_return_no_changes() {
        let guild_commander = Arc::from(GuildCommanderMock::new());
        let event_listener = Arc::from(CommandEventListenerMock::new());

        let existing_guild = given_empty_existing_guild();
        let guild_params = given_empty_guild_params();

        let service = ChangesService::new(
            guild_commander.clone(),
            given_guild_querier_for(GUILD_ID, existing_guild),
            event_listener.clone(),
        );

        let diffs = service.list_changes(GUILD_ID, guild_params);

        assert_eq!(diffs, vec![]);
    }

    #[test]
    fn given_role_differences_when_listing_changes_then_return_all_role_changes() {
        let guild_commander = Arc::from(GuildCommanderMock::new());
        let event_listener = Arc::from(CommandEventListenerMock::new());

        let existing_guild = ExistingGuild {
            roles: RolesList::from(vec![
                given_an_existing_role_with("no_change", "no_change"),
                given_an_existing_role_with("to_update", "to_update"),
                given_an_existing_role_with("to_delete", "to_delete"),
            ]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
        };
        let guild_params = GuildParams {
            roles: given_roles_params_list_for(vec![
                given_role_params_with("to_create"),
                given_role_params_with("no_change"),
                given_other_role_params_with("to_update"),
            ]),
            categories: given_categories_params_list_for(vec![]),
            channels: given_channels_params_list_for(vec![]),
        };
        let expected_diffs = vec![
            CommandDescription::Create(CommandEntity::Role, "to_create".to_string()),
            CommandDescription::Update(
                CommandEntity::Role,
                "to_update".to_string(),
                vec![
                    Diff::Update(
                        "permissions".to_string(),
                        vec![
                            Diff::Remove("READ_MESSAGE_HISTORY".to_string()),
                            Diff::Add("CHANGE_NICKNAME".to_string()),
                        ],
                    ),
                    Diff::Update(
                        "is_mentionable".to_string(),
                        vec![
                            Diff::Remove("true".to_string()),
                            Diff::Add("false".to_string()),
                        ],
                    ),
                    Diff::Update(
                        "show_in_sidebar".to_string(),
                        vec![
                            Diff::Remove("true".to_string()),
                            Diff::Add("false".to_string()),
                        ],
                    ),
                    Diff::Update(
                        "color".to_string(),
                        vec![Diff::Remove("abcdef".to_string())],
                    ),
                ],
            ),
            CommandDescription::Delete(CommandEntity::Role, "to_delete".to_string()),
        ];

        let service = ChangesService::new(
            guild_commander.clone(),
            given_guild_querier_for(GUILD_ID, existing_guild),
            event_listener.clone(),
        );

        let diffs = service.list_changes(GUILD_ID, guild_params);

        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn given_no_difference_when_applying_changes_then_does_not_call_discord_api() {
        let guild_commander = Arc::from(GuildCommanderMock::new());
        let event_listener = Arc::from(CommandEventListenerMock::new());

        let existing_guild = given_empty_existing_guild();
        let guild_params = given_empty_guild_params();

        let service = ChangesService::new(
            guild_commander.clone(),
            given_guild_querier_for(GUILD_ID, existing_guild),
            event_listener.clone(),
        );

        service.apply_changes(GUILD_ID, guild_params);
    }

    #[test]
    fn given_role_differences_when_applying_changes_then_applies_all_changes() {
        let guild_commander = Arc::from(GuildCommanderMock::new());
        let event_listener = given_fake_diff_event_listener();
        let created_role = given_role_params_with("to_create");
        let role_to_update = given_an_existing_role_with("to_update", "to_update");
        let updated_role = given_other_role_params_with("to_update");
        let role_to_not_change = given_an_existing_role_with("no_change", "no_change");
        let unchanged_role = given_role_params_with("no_change");
        let role_to_delete = given_an_existing_role_with("to_delete", "to_delete");

        let existing_guild = ExistingGuild {
            roles: RolesList::from(vec![
                role_to_not_change.clone(),
                role_to_update.clone(),
                role_to_delete.clone(),
            ]),
            categories: CategoriesList::from(vec![]),
            channels: ChannelsList::from(vec![]),
        };
        let guild_params = GuildParams {
            roles: given_roles_params_list_for(vec![
                created_role.clone(),
                unchanged_role.clone(),
                updated_role.clone(),
            ]),
            categories: given_categories_params_list_for(vec![]),
            channels: given_channels_params_list_for(vec![]),
        };
        guild_commander
            .when_add_role(eq(&created_role.into()))
            .will_return_default();
        guild_commander
            .when_update_role(eq(&role_to_update.id), eq(&updated_role.into()))
            .will_return_default();
        guild_commander
            .when_delete_role(eq(&role_to_delete.id))
            .will_return_default();

        let service = ChangesService::new(
            guild_commander.clone(),
            given_guild_querier_for(GUILD_ID, existing_guild),
            event_listener.clone(),
        );

        service.apply_changes(GUILD_ID, guild_params);
    }
}
