use std::sync::Arc;

use super::GuildCommand;

pub struct CommandsExecutor {}

impl CommandsExecutor {
    pub fn execute_commands(
        &self,
        commands: Vec<Arc<dyn GuildCommand>>,
        dry_run: bool,
        force: bool,
    ) {
        if commands.is_empty() {
            println!("\nNo commands to be executed.");
            return;
        }

        println!("\nCommands to be executed :");

        for command in &commands {
            println!(" - {}", command.describe());
        }

        if !dry_run {
            if !force {
                // TODO ask before executing
            }

            for command in &commands {
                command.execute();
            }
        }
    }
}
