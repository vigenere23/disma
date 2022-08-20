use std::fs;

use crate::{domain::guild::AwaitingGuild, infra::config::GuildConfig};

pub struct AwaitingGuildLoader {}

impl AwaitingGuildLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_awaiting_guild(&self, file_path: &str) -> AwaitingGuild {
        let file_content = fs::read_to_string(&file_path).unwrap();
        let config: GuildConfig = serde_json::from_str(&file_content).unwrap();

        config.into()
    }
}
