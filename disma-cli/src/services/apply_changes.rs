use std::{path::Path, sync::Arc};

use disma::diff::GuildDiffService;

use crate::{
    infra::config::guild::GuildConfig,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Deserializer,
    },
};

pub struct ApplyChanges {
    changes_service: Arc<GuildDiffService>,
    deserializer: Arc<Deserializer>,
}

impl ApplyChanges {
    pub fn new(changes_service: Arc<GuildDiffService>, deserializer: Arc<Deserializer>) -> Self {
        Self {
            changes_service,
            deserializer,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, dry_run: bool, force: bool) {
        let file_path = Path::new(file);

        println!("🛠️  Loading guild config from '{}'...", &file);
        let config = self.deserializer.deserialize::<GuildConfig>(file_path);
        let awaiting_guild = config.into();

        println!("\n🔎 Looking for changes...");
        let diffs = self.changes_service.list_diffs(guild_id, &awaiting_guild);

        if diffs.is_empty() {
            println!("✨ No change to be applied.");
            return;
        }

        println!("\n📜 Found the following changes :");
        for diff in diffs {
            println!(" - {}", &diff.summary);
        }

        if dry_run {
            return;
        }

        if !force && !ask_user_confirmation("Ready to apply?") {
            abort();
        }

        println!("\n🚀 Applying changes...");
        self.changes_service.apply_diffs(guild_id, &awaiting_guild);
    }
}
