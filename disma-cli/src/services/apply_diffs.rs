use colored::Colorize;
use std::{path::Path, sync::Arc};

use crate::{
    infra::{config::guild::GuildConfig, diff::formatter::DiffFormaterRef},
    utils::{
        input::{abort, ask_user_confirmation},
        io::Deserializer,
    },
};
use disma::diff::base::EntityChange;
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
        println!();
        let file_path = Path::new(file);

        println!(
            "{}",
            format!("🡲 🛠️  Loading guild config from '{}'...", &file).bold()
        );
        let config = self.deserializer.deserialize::<GuildConfig>(file_path);
        let awaiting_guild = config.into();

        println!("{}", "🡲 🔎 Looking for changes...".bold());
        let diffs = self.diff_service.list_diffs(guild_id, &awaiting_guild);

        if diffs.is_empty() {
            println!("{}", "🡲 ✨ No change to be applied.".bold());
            return;
        }

        println!("{}", "🡲 📜 Found the following changes :".bold());

        for diff in diffs {
            match diff {
                EntityChange::Create(entity, name) => {
                    println!("\n● 🆕 Adding {:?} {name}", entity)
                }
                EntityChange::Delete(entity, name) => {
                    println!("\n● 🗑️  Removing {:?} {name}", entity)
                }
                EntityChange::Update(entity, name, diffs) => {
                    println!("\n● 🔄 Updating {:?} {name} with diffs:", entity);
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

        println!("{}", "🡲 🚀 Applying changes...".bold());
        // TODO BUG: changes are not applied...
        self.diff_service.apply_diffs(guild_id, &awaiting_guild);
    }
}
