use colored::Colorize;
use std::{path::Path, sync::Arc};

use crate::{
    infra::{config::guild::GuildConfig, diff::formatter::DiffFormaterRef},
    utils::{
        input::{abort, ask_user_confirmation},
        io::Deserializer,
    },
};
use disma::{changes::ChangesService, commands::CommandDescription};

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
            format!("š”² š ļø  Loading guild config from '{}'...", &file).bold()
        );
        let config = self.deserializer.deserialize::<GuildConfig>(file_path);
        let awaiting_guild = config.into();

        println!("{}", "š”² š Looking for changes...".bold());
        let diffs = self.diff_service.list_changes(guild_id, &awaiting_guild);

        if diffs.is_empty() {
            println!("{}", "š”² āØ No change to be applied.".bold());
            return;
        }

        println!("{}", "š”² š Found the following changes :".bold());

        for diff in diffs {
            match diff {
                CommandDescription::Create(entity, name) => {
                    println!("\nā š Adding {:?} {}", entity, name.bold().on_black())
                }
                CommandDescription::Delete(entity, name) => {
                    println!("\nā šļø  Removing {:?} {}", entity, name.bold().on_black())
                }
                CommandDescription::Update(entity, name, diffs) => {
                    println!(
                        "\nā š Updating {:?} {} with diffs:",
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

        println!("{}", "š”² š Applying changes...\n".bold());
        self.diff_service.apply_changes(guild_id, &awaiting_guild);
    }
}
