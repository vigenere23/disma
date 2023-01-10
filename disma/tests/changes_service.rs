use std::sync::Arc;

use disma::{
    category::{AwaitingCategoriesList, AwaitingCategory, CategoriesList, RemoveExtraCategories},
    changes::ChangesService,
    channel::{AwaitingChannel, AwaitingChannelsList, ChannelsList, RemoveExtraChannels},
    commands::{
        CommandDescription, CommandEntity, CommandEventListenerMock, CommandEventListenerRef,
    },
    diff::Diff,
    guild::{AwaitingGuild, ExistingGuild, GuildCommanderMock, GuildQuerierMock, GuildQuerierRef},
    permission::{Permission, PermissionsList},
    role::{AwaitingRole, AwaitingRolesList, ExistingRole, RemoveExtraRoles, RolesList},
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

fn given_an_awaiting_role_with(name: &str) -> AwaitingRole {
    AwaitingRole {
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

fn given_another_awaiting_role_with(name: &str) -> AwaitingRole {
    AwaitingRole {
        name: name.to_string(),
        color: None,
        is_mentionable: false,
        show_in_sidebar: false,
        permissions: PermissionsList::from(vec![
            Permission::SEND_MESSAGES,
            Permission::CHANGE_NICKNAME,
        ]),
    }
}

fn given_empty_awaiting_guild() -> AwaitingGuild {
    AwaitingGuild {
        roles: given_awaiting_roles_list_for(vec![]),
        categories: given_awaiting_categories_list_for(vec![]),
        channels: given_awaiting_channels_list_for(vec![]),
    }
}

fn given_awaiting_roles_list_for(roles: Vec<AwaitingRole>) -> AwaitingRolesList {
    AwaitingRolesList {
        items: RolesList::from(roles),
        extra_items_strategy: Arc::from(RemoveExtraRoles {}),
    }
}

fn given_awaiting_categories_list_for(categories: Vec<AwaitingCategory>) -> AwaitingCategoriesList {
    AwaitingCategoriesList {
        items: CategoriesList::from(categories),
        extra_items_strategy: Arc::from(RemoveExtraCategories {}),
    }
}

fn given_awaiting_channels_list_for(channels: Vec<AwaitingChannel>) -> AwaitingChannelsList {
    AwaitingChannelsList {
        items: ChannelsList::from(channels),
        extra_items_strategy: Arc::from(RemoveExtraChannels {}),
    }
}

const GUILD_ID: &str = "1234";

#[test]
fn given_no_difference_when_listing_changes_then_return_no_changes() {
    let guild_commander = Arc::from(GuildCommanderMock::new());
    let event_listener = Arc::from(CommandEventListenerMock::new());

    let existing_guild = given_empty_existing_guild();
    let awaiting_guild = given_empty_awaiting_guild();

    let service = ChangesService::new(
        guild_commander.clone(),
        given_guild_querier_for(GUILD_ID, existing_guild),
        event_listener.clone(),
    );

    let diffs = service.list_changes(GUILD_ID, &awaiting_guild);

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
    let awaiting_guild = AwaitingGuild {
        roles: given_awaiting_roles_list_for(vec![
            given_an_awaiting_role_with("to_create"),
            given_an_awaiting_role_with("no_change"),
            given_another_awaiting_role_with("to_update"),
        ]),
        categories: given_awaiting_categories_list_for(vec![]),
        channels: given_awaiting_channels_list_for(vec![]),
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

    let diffs = service.list_changes(GUILD_ID, &awaiting_guild);

    assert_eq!(diffs, expected_diffs);
}

#[test]
fn given_role_differences_when_applying_changes_then_applies_all_changes() {
    let guild_commander = Arc::from(GuildCommanderMock::new());
    let event_listener = given_fake_diff_event_listener();
    let created_role = given_an_awaiting_role_with("to_create");
    let role_to_update = given_an_existing_role_with("to_update", "to_update");
    let updated_role = given_another_awaiting_role_with("to_update");
    let role_to_not_change = given_an_existing_role_with("no_change", "no_change");
    let unchanged_role = given_an_awaiting_role_with("no_change");
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
    let awaiting_guild = AwaitingGuild {
        roles: given_awaiting_roles_list_for(vec![
            created_role.clone(),
            unchanged_role.clone(),
            updated_role.clone(),
        ]),
        categories: given_awaiting_categories_list_for(vec![]),
        channels: given_awaiting_channels_list_for(vec![]),
    };
    guild_commander
        .when_add_role(eq(&created_role))
        .will_return_default();
    guild_commander
        .when_update_role(eq(&role_to_update.id), eq(&updated_role))
        .will_return_default();
    guild_commander
        .when_delete_role(eq(&role_to_delete.id))
        .will_return_default();

    let service = ChangesService::new(
        guild_commander.clone(),
        given_guild_querier_for(GUILD_ID, existing_guild),
        event_listener.clone(),
    );

    service.apply_changes(GUILD_ID, &awaiting_guild);
}

#[test]
fn given_no_difference_when_applying_changes_then_does_not_call_discord_api() {
    let guild_commander = Arc::from(GuildCommanderMock::new());
    let event_listener = Arc::from(CommandEventListenerMock::new());

    let existing_guild = given_empty_existing_guild();
    let awaiting_guild = given_empty_awaiting_guild();

    let service = ChangesService::new(
        guild_commander.clone(),
        given_guild_querier_for(GUILD_ID, existing_guild),
        event_listener.clone(),
    );

    service.apply_changes(GUILD_ID, &awaiting_guild);
}
