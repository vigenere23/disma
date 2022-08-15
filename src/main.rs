mod domain;
mod infra;
mod utils;

use std::{env, sync::Arc};

use crate::{
    domain::{
        diff::DiffCalculator,
        guild::{AwaitingGuild, GuildQuerier},
        role::{AwaitingRole, AwaitingRolesList},
    },
    infra::api::DiscordApi,
};

fn main() {
    let api = Arc::from(DiscordApi::new(
        env::var("DAC_DISCORD_TOKEN").expect("Missing env variable 'DISCORD_TOKEN'."),
        "969728902891184239".to_string(),
    ));

    let diff_calculator = DiffCalculator::new(api.clone());

    let existing_guild = api.guild();
    let awaiting_guild = AwaitingGuild {
        roles: AwaitingRolesList::new(Vec::from([
            AwaitingRole {
                name: String::from("test1"),
                is_mentionalbe: true,
                show_in_sidebar: true,
            },
            AwaitingRole {
                name: String::from("test2"),
                is_mentionalbe: false,
                show_in_sidebar: false,
            },
        ])),
    };

    let commands = diff_calculator.create_commands(existing_guild, awaiting_guild);

    println!("\nCommands to be executed :");
    for command in commands.into_iter() {
        println!(" - {}", command.describe());
    }
}
