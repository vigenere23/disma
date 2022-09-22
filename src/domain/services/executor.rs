use std::sync::Arc;

use crate::domain::{commands::GuildCommand, guild::GuildCommander};

pub trait CommandsExecutor {
    fn execute_commands(
        &self,
        commands: Vec<Arc<dyn GuildCommand>>,
        guild: Arc<dyn GuildCommander>,
        dry_run: bool,
        force: bool,
    );
}
