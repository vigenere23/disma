use std::sync::Arc;

use crate::domain::{
    category::{AwaitingCategory, CategoriesList, CategoryPermissionsOverwrites, ExistingCategory},
    guild::{ExistingGuild, GuildCommander, GuildQuerier, GuildSummary},
    permission::PermissionsList,
    role::{AwaitingRole, ExistingRole, RolesList},
};

use super::{
    api::DiscordApi,
    dtos::{channel::ChannelRequest, role::RoleRequest},
};

pub struct DiscordClient {
    api: Arc<DiscordApi>,
}

impl DiscordClient {
    pub fn new(api: Arc<DiscordApi>) -> Self {
        Self { api }
    }
}

impl GuildQuerier for DiscordClient {
    fn get_guild(&self, guild_id: &str) -> ExistingGuild {
        let roles: Vec<ExistingRole> = self
            .api
            .list_roles(guild_id)
            .unwrap()
            .into_iter()
            .map(|value| value.into())
            .collect();

        let roles_list = RolesList::from(roles);

        let channel_responses = self.api.list_channels(guild_id).unwrap();

        let categories: Vec<ExistingCategory> = channel_responses
            .iter()
            .filter_map(|response| match response._type {
                4 => Some(ExistingCategory {
                    id: response.id.clone(),
                    name: response.name.clone(),
                    permissions_overwrites: response.permission_overwrites.as_ref().map(
                        |overwrites| {
                            overwrites
                                .iter()
                                .map(|permissions| CategoryPermissionsOverwrites {
                                    role: roles_list.find_by_id(&permissions.role_id).clone(),
                                    allow: PermissionsList::from(permissions.allow.as_str()),
                                    deny: PermissionsList::from(permissions.deny.as_str()),
                                })
                                .collect()
                        },
                    ),
                }),
                _ => None,
            })
            .collect();

        // let channels: Vec<ExistingChannel> = channel_responses
        //     .iter()
        //     .filter_map(|response| {
        //         let channel_type = match response._type {
        //             0 => Some(ChannelType::Text),
        //             2 => Some(ChannelType::Voice),
        //             4 => None,
        //             other => panic!("Channel type {other} not supported."),
        //         };

        //         channel_type.map(|channel_type| ExistingChannel {
        //             id: response.id.clone(),
        //             name: response.name.clone(),
        //             channel_type,
        //             category: None, // TODO
        //             topic: response.topic.clone(),
        //         })
        //     })
        //     .collect();

        ExistingGuild {
            roles: roles_list,
            categories: CategoriesList::from(categories),
            // channels,
        }
    }

    fn list_guilds(&self) -> Vec<GuildSummary> {
        self.api
            .list_guilds()
            .unwrap()
            .into_iter()
            .map(|guild| guild.into())
            .collect()
    }
}

pub struct DiscordGuildClient {
    api: Arc<DiscordApi>,
    guild_id: String,
}

impl DiscordGuildClient {
    pub fn new(api: Arc<DiscordApi>, guild_id: &str) -> Self {
        Self {
            api,
            guild_id: String::from(guild_id),
        }
    }
}

impl GuildCommander for DiscordGuildClient {
    fn add_role(&self, role: &AwaitingRole) {
        self.api.add_role(&self.guild_id, RoleRequest::from(role));
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) {
        self.api
            .update_role(&self.guild_id, id, RoleRequest::from(role));
    }

    fn delete_role(&self, id: &str) {
        self.api.delete_role(&self.guild_id, id);
    }

    fn add_category(&self, category: &AwaitingCategory, roles: &RolesList<ExistingRole>) {
        self.api
            .add_channel(&self.guild_id, ChannelRequest::from(category, roles));
    }

    fn update_category(
        &self,
        id: &str,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) {
        self.api
            .update_channel(id, ChannelRequest::from(category, roles));
    }

    fn delete_category(&self, id: &str) {
        self.api.delete_channel(id);
    }
}
