use std::sync::Arc;

use crate::domain::{
    category::ExistingCategory,
    channel::{ChannelType, ExistingChannel},
    guild::{ExistingGuild, GuildCommander, GuildQuerier, GuildSummary},
    role::{AwaitingRole, ExistingRole, RolesList},
};

use super::{api::DiscordApi, dtos::role::RoleRequest};

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

        let channel_responses = self.api.list_channels(guild_id).unwrap();

        let categories: Vec<ExistingCategory> = channel_responses
            .iter()
            .filter_map(|response| match response._type {
                4 => Some(ExistingCategory {
                    id: response.id.clone(),
                    name: response.name.clone(),
                }),
                _ => None,
            })
            .collect();

        // let channels = Vec::new();
        let channels: Vec<ExistingChannel> = channel_responses
            .iter()
            .filter_map(|response| {
                let channel_type = match response._type {
                    0 => Some(ChannelType::Text),
                    2 => Some(ChannelType::Voice),
                    4 => None,
                    other => panic!("Channel type {other} not supported."),
                };

                return match channel_type {
                    Some(channel_type) => Some(ExistingChannel {
                        id: response.id.clone(),
                        name: response.name.clone(),
                        channel_type,
                        category: None, // TODO
                        topic: response.topic.clone(),
                    }),
                    None => None,
                };
            })
            .collect();

        ExistingGuild {
            roles: RolesList::from(roles),
            categories,
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
        self.api.add_role(&self.guild_id, RoleRequest::from(role));
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) {
        self.api
            .update_role(&self.guild_id, id, RoleRequest::from(role));
    }

    fn delete_role(&self, id: &str) {
        self.api.delete_role(&self.guild_id, id);
    }
}
