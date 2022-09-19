use std::sync::Arc;

use crate::domain::{
    guild::{AwaitingGuild, GuildQuerier},
    services::{diff::DiffCalculator, executor::CommandsExecutor},
};

pub struct ChangesService {
    guild_querier: Arc<dyn GuildQuerier>,
    diff_calculator: Arc<DiffCalculator>,
    commands_executor: Arc<CommandsExecutor>,
}

impl ChangesService {
    pub fn new(
        guild_querier: Arc<dyn GuildQuerier>,
        diff_calculator: Arc<DiffCalculator>,
        commands_executor: Arc<CommandsExecutor>,
    ) -> Self {
        Self {
            guild_querier,
            diff_calculator,
            commands_executor,
        }
    }

    pub fn apply_role_changes(
        &self,
        guild_id: &str,
        awaiting_guild: &AwaitingGuild,
        dry_run: bool,
        force: bool,
    ) {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let commands = self
            .diff_calculator
            .create_role_commands(&existing_guild, awaiting_guild);
        self.commands_executor
            .execute_commands(commands, dry_run, force);
    }

    pub fn apply_category_changes(
        &self,
        guild_id: &str,
        awaiting_guild: &AwaitingGuild,
        dry_run: bool,
        force: bool,
    ) {
        let existing_guild = self.guild_querier.get_guild(guild_id);

        let commands = self
            .diff_calculator
            .create_category_commands(&existing_guild, awaiting_guild);
        self.commands_executor
            .execute_commands(commands, dry_run, force);
    }
}
