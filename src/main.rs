mod domain;
mod infra;
mod utils;

use std::env;

use crate::{domain::guild::GuildRepo, infra::api::DiscordApi};

fn main() {
    let repo = DiscordApi::new(
        env::var("DAC_DISCORD_TOKEN").expect("Missing env variable 'DISCORD_TOKEN'."),
        "969728902891184239".to_string(),
    );

    let guild = repo.guild();

    println!("{:?}", guild);
}
