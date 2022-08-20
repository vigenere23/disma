use std::{path::Path, sync::Arc};

use crate::{
    domain::guild::ExistingGuild,
    infra::config::GuildConfig,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Serializer,
    },
};

pub struct ExistingGuildSaver {
    serializer: Arc<Serializer>,
}

impl ExistingGuildSaver {
    pub fn new(serializer: Arc<Serializer>) -> Self {
        Self { serializer }
    }

    pub fn save_existing_guild(&self, file_path: &Path, guild: &ExistingGuild, force: bool) {
        let config = GuildConfig::from(guild);
        let filename = file_path.as_os_str().to_str().unwrap();

        println!("\n💾 Saving current guild config to '{}'...", filename);

        if !force && file_path.exists() {
            println!("A file named '{}' already exists.", filename);

            if !ask_user_confirmation() {
                abort();
            }
        }

        self.serializer.serialize(&config, file_path);

        println!("\n✨ DONE.");
    }
}
