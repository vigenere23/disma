use std::{path::Path, sync::Arc};

use crate::interfaces::cli::{infra::config::guild::GuildConfig, utils::io::Deserializer};
use dac::application::changes::ChangesService;

pub struct ApplyChanges {
    changes_service: Arc<ChangesService>,
    deserializer: Arc<Deserializer>,
}

impl ApplyChanges {
    pub fn new(changes_service: Arc<ChangesService>, deserializer: Arc<Deserializer>) -> Self {
        Self {
            changes_service,
            deserializer,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, dry_run: bool, force: bool) {
        let file_path = Path::new(file);
        println!("🛠️  Loading guild config from '{}'...", &file);

        let config: GuildConfig = self.deserializer.deserialize(file_path);

        let awaiting_guild = config.into();

        println!("\n🔎 Looking for changes in roles...");
        self.changes_service
            .apply_role_changes(guild_id, &awaiting_guild, dry_run, force);

        println!("\n🔎 Looking for changes in categories...");
        self.changes_service
            .apply_category_changes(guild_id, &awaiting_guild, dry_run, force);
    }
}
