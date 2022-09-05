use std::sync::Arc;

use crate::{
    domain::commands::GuildCommand,
    utils::input::{abort, ask_user_confirmation},
};

pub struct CommandsExecutor();

impl CommandsExecutor {
    pub fn execute_commands(
        &self,
        commands: Vec<Arc<dyn GuildCommand>>,
        dry_run: bool,
        force: bool,
    ) {
        if commands.is_empty() {
            println!("✨ No change to be applied.");
            return;
        }

        println!("📜 Changes to be applied :");

        for command in &commands {
            println!(" - {}", command.describe());
        }

        if dry_run {
            return;
        }

        if !force && !ask_user_confirmation() {
            abort();
        }

        println!("\n🚀 Applying changes...");

        for command in &commands {
            println!(" - {}", command.describe());
            command.execute();
        }

        println!("\n✨ DONE.");
    }
}
