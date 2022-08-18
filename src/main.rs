mod domain;
mod infra;
mod utils;

use std::{env, sync::Arc};

use crate::{
    domain::{
        commands::{diff::DiffCalculator, executor::CommandsExecutor},
        guild::{AwaitingGuild, GuildQuerier},
        role::{AwaitingRole, AwaitingRolesList},
    },
    infra::api::DiscordApi,
};

fn main() {
    let api = Arc::from(DiscordApi::from_bot(
        env::var("DAC_DISCORD_TOKEN").expect("Missing env variable 'DISCORD_TOKEN'."),
        "969728902891184239".to_string(),
    ));

    let diff_calculator = DiffCalculator::new(api.clone());

    let commands_executor = CommandsExecutor {};

    let existing_guild = api.guild();
    let awaiting_guild = AwaitingGuild {
        roles: AwaitingRolesList::new(Vec::from([
            // AwaitingRole {
            //     name: String::from("test1"),
            //     permissions: String::from("1071698660929"),
            //     is_mentionalbe: true,
            //     show_in_sidebar: true,
            // },
            AwaitingRole {
                name: String::from("@everyone"),
                permissions: String::from("1071698660929"),
                is_mentionalbe: false,
                show_in_sidebar: false,
            },
            AwaitingRole {
                name: String::from("dac"),
                permissions: String::from("8"),
                is_mentionalbe: false,
                show_in_sidebar: false,
            },
        ])),
    };

    let commands = diff_calculator.create_commands(existing_guild, awaiting_guild);
    commands_executor.execute_commands(commands, false, false);
}
