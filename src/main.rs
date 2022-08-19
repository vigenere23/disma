#![allow(dead_code)]

mod domain;
mod infra;
mod utils;

use std::{env, sync::Arc};

use domain::services::{loader::AwaitingGuildLoader, saver::ExistingGuildSaver};

use crate::{
    domain::guild::{AwaitingGuild, GuildQuerier},
    domain::services::{diff::DiffCalculator, executor::CommandsExecutor},
    infra::api::DiscordApi,
};

fn main() {
    apply_changes();
}

fn load_guild() -> AwaitingGuild {
    let loader = AwaitingGuildLoader {};

    loader.load_awaiting_guild()
}

fn save_guild() {
    let api = api();
    let saver = ExistingGuildSaver {};

    let existing_guild = api.guild();

    saver.save_existing_guild(&existing_guild);
}

fn apply_changes() {
    let api = api();
    let diff_calculator = DiffCalculator::new(api.clone());
    let commands_executor = CommandsExecutor {};

    let awaiting_guild = load_guild();
    let existing_guild = api.guild();

    let commands = diff_calculator.create_commands(existing_guild, awaiting_guild);
    commands_executor.execute_commands(commands, false, false);
}

fn display_changes() {
    let api = api();
    let diff_calculator = DiffCalculator::new(api.clone());
    let commands_executor = CommandsExecutor {};

    let awaiting_guild = load_guild();
    let existing_guild = api.guild();

    let commands = diff_calculator.create_commands(existing_guild, awaiting_guild);
    commands_executor.execute_commands(commands, true, false);
}

fn api() -> Arc<DiscordApi> {
    Arc::from(DiscordApi::from_bot(
        env::var("DAC_DISCORD_TOKEN").expect("Missing env variable 'DISCORD_TOKEN'."),
        "969728902891184239".to_string(),
    ))
}
