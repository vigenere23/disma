mod client;
mod domain;
mod utils;

use std::env;

use crate::{client::api::DiscordApi, domain::guild::GuildRepo};

fn main() {
    let repo = DiscordApi::new(
        env::var("DAC_DISCORD_TOKEN").expect("Missing env variable 'DISCORD_TOKEN'."),
        "969728902891184239".to_string(),
    );

    let guild = repo.guild();

    println!("{:?}", guild);
}
