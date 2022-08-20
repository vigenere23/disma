use std::{path::Path, sync::Arc};

use crate::domain::{guild::GuildQuerier, services::saver::ExistingGuildSaver};

pub struct SaveExistingGuild {
    guild_querier: Arc<dyn GuildQuerier>,
    guild_saver: Arc<ExistingGuildSaver>,
}

impl SaveExistingGuild {
    pub fn new(guild_querier: Arc<dyn GuildQuerier>, guild_saver: Arc<ExistingGuildSaver>) -> Self {
        Self {
            guild_querier,
            guild_saver,
        }
    }

    pub fn run(&self, file_path: &str, force: bool) {
        let guild = self.guild_querier.guild();
        self.guild_saver
            .save_existing_guild(Path::new(file_path), &guild, force);
    }
}
