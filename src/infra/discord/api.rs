use crate::utils::http::{Client, ClientBuilder};

use reqwest::header::{AUTHORIZATION, USER_AGENT};

use super::{
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

    pub fn get_roles(&self, guild_id: &str) -> Result<Vec<RoleResponse>, String> {
        let url = format!("/guilds/{}/roles", guild_id);
        self.client.get(&url).send().unwrap().parsed_body()
    }

    pub fn add_role(&self, guild_id: &str, body: RoleRequest) {
        let url = format!("/guilds/{}/roles", guild_id);
        self.client
            .post(&url)
            .json_body(body)
            .unwrap()
            .send()
            .unwrap();
    }

    pub fn update_role(&self, guild_id: &str, role_id: &str, body: RoleRequest) {
        let url = format!("/guilds/{}/roles/{}", guild_id, role_id);
        self.client
            .patch(&url)
            .json_body(body)
            .unwrap()
            .send()
            .unwrap();
    }

    pub fn delete_role(&self, guild_id: &str, role_id: &str) {
        let url = format!("/guilds/{}/roles/{}", guild_id, role_id);
        self.client.delete(&url).send().unwrap();
    }

    pub fn list_guilds(&self) -> Result<Vec<GuildResponse>, String> {
        self.client
            .get("/users/@me/guilds")
            .send()
            .unwrap()
            .parsed_body()
    }

    pub fn _get_channels(&self, guild_id: &str) -> Result<Vec<ChannelResponse>, String> {
        let url = format!("/guilds/{}/channels", guild_id);
        self.client.get(&url).send().unwrap().parsed_body()
    }
}
