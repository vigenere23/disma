use std::{env, sync::Arc};

use crate::{
    domain::{
        guild::GuildQuerier,
        services::{
            diff::DiffCalculator, executor::CommandsExecutor, loader::AwaitingGuildLoader,
            saver::ExistingGuildSaver,
        },
    },
    infra::discord::{
        api::DiscordApi,
        client::{DiscordClient, DiscordGuildClient},
    },
    utils::{
        injector::Get,
        io::{Deserializer, Serializer},
    },
};

pub struct MainInjector {
    guild_id: Option<String>,
}

impl MainInjector {
    pub fn new(guild_id: Option<String>) -> Self {
        Self { guild_id }
    }
}

impl Get<Arc<DiscordApi>> for MainInjector {
    fn get(&self) -> Arc<DiscordApi> {
        let bot_token = env::var("DAC_DISCORD_BOT_TOKEN")
            .expect("Missing env variable 'DAC_DISCORD_BOT_TOKEN'.");

        Arc::from(DiscordApi::from_bot(&bot_token))
    }
}

impl Get<Arc<DiscordClient>> for MainInjector {
    fn get(&self) -> Arc<DiscordClient> {
        Arc::from(DiscordClient::new(self.get()))
    }
}

impl Get<Arc<DiscordGuildClient>> for MainInjector {
    fn get(&self) -> Arc<DiscordGuildClient> {
        let guild_id = self.guild_id.clone().expect("Missing guild id.");
        Arc::from(DiscordGuildClient::new(self.get(), &guild_id))
    }
}

impl Get<Arc<DiffCalculator>> for MainInjector {
    fn get(&self) -> Arc<DiffCalculator> {
        let discord_guild: Arc<DiscordGuildClient> = self.get();
        Arc::from(DiffCalculator::new(discord_guild))
    }
}

impl Get<Arc<CommandsExecutor>> for MainInjector {
    fn get(&self) -> Arc<CommandsExecutor> {
        Arc::from(CommandsExecutor())
    }
}

impl Get<Arc<ExistingGuildSaver>> for MainInjector {
    fn get(&self) -> Arc<ExistingGuildSaver> {
        Arc::from(ExistingGuildSaver::new(self.get()))
    }
}

impl Get<Arc<AwaitingGuildLoader>> for MainInjector {
    fn get(&self) -> Arc<AwaitingGuildLoader> {
        Arc::from(AwaitingGuildLoader::new(self.get()))
    }
}

impl Get<Arc<Deserializer>> for MainInjector {
    fn get(&self) -> Arc<Deserializer> {
        Arc::from(Deserializer())
    }
}

impl Get<Arc<Serializer>> for MainInjector {
    fn get(&self) -> Arc<Serializer> {
        Arc::from(Serializer())
    }
}

impl Get<Arc<dyn GuildQuerier>> for MainInjector {
    fn get(&self) -> Arc<dyn GuildQuerier> {
        <Self as Get<Arc<DiscordClient>>>::get(self)
    }
}
