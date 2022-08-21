use std::sync::Arc;

use crate::domain::{
    guild::{ExistingGuild, GuildCommander, GuildQuerier, GuildSummary},
    role::{AwaitingRole, ExistingRole, RolesList},
};

use super::{api::DiscordApi, requests::RoleRequest};

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
            .get_roles(guild_id)
            .unwrap()
            .into_iter()
            .map(|value| value.into())
            .collect();

        ExistingGuild {
            roles: RolesList::from(roles),
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
