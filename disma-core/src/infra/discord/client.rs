use std::sync::Arc;

use crate::{
    channel::{ChannelType, ExistingChannel},
    domain::entities::{
        category::{AwaitingCategory, CategoriesList, ExistingCategory},
        guild::{ExistingGuild, GuildCommander, GuildQuerier, GuildSummary},
        role::{AwaitingRole, ExistingRole, RolesList},
    },
    overwrites::{PermissionsOverwrites, PermissionsOverwritesList},
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
                    overwrites: PermissionsOverwritesList::from(
                        response
                            .permission_overwrites
                            .iter()
                            .map(|permissions| permissions.into(&roles_list))
                            .collect::<Vec<PermissionsOverwrites<ExistingRole>>>(),
                    ),
                }),
                _ => None,
            })
            .collect();

        let categories_list = CategoriesList::from(categories);

        let channels: Vec<ExistingChannel> = channel_responses
            .iter()
            .filter_map(|response| {
                let channel_type = match response._type {
                    0 => ChannelType::TEXT,
                    2 => ChannelType::VOICE,
                    _ => return None,
                };

                let category = response
                    .parent_id
                    .as_ref()
                    .map(|category_id| categories_list.find_by_id(category_id).clone());

                let overwrites = PermissionsOverwritesList::from(
                    response
                        .permission_overwrites
                        .iter()
                        .map(|permissions| permissions.into(&roles_list))
                        .collect::<Vec<PermissionsOverwrites<ExistingRole>>>(),
                );

                Some(ExistingChannel {
                    id: response.id.clone(),
                    name: response.name.clone(),
                    channel_type,
                    category,
                    topic: response.topic.clone(),
                    overwrites,
                })
            })
            .collect();

        ExistingGuild {
            roles: roles_list,
            categories: categories_list,
            channels,
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
        self.api
            .add_role(&self.guild_id, RoleRequest::from(role))
            .unwrap();
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) {
        self.api
            .update_role(&self.guild_id, id, RoleRequest::from(role))
            .unwrap();
    }

    fn delete_role(&self, id: &str) {
        self.api.delete_role(&self.guild_id, id).unwrap();
    }

    fn add_category(&self, category: &AwaitingCategory, roles: &RolesList<ExistingRole>) {
        self.api
            .add_channel(
                &self.guild_id,
                ChannelRequest::from_category(category, roles),
            )
            .unwrap();
    }

    fn update_category(
        &self,
        id: &str,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) {
        self.api
            .update_channel(id, ChannelRequest::from_category(category, roles))
            .unwrap();
    }

    fn delete_category(&self, id: &str) {
        self.api.delete_channel(id).unwrap();
    }

    fn add_channel(
        &self,
        channel: &crate::channel::AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) {
        self.api
            .add_channel(
                &self.guild_id,
                ChannelRequest::from_channel(channel, roles, categories),
            )
            .unwrap();
    }

    fn update_channel(
        &self,
        id: &str,
        channel: &crate::channel::AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) {
        self.api
            .update_channel(id, ChannelRequest::from_channel(channel, roles, categories))
            .unwrap();
    }

    fn delete_channel(&self, id: &str) {
        self.api.delete_channel(id).unwrap();
    }
}
