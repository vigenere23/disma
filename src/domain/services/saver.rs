use std::{fs, path::Path};

use crate::{
    domain::guild::ExistingGuild,
    infra::config::GuildConfig,
    utils::input::{abort, ask_user_confirmation},
};

pub struct ExistingGuildSaver {}

impl ExistingGuildSaver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn save_existing_guild(&self, file_path: &str, guild: &ExistingGuild, force: bool) {
        let config = GuildConfig::from(guild);
        let file_content = serde_json::to_string_pretty(&config).unwrap();

        println!("\n💾 Saving current guild config to '{}'...", &file_path);

        if !force && Path::new(&file_path).exists() {
            println!("A file named '{}' already exists.", &file_path);

            if !ask_user_confirmation() {
                abort();
            }
        }

        fs::write(&file_path, format!("{}\n", file_content)).unwrap();

        println!("\n✨ DONE.");
    }
}
