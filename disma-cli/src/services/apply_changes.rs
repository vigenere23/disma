use colored::Colorize;
use std::{path::Path, sync::Arc};

use crate::{
    infra::diff::formatter::DiffFormaterRef,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Deserializer,
    },
};
use disma::{changes::ChangesService, commands::CommandDescription, params::guild::GuildParams};

pub struct ApplyChanges {
    diff_service: Arc<ChangesService>,
    deserializer: Arc<Deserializer>,
    formatter: DiffFormaterRef,
}

impl ApplyChanges {
    pub fn new(
        diff_service: Arc<ChangesService>,
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
        println!(
            "{}",
            format!("🡲 🛠️  Loading guild config from '{}'...", &file).bold()
        );
        let guild_params = self.deserializer.deserialize::<GuildParams>(file_path);

        println!("{}", "🡲 🔎 Looking for changes...".bold());
        let diffs = self
            .diff_service
            .list_changes(guild_id, guild_params.clone());

        if diffs.is_empty() {
            println!("{}", "🡲 ✨ No change to be applied.".bold());
            return;
        }

        println!("{}", "🡲 📜 Found the following changes :".bold());

        for diff in diffs {
            match diff {
                CommandDescription::Create(entity, name) => {
                    println!("\n● 🆕 Adding {:?} {}", entity, name.bold().on_black())
                }
                CommandDescription::Delete(entity, name) => {
                    println!("\n● 🗑️  Removing {:?} {}", entity, name.bold().on_black())
                }
                CommandDescription::Update(entity, name, diffs) => {
                    println!(
                        "\n● 🔄 Updating {:?} {} with diffs:",
                        entity,
                        name.bold().on_black()
                    );
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

        println!("{}", "🡲 🚀 Applying changes...\n".bold());
        self.diff_service.apply_changes(guild_id, guild_params);
    }
}
