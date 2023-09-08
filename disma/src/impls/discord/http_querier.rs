use std::sync::Arc;

use crate::{
    category::{CategoriesList, ExistingCategory},
    channel::{ChannelsList, ExistingChannel},
    guild::{ExistingGuild, GuildQuerier, GuildSummary},
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
            .filter(|response| response._type == 4)
            .map(|response| response.clone().into_category(&roles_list))
            .collect();
        let categories_list = CategoriesList::from(categories);

        let channels: Vec<ExistingChannel> = channel_responses
            .iter()
            .filter(|response| [0, 2].contains(&response._type))
            .map(|response| response.clone().into_channel(&roles_list, &categories_list))
            .collect();
        let channels_list = ChannelsList::from(channels);

        ExistingGuild::new(roles_list, categories_list, channels_list)
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
