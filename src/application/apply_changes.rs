use std::{path::Path, sync::Arc};

use crate::domain::{
    guild::GuildQuerier,
    services::{diff::DiffCalculator, executor::CommandsExecutor, loader::AwaitingGuildLoader},
};

pub struct ApplyChanges {
    guild_querier: Arc<dyn GuildQuerier>,
    diff_calculator: Arc<DiffCalculator>,
    commands_executor: Arc<CommandsExecutor>,
    guild_loader: Arc<AwaitingGuildLoader>,
}

impl ApplyChanges {
    pub fn new(
        guild_querier: Arc<dyn GuildQuerier>,
        diff_calculator: Arc<DiffCalculator>,
        commands_executor: Arc<CommandsExecutor>,
        guild_loader: Arc<AwaitingGuildLoader>,
    ) -> Self {
        Self {
            guild_querier,
            diff_calculator,
            commands_executor,
            guild_loader,
        }
    }

    pub fn run(&self, file_path: &str, dry_run: bool, force: bool) {
        let awaiting_guild = self.guild_loader.load_awaiting_guild(Path::new(file_path));
        let existing_guild = self.guild_querier.guild();

        let commands = self
            .diff_calculator
            .create_commands(existing_guild, awaiting_guild);
        self.commands_executor
            .execute_commands(commands, dry_run, force);
    }
}
