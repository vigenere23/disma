use colored::Colorize;
use disma::{guild::GuildQuerier, permission::Permission};
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

        for guild in guilds.into_iter() {
            println!(
                " - [{}] {} ({} members)",
                guild.id, guild.name, guild.nb_members
            );

            if !guild.permissions.contains(Permission::MANAGE_ROLES) {
                println!(
                    "{}",
                    &"   ↳ ⚠️ Warning: missing permission MANAGE_ROLES."
                        .bold()
                        .yellow(),
                );
            }
            if !guild.permissions.contains(Permission::MANAGE_CHANNELS) {
                println!(
                    "{}",
                    &"   ↳ ⚠️ Warning: missing permission MANAGE_CHANNELS."
                        .bold()
                        .yellow()
                );
            }
        }
    }
}
