use colored::Colorize;
use disma::{
    api::{params::guild::GuildParams, ApplyChangesUseCase, ListChangesUseCase},
    core::changes::Change,
};
use std::{path::Path, sync::Arc};

use crate::{
    infra::diff::formatter::DiffFormaterRef,
    utils::{
        input::{abort, ask_user_confirmation},
        io::Deserializer,
    },
};

pub struct ApplyChanges {
    list_changes: Arc<ListChangesUseCase>,
    apply_changes: Arc<ApplyChangesUseCase>,
    deserializer: Arc<Deserializer>,
    formatter: DiffFormaterRef,
}

impl ApplyChanges {
    pub fn new(
        list_changes: Arc<ListChangesUseCase>,
        apply_changes: Arc<ApplyChangesUseCase>,
        deserializer: Arc<Deserializer>,
        formatter: DiffFormaterRef,
    ) -> Self {
        Self {
            list_changes,
            apply_changes,
            deserializer,
            formatter,
        }
    }

    pub fn run(&self, guild_id: &str, file: &str, dry_run: bool, force: bool) {
        let file_path = Path::new(file);

        println!();
        println!(
            "{}",
            format!("➜ 🛠️  Loading guild config from '{}'...", &file).bold()
        );
        let guild_params = self.deserializer.deserialize::<GuildParams>(file_path);

        println!("{}", "➜ 🔎 Looking for changes...".bold());
        let changes = self.list_changes.execute(guild_id, guild_params.clone());

        if changes.is_empty() {
            println!("{}", "➜ ✨ No change to be applied.".bold());
            return;
        }

        println!("{}", "➜ 📜 Found the following changes :".bold());

        for change in changes {
            match change {
                Change::Create(entity, name) => {
                    println!("\n● 🆕 Adding {:?} {}", entity, name.bold().on_black())
                }
                Change::Delete(entity, name) => {
                    println!("\n● 🗑️  Removing {:?} {}", entity, name.bold().on_black())
                }
                Change::Update(entity, name, diffs) => {
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

        println!("{}", "➜ 🚀 Applying changes...\n".bold());
        self.apply_changes.execute(guild_id, guild_params);
    }
}
