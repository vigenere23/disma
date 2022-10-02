use std::{path::Path, sync::Arc};

use crate::{
    infra::config::guild::GuildConfig,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Deserializer,
    },
};
use disma::diff::base::Diff;
use disma::diff_service::GuildDiffService;

pub struct ApplyDiffs {
    diff_service: Arc<GuildDiffService>,
    deserializer: Arc<Deserializer>,
}

impl ApplyDiffs {
    pub fn new(diff_service: Arc<GuildDiffService>, deserializer: Arc<Deserializer>) -> Self {
        Self {
            diff_service,
            deserializer,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, dry_run: bool, force: bool) {
        let file_path = Path::new(file);

        println!("🛠️  Loading guild config from '{}'...", &file);
        let config = self.deserializer.deserialize::<GuildConfig>(file_path);
        let awaiting_guild = config.into();

        println!("\n🔎 Looking for changes...");
        let diffs = self.diff_service.list_diffs(guild_id, &awaiting_guild);

        if diffs.is_empty() {
            println!("✨ No change to be applied.");
            return;
        }

        println!("\n📜 Found the following changes :");
        for diff in diffs {
            match diff {
                // TODO use DiffPresenter (recursive, adds indents)
                Diff::Add(desc) => println!(" - 🆕 Creating {}", &desc),
                Diff::Update(desc, _) => println!(" - 🔄 Updating {}", &desc),
                Diff::Remove(desc) => println!(" - 🗑️  Deleting {}", &desc),
            };
        }

        if dry_run {
            return;
        }

        if !force && !ask_user_confirmation("Ready to apply?") {
            abort();
        }

        println!("\n🚀 Applying changes...");
        self.diff_service.apply_diffs(guild_id, &awaiting_guild);
    }
}
