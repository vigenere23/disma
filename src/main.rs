mod client;
mod utils;

use std::env;

use client::base::DiscordClient;

use crate::client::api::DiscordApi;

#[tokio::main]
async fn main() {
    let client = DiscordApi::new(
        env::var("DAC_DISCORD_TOKEN").expect("Missing env variable 'DISCORD_TOKEN'."),
        "969728902891184239".to_string(),
    );

    println!("{:?}", client.get_roles().await);
    println!("{:?}", client.get_channels().await);
}
