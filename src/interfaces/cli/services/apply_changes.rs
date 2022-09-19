use std::{path::Path, sync::Arc};

use crate::{
    application::services::changes::ChangesService, domain::services::loader::AwaitingGuildLoader,
};

pub struct ApplyChanges {
    changes_service: Arc<ChangesService>,
    guild_loader: Arc<AwaitingGuildLoader>,
}

impl ApplyChanges {
    pub fn new(
        changes_service: Arc<ChangesService>,
        guild_loader: Arc<AwaitingGuildLoader>,
    ) -> Self {
        Self {
            changes_service,
            guild_loader,
        }
    }

    pub fn run(&self, guild_id: &str, file_path: &str, dry_run: bool, force: bool) {
        let awaiting_guild = self.guild_loader.load_awaiting_guild(Path::new(file_path));

        println!("\n🔎 Looking for changes in roles...");
        self.changes_service
            .apply_role_changes(guild_id, &awaiting_guild, dry_run, force);

        println!("\n🔎 Looking for changes in categories...");
        self.changes_service
            .apply_category_changes(guild_id, &awaiting_guild, dry_run, force);
    }
}
