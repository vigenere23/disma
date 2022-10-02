use std::{path::Path, sync::Arc};

use crate::{
    infra::{config::guild::GuildConfig, diff::formatter::DiffFormaterRef},
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
    formatter: DiffFormaterRef,
}

impl ApplyDiffs {
    pub fn new(
        diff_service: Arc<GuildDiffService>,
        deserializer: Arc<Deserializer>,
        formatter: DiffFormaterRef,
    ) -> Self {
        Self {
            diff_service,
            deserializer,
            formatter,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, dry_run: bool, force: bool) {
        let file_path = Path::new(file);
        println!();

        println!("🡲 🛠️  Loading guild config from '{}'...", &file);
        let config = self.deserializer.deserialize::<GuildConfig>(file_path);
        let awaiting_guild = config.into();

        println!("🡲 🔎 Looking for changes...");
        let diffs = self.diff_service.list_diffs(guild_id, &awaiting_guild);

        if diffs.is_empty() {
            println!("✨ No change to be applied.");
            return;
        }

        println!("🡲 📜 Found the following changes :\n");

        for diff in diffs {
            match diff {
                Diff::Add(desc) => println!("● 🆕 Adding {}", desc),
                Diff::Remove(desc) => println!("● 🗑️  Removing {}", desc),
                Diff::Update(desc, diffs) => {
                    println!("● 🔄 Updating {} with diffs:", desc);
                    for diff in diffs {
                        print!("{}", self.formatter.format(&diff));
                    }
                }
            }
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
