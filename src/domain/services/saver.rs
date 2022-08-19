use std::fs;

use crate::{domain::guild::ExistingGuild, infra::config::GuildConfig};

pub struct ExistingGuildSaver {
    file_path: String,
}

impl ExistingGuildSaver {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn save_existing_guild(&self, guild: &ExistingGuild) {
        let config = GuildConfig::from(guild);
        let file_content = serde_json::to_string_pretty(&config).unwrap();

        fs::write(&self.file_path, format!("{}\n", file_content)).unwrap();
    }
}
