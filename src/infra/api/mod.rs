pub mod requests;
pub mod responses;

use std::sync::Arc;

use crate::{
    domain::{
        guild::{ExistingGuild, GuildCommander, GuildQuerier, GuildSummary},
        role::{AwaitingRole, ExistingRole, RolesList},
    },
    utils::http::{Client, ClientBuilder},
};

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use self::{
    requests::RoleRequest,
    responses::{ChannelResponse, GuildResponse, RoleResponse},
};

pub struct DiscordApi {
    client: Client,
}

impl DiscordApi {
    pub fn from_bot(bot_token: &str) -> DiscordApi {
        let client = ClientBuilder::new()
            .base_url("https://discord.com/api/v9")
            .header(USER_AGENT, "")
            .header(AUTHORIZATION, &format!("Bot {}", bot_token))
            .build();
        Self { client }
    }

    pub fn get_roles(&self, guild_id: &str) -> Vec<RoleResponse> {
        let url = format!("/guilds/{}/roles", guild_id);
        self.client.get(&url).send().unwrap().parsed_body().unwrap()
    }

    pub fn add_role(&self, guild_id: &str, role: &AwaitingRole) {
        let url = format!("/guilds/{}/roles", guild_id);
        self.client
            .post(&url)
            .json_body(RoleRequest::from(role))
            .unwrap()
            .send()
            .unwrap();
    }

    pub fn update_role(&self, guild_id: &str, role_id: &str, role: &AwaitingRole) {
        let url = format!("/guilds/{}/roles/{}", guild_id, role_id);
        self.client
            .patch(&url)
            .json_body(RoleRequest::from(role))
            .unwrap()
            .send()
            .unwrap();
    }

    pub fn delete_role(&self, guild_id: &str, role_id: &str) {
        let url = format!("/guilds/{}/roles/{}", guild_id, role_id);
        self.client.delete(&url).send().unwrap();
    }

    pub fn list_guilds(&self) -> Vec<GuildResponse> {
        self.client
            .get("/users/@me/guilds")
            .send()
            .unwrap()
            .parsed_body()
            .unwrap()
    }

    pub fn _get_channels(&self, guild_id: &str) -> Vec<ChannelResponse> {
        let url = format!("/guilds/{}/channels", guild_id);
        self.client.get(&url).send().unwrap().parsed_body().unwrap()
    }
}

pub struct Discord {
    api: Arc<DiscordApi>,
}

impl Discord {
    pub fn new(api: Arc<DiscordApi>) -> Self {
        Self { api }
    }
}

impl GuildQuerier for Discord {
    fn get_guild(&self, guild_id: &str) -> ExistingGuild {
        let roles: Vec<ExistingRole> = self
            .api
            .get_roles(guild_id)
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
            .into_iter()
            .map(|guild| guild.into())
            .collect()
    }
}

pub struct DiscordGuild {
    api: Arc<DiscordApi>,
    guild_id: String,
}

impl DiscordGuild {
    pub fn new(api: Arc<DiscordApi>, guild_id: &str) -> Self {
        Self {
            api,
            guild_id: String::from(guild_id),
        }
    }
}

impl GuildCommander for DiscordGuild {
    fn add_role(&self, role: &AwaitingRole) {
        self.api.add_role(&self.guild_id, role);
    }

    fn update_role(&self, id: &str, role: &AwaitingRole) {
        self.api.update_role(&self.guild_id, id, role);
    }

    fn delete_role(&self, id: &str) {
        self.api.delete_role(&self.guild_id, id);
    }
}
