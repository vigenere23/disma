use crate::utils::http::{Client, ClientBuilder, Response};

use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    StatusCode,
};

use super::dtos::{
    channel::{ChannelRequest, ChannelResponse},
    guild::GuildResponse,
    role::{RoleRequest, RoleResponse},
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

    pub fn list_roles(&self, guild_id: &str) -> Result<Vec<RoleResponse>, String> {
        let url = format!("/guilds/{}/roles", guild_id);
        self.client.get(&url).send().unwrap().parsed_body()
    }

    pub fn add_role(&self, guild_id: &str, body: RoleRequest) -> Result<RoleResponse, String> {
        let url = format!("/guilds/{}/roles", guild_id);
        let response = self.handle_request(|| self.client.post(&url).json_body(body)?.send());
        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn update_role(
        &self,
        guild_id: &str,
        role_id: &str,
        body: RoleRequest,
    ) -> Result<RoleResponse, String> {
        let url = format!("/guilds/{}/roles/{}", guild_id, role_id);
        let response = self.handle_request(|| self.client.patch(&url).json_body(body)?.send());
        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn delete_role(&self, guild_id: &str, role_id: &str) -> Result<(), String> {
        let url = format!("/guilds/{}/roles/{}", guild_id, role_id);
        let response = self.handle_request(|| self.client.delete(&url).send());
        self.handle_response(response).map(|_| ())
    }

    pub fn list_guilds(&self) -> Result<Vec<GuildResponse>, String> {
        let response = self.handle_request(|| self.client.get("/users/@me/guilds").send());
        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn list_channels(&self, guild_id: &str) -> Result<Vec<ChannelResponse>, String> {
        let url = format!("/guilds/{}/channels", guild_id);
        let response = self.handle_request(|| self.client.get(&url).send());
        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn add_channel(
        &self,
        guild_id: &str,
        body: ChannelRequest,
    ) -> Result<ChannelResponse, String> {
        let url = format!("/guilds/{}/channels", guild_id);
        let response = self.handle_request(|| self.client.post(&url).json_body(body)?.send());
        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn update_channel(
        &self,
        id: &str,
        body: ChannelRequest,
    ) -> Result<ChannelResponse, String> {
        let url = format!("/channels/{}", id);
        let response = self.handle_request(|| self.client.patch(&url).json_body(body)?.send());
        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn delete_channel(&self, id: &str) -> Result<(), String> {
        let url = format!("/channels/{}", id);
        let response = self.handle_request(|| self.client.delete(&url).send());
        self.handle_response(response).map(|_| ())
    }

    fn handle_request<F>(&self, f: F) -> Response
    where
        F: FnOnce() -> Result<Response, String>,
    {
        let result = f();

        match result {
            Err(message) => panic!("Error while sending Discord request : {}", message),
            Ok(response) => response,
        }
    }

    fn handle_response(&self, response: Response) -> Result<Response, String> {
        match response.status {
            StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(response),
            StatusCode::UNAUTHORIZED => Err("Invalid credentials. Make sure that the bot token exsits and has the right value.".into()),
            StatusCode::FORBIDDEN => {
                Err("Insufficient permissions. The bot is either not in the right guild, does not have the Manage Role permission or has lower permissions than the objects it wants to modify or delete.".into())
            },
            StatusCode::BAD_REQUEST => Err(format!("Invalid request. This should not happen... Make sure to file an issue if persistent. Error : {}", response.text_body())),
            _ => Err(format!(
                "Unhandled Discord response status {}. This issue is temporary, make sure that Discord's APIs are up and running. Make sure to file an issue if persistent. Error : {}",
                response.status,
                response.text_body()
            )),
        }
    }
}
