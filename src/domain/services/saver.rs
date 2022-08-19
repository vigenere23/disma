use std::fs;

use crate::{domain::guild::ExistingGuild, infra::config::GuildConfig};

pub struct ExistingGuildSaver {}

impl ExistingGuildSaver {
    pub fn save_existing_guild(&self, guild: &ExistingGuild) {
        let config = GuildConfig::from(guild);
        let file_content = serde_json::to_string_pretty(&config).unwrap();

        fs::write("config.json", format!("{}\n", file_content)).unwrap();
    }
}
