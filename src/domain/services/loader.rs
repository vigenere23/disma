use std::fs;

use crate::{domain::guild::AwaitingGuild, infra::config::GuildConfig};

pub struct AwaitingGuildLoader {
    file_path: String,
}

impl AwaitingGuildLoader {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn load_awaiting_guild(&self) -> AwaitingGuild {
        let file_content = fs::read_to_string(&self.file_path).unwrap();
        let config: GuildConfig = serde_json::from_str(&file_content).unwrap();

        config.into()
    }
}
