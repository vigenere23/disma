use std::{path::Path, sync::Arc};

use crate::domain::{
    guild::{AwaitingGuild, GuildQuerier},
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

    pub fn run(&self, guild_id: &str, file_path: &str, dry_run: bool, force: bool) {
        let awaiting_guild = self.guild_loader.load_awaiting_guild(Path::new(file_path));

        self.apply_role_changes(guild_id, &awaiting_guild, dry_run, force);
        self.apply_category_changes(guild_id, &awaiting_guild, dry_run, force);
    }

    fn apply_role_changes(
        &self,
        guild_id: &str,
        awaiting_guild: &AwaitingGuild,
        dry_run: bool,
        force: bool,
    ) {
        println!("\n🔎 Looking for changes in roles...");

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let commands = self
            .diff_calculator
            .create_role_commands(&existing_guild, awaiting_guild);
        self.commands_executor
            .execute_commands(commands, dry_run, force);
    }

    fn apply_category_changes(
        &self,
        guild_id: &str,
        awaiting_guild: &AwaitingGuild,
        dry_run: bool,
        force: bool,
    ) {
        println!("\n🔎 Looking for changes in categories...");

        let existing_guild = self.guild_querier.get_guild(guild_id);

        let commands = self
            .diff_calculator
            .create_category_commands(&existing_guild, awaiting_guild);
        self.commands_executor
            .execute_commands(commands, dry_run, force);
    }
}
