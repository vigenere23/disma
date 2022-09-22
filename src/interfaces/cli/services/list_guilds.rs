use std::sync::Arc;

use dac::domain::guild::GuildQuerier;

pub struct ListGuilds {
    guild_querier: Arc<dyn GuildQuerier>,
}

impl ListGuilds {
    pub fn new(guild_querier: Arc<dyn GuildQuerier>) -> Self {
        Self { guild_querier }
    }

    pub fn run(&self) {
        println!("Listing guilds...");

        let guilds = self.guild_querier.list_guilds();

        for guild in guilds.iter() {
            println!(" - [{}] {}", &guild.id, &guild.name)
        }
    }
}
