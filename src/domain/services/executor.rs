use std::{
    io::{self, Write},
    process::exit,
    sync::Arc,
};

use crate::domain::commands::GuildCommand;

pub struct CommandsExecutor();

impl CommandsExecutor {
    pub fn execute_commands(
        &self,
        commands: Vec<Arc<dyn GuildCommand>>,
        dry_run: bool,
        force: bool,
    ) {
        if commands.is_empty() {
            println!("\n✨ No change to be applied.");
            return;
        }

        println!("\n📜 Changes to be applied :");

        for command in &commands {
            println!(" - {}", command.describe());
        }

        if !dry_run {
            if !force && !self.confirmed() {
                println!("❌ CANCELED.");
                exit(1);
            }

            println!("\n🚀 Applying changes...");

            for command in &commands {
                println!(" - {}", command.describe());
                command.execute();
            }

            println!("\n✨ DONE.");
        }
    }

    fn confirmed(&self) -> bool {
        print!("\n❔ Do you want to proceeed? (y/N) ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("❌ Unable to read user input");

        input.trim().to_lowercase() == "y"
    }
}
