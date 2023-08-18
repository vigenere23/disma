use std::{error::Error, fmt::Display};

use crate::utils::http::{Client, HttpError, Request, Response};

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

#[derive(Debug)]
pub enum DiscordError {
    InvalidCredentials,
    InsuffiscientPermissions,
    InvalidRequest(String),
    ClientError(String),
    Unknown(u16, String),
}

impl Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCredentials => f.write_str("Invalid credentials. Make sure that the bot token exsits and has the right value."),
            Self::InsuffiscientPermissions => f.write_str("Insufficient permissions. The bot is either not in the right guild, does not have the Manage Role permission or has lower permissions than the objects it wants to modify or delete."),
            Self::InvalidRequest(description) => f.write_str(&format!("Invalid request. This should not happen... Make sure to file an issue if persistent. Error : {description}")),
            Self::Unknown(status, description) => f.write_str(&format!("Unhandled Discord response status {status}. This issue is temporary, make sure that Discord's APIs are up and running. Make sure to file an issue if persistent. Error : {description}")),
            Self::ClientError(description) => f.write_str(&format!("Invalid parameters. {description}"))
        }
    }
}

impl Error for DiscordError {}

impl DiscordApi {
    pub fn from_bot(bot_token: &str) -> DiscordApi {
        let client = Client::new()
            .base_url("https://discord.com/api/v9")
            .header(USER_AGENT, "")
            .header(AUTHORIZATION, &format!("Bot {bot_token}"));
        Self { client }
    }

    pub fn list_roles(&self, guild_id: &str) -> Result<Vec<RoleResponse>, DiscordError> {
        let url = format!("/guilds/{guild_id}/roles");
        let response = self.handle_http_error(self.client.clone().get(&url).send())?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn add_role(
        &self,
        guild_id: &str,
        body: RoleRequest,
    ) -> Result<RoleResponse, DiscordError> {
        let url = format!("/guilds/{guild_id}/roles");
        let request = self.handle_request(self.client.clone().post(&url).json_body(body))?;
        let response = self.handle_http_error(request.send())?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn update_role(
        &self,
        guild_id: &str,
        role_id: &str,
        body: RoleRequest,
    ) -> Result<RoleResponse, DiscordError> {
        let url = format!("/guilds/{guild_id}/roles/{role_id}");
        let request = self.handle_request(self.client.clone().patch(&url).json_body(body))?;
        let response = self.handle_http_error(request.send())?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn delete_role(&self, guild_id: &str, role_id: &str) -> Result<(), DiscordError> {
        let url = format!("/guilds/{guild_id}/roles/{role_id}");
        let response = self.handle_http_error(self.client.clone().delete(&url).send())?;
        self.handle_response(response).map(|_| ())
    }

    pub fn list_guilds(&self) -> Result<Vec<GuildResponse>, DiscordError> {
        let response = self.handle_http_error(
            self.client
                .clone()
                .get("/users/@me/guilds?with_counts=true")
                .send(),
        )?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn list_channels(&self, guild_id: &str) -> Result<Vec<ChannelResponse>, DiscordError> {
        let url = format!("/guilds/{guild_id}/channels");
        let response = self.handle_http_error(self.client.clone().get(&url).send())?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn add_channel(
        &self,
        guild_id: &str,
        body: ChannelRequest,
    ) -> Result<ChannelResponse, DiscordError> {
        let url = format!("/guilds/{guild_id}/channels");
        let request = self.handle_request(self.client.clone().post(&url).json_body(body))?;
        let response = self.handle_http_error(request.send())?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn update_channel(
        &self,
        id: &str,
        body: ChannelRequest,
    ) -> Result<ChannelResponse, DiscordError> {
        let url = format!("/channels/{id}");
        let request = self.handle_request(self.client.clone().patch(&url).json_body(body))?;
        let response = self.handle_http_error(request.send())?;

        self.handle_response(response)
            .map(|response| response.parsed_body().unwrap())
    }

    pub fn delete_channel(&self, id: &str) -> Result<(), DiscordError> {
        let url = format!("/channels/{id}");
        let response = self.handle_http_error(self.client.clone().delete(&url).send())?;

        self.handle_response(response).map(|_| ())
    }

    fn handle_request(&self, result: Result<Request, HttpError>) -> Result<Request, DiscordError> {
        result.map_err(|error| DiscordError::InvalidRequest(error.to_string()))
    }

    fn handle_http_error(
        &self,
        result: Result<Response, HttpError>,
    ) -> Result<Response, DiscordError> {
        result.map_err(|error| DiscordError::InvalidRequest(error.to_string()))
    }

    fn handle_response(&self, response: Response) -> Result<Response, DiscordError> {
        match response.status {
            StatusCode::OK
            | StatusCode::ACCEPTED
            | StatusCode::CREATED
            | StatusCode::NO_CONTENT => Ok(response),
            StatusCode::UNAUTHORIZED => Err(DiscordError::InvalidCredentials),
            StatusCode::FORBIDDEN => Err(DiscordError::InsuffiscientPermissions),
            StatusCode::BAD_REQUEST => Err(DiscordError::ClientError(response.text_body().into())),
            _ => Err(DiscordError::Unknown(
                response.status.as_u16(),
                response.text_body().into(),
            )),
        }
    }
}
