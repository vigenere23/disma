use std::fs;

use crate::{domain::guild::ExistingGuild, infra::config::GuildConfig};

pub struct ExistingGuildSaver {}

impl ExistingGuildSaver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn save_existing_guild(&self, file_path: &str, guild: &ExistingGuild) {
        let config = GuildConfig::from(guild);
        let file_content = serde_json::to_string_pretty(&config).unwrap();

        println!("\n💾 Saving current guild config...");

        fs::write(&file_path, format!("{}\n", file_content)).unwrap();

        println!("\n✨ DONE.");
    }
}
