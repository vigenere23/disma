use colored::Colorize;
use disma::guild::GuildQuerier;
use std::sync::Arc;

pub struct ListGuilds {
    guild_querier: Arc<dyn GuildQuerier>,
}

impl ListGuilds {
    pub fn new(guild_querier: Arc<dyn GuildQuerier>) -> Self {
        Self { guild_querier }
    }

    pub fn run(&self) {
        println!();
        println!("{}", "➜ ✅ Listing accessible guilds...".bold());

        let guilds = self.guild_querier.list_guilds();

        for guild in guilds.iter() {
            println!(" - [{}] {}", &guild.id, &guild.name)
        }
    }
}
