use std::{path::Path, sync::Arc};

use crate::{
    infra::config::guild::GuildConfig,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Serializer,
    },
};
use disma::guild::GuildQuerier;

pub struct SaveExistingGuild {
    guild_querier: Arc<dyn GuildQuerier>,
    serializer: Arc<Serializer>,
}

impl SaveExistingGuild {
    pub fn new(guild_querier: Arc<dyn GuildQuerier>, serializer: Arc<Serializer>) -> Self {
        Self {
            guild_querier,
            serializer,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, force: bool) {
        let guild = self.guild_querier.get_guild(guild_id);

        let config = GuildConfig::from(&guild);
        let file_path = Path::new(file);

        println!("\n💾 Saving current guild config to '{}'...", file);

        if !force && file_path.exists() {
            println!("A file named '{}' already exists.", file);

            if !ask_user_confirmation("Do you still want to proceeed?") {
                abort();
            }
        }

        self.serializer.serialize(&config, file_path);

        println!("\n✨ DONE.");
    }
}
