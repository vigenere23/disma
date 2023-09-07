use std::sync::Arc;

use crate::{
    category::{CategoriesList, ExistingCategory},
    channel::{ChannelType, ChannelsList, ExistingChannel},
    guild::{ExistingGuild, GuildQuerier, GuildSummary},
    permission::{PermissionsOverwrite, PermissionsOverwritesList},
    role::{ExistingRole, RolesList},
};

use super::api::DiscordApi;

pub struct HttpGuildQuerier {
    api: Arc<DiscordApi>,
}

impl HttpGuildQuerier {
    pub fn new(api: Arc<DiscordApi>) -> Self {
        Self { api }
    }
}

impl GuildQuerier for HttpGuildQuerier {
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
                            .filter_map(|permissions| {
                                let result = permissions._try_into(&roles_list);
                                match result {
                                    Ok(overwrites) => Some(overwrites),
                                    Err(message) => {eprintln!("Error while parsing permissions overwrites for category {}: {}", response.name.clone(), message); None}
                                }
                            })
                            .collect::<Vec<PermissionsOverwrite>>(),
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

                let category_name = response
                    .parent_id
                    .as_ref()
                    .map(|category_id| categories_list.find_by_id(category_id).name.clone());

                let overwrites = PermissionsOverwritesList::from(
                    response
                        .permission_overwrites
                        .iter()
                        .filter_map(|permissions| {
                            let result = permissions._try_into(&roles_list);
                            match result {
                                Ok(overwrites) => Some(overwrites),
                                Err(message) => {eprintln!("Error while parsing permissions overwrites for channel {}: {}", response.name.clone(), message); None}
                            }
                        })
                        .collect::<Vec<PermissionsOverwrite>>(),
                );

                Some(ExistingChannel {
                    id: response.id.clone(),
                    name: response.name.clone(),
                    channel_type,
                    category_name,
                    topic: response.topic.clone(),
                    overwrites,
                })
            })
            .collect();

        let channels_list = ChannelsList::from(channels);

        ExistingGuild {
            roles: roles_list,
            categories: categories_list,
            channels: channels_list,
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
