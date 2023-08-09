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
        channel: &AwaitingChannel,
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
        channel: &AwaitingChannel,
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
