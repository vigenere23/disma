use std::{env, sync::Arc};

use crate::{
    application::{
        apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
        save_guild::SaveExistingGuild,
    },
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
    utils::io::{Deserializer, Serializer},
};

pub struct Injector {
    guild_id: Option<String>,
}

impl Injector {
    pub fn new(guild_id: Option<String>) -> Self {
        Self { guild_id }
    }
}

pub trait Get<T> {
    fn get(&self) -> T;
}

impl Get<Arc<DiscordApi>> for Injector {
    fn get(&self) -> Arc<DiscordApi> {
        let bot_token = env::var("DAC_DISCORD_BOT_TOKEN")
            .expect("Missing env variable 'DAC_DISCORD_BOT_TOKEN'.");

        Arc::from(DiscordApi::from_bot(&bot_token))
    }
}

impl Get<Arc<DiscordClient>> for Injector {
    fn get(&self) -> Arc<DiscordClient> {
        Arc::from(DiscordClient::new(self.get()))
    }
}

impl Get<Arc<DiscordGuildClient>> for Injector {
    fn get(&self) -> Arc<DiscordGuildClient> {
        let guild_id = self.guild_id.clone().expect("Missing guild id.");
        Arc::from(DiscordGuildClient::new(self.get(), &guild_id))
    }
}

impl Get<Arc<DiffCalculator>> for Injector {
    fn get(&self) -> Arc<DiffCalculator> {
        let discord_guild: Arc<DiscordGuildClient> = self.get();
        Arc::from(DiffCalculator::new(discord_guild))
    }
}

impl Get<Arc<CommandsExecutor>> for Injector {
    fn get(&self) -> Arc<CommandsExecutor> {
        Arc::from(CommandsExecutor())
    }
}

impl Get<Arc<ExistingGuildSaver>> for Injector {
    fn get(&self) -> Arc<ExistingGuildSaver> {
        Arc::from(ExistingGuildSaver::new(self.get()))
    }
}

impl Get<Arc<AwaitingGuildLoader>> for Injector {
    fn get(&self) -> Arc<AwaitingGuildLoader> {
        Arc::from(AwaitingGuildLoader::new(self.get()))
    }
}

impl Get<Arc<Deserializer>> for Injector {
    fn get(&self) -> Arc<Deserializer> {
        Arc::from(Deserializer())
    }
}

impl Get<Arc<Serializer>> for Injector {
    fn get(&self) -> Arc<Serializer> {
        Arc::from(Serializer())
    }
}

impl Get<Arc<dyn GuildQuerier>> for Injector {
    fn get(&self) -> Arc<dyn GuildQuerier> {
        <Self as Get<Arc<DiscordClient>>>::get(self)
    }
}

impl Get<Arc<ApplyChanges>> for Injector {
    fn get(&self) -> Arc<ApplyChanges> {
        let querier: Arc<dyn GuildQuerier> = self.get();

        Arc::from(ApplyChanges::new(
            querier,
            self.get(),
            self.get(),
            self.get(),
        ))
    }
}

impl Get<Arc<SaveExistingGuild>> for Injector {
    fn get(&self) -> Arc<SaveExistingGuild> {
        let querier: Arc<dyn GuildQuerier> = self.get();

        Arc::from(SaveExistingGuild::new(querier, self.get()))
    }
}

impl Get<Arc<ListGuilds>> for Injector {
    fn get(&self) -> Arc<ListGuilds> {
        Arc::from(ListGuilds::new(self.get()))
    }
}

impl Get<Arc<CompileConfig>> for Injector {
    fn get(&self) -> Arc<CompileConfig> {
        Arc::from(CompileConfig::new(self.get()))
    }
}
