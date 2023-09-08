use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::AwaitingChannel,
    guild::GuildCommander,
    role::{AwaitingRole, ExistingRole, RolesList},
};

use super::{
    api::DiscordApi,
    dtos::{channel::ChannelRequest, role::RoleRequest},
};

pub struct HttpGuildCommander {
    api: Arc<DiscordApi>,
    guild_id: String,
}

impl HttpGuildCommander {
    pub fn new(api: Arc<DiscordApi>, guild_id: &str) -> Self {
        Self {
            api,
            guild_id: String::from(guild_id),
        }
    }
}

impl GuildCommander for HttpGuildCommander {
    fn add_role(&self, role: &AwaitingRole) -> Result<ExistingRole, String> {
        self.api
            .add_role(&self.guild_id, RoleRequest::from(role))
            .map(|response| response.into())
            .map_err(|error| error.to_string())
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) -> Result<ExistingRole, String> {
        self.api
            .update_role(&self.guild_id, id, RoleRequest::from(role))
            .map(|response| response.into())
            .map_err(|error| error.to_string())
    }

    fn delete_role(&self, id: &str) -> Result<(), String> {
        self.api
            .delete_role(&self.guild_id, id)
            .map_err(|error| error.to_string())
    }

    fn add_category(
        &self,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) -> Result<ExistingCategory, String> {
        self.api
            .add_channel(
                &self.guild_id,
                ChannelRequest::from_category(category, roles),
            )
            .map(|response| response._into(roles))
            .map_err(|error| error.to_string())
    }

    fn update_category(
        &self,
        id: &str,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) -> Result<ExistingCategory, String> {
        self.api
            .update_channel(id, ChannelRequest::from_category(category, roles))
            .map(|response| response._into(roles))
            .map_err(|error| error.to_string())
    }

    fn delete_category(&self, id: &str) -> Result<(), String> {
        self.api
            .delete_channel(id)
            .map_err(|error| error.to_string())
    }

    fn add_channel(
        &self,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Result<(), String> {
        self.api
            .add_channel(
                &self.guild_id,
                ChannelRequest::from_channel(channel, roles, categories),
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn update_channel(
        &self,
        id: &str,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Result<(), String> {
        self.api
            .update_channel(id, ChannelRequest::from_channel(channel, roles, categories))
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn delete_channel(&self, id: &str) -> Result<(), String> {
        self.api
            .delete_channel(id)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
}
