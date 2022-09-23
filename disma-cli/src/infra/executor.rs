use std::sync::Arc;

use crate::utils::input::{abort, ask_user_confirmation};
use disma::{commands::GuildCommand, executor::CommandsExecutor, guild::GuildCommander};

pub struct CliCommandsExecutor();

impl CommandsExecutor for CliCommandsExecutor {
    fn execute_commands(
        &self,
        commands: Vec<Arc<dyn GuildCommand>>,
        guild: Arc<dyn GuildCommander>,
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
            command.execute(guild.clone());
        }

        println!("\n✨ DONE.");
    }
}
