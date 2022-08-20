use std::{env, sync::Arc};

use crate::{
    application::{apply_changes::ApplyChanges, save_guild::SaveExistingGuild},
    domain::{
        guild::GuildQuerier,
        services::{
            diff::DiffCalculator, executor::CommandsExecutor, loader::AwaitingGuildLoader,
            saver::ExistingGuildSaver,
        },
    },
    infra::api::DiscordApi,
    utils::io::{Deserializer, Serializer},
};

pub struct Injector {}

impl Injector {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Get<T> {
    fn get(&self) -> T;
}

impl Get<Arc<DiscordApi>> for Injector {
    fn get(&self) -> Arc<DiscordApi> {
        let bot_token = env::var("DAC_DISCORD_BOT_TOKEN")
            .expect("Missing env variable 'DAC_DISCORD_BOT_TOKEN'.");
        let guild_id = env::var("DAC_GUILD_ID").expect("Missing env variable 'DAC_GUILD_ID'.");

        Arc::from(DiscordApi::from_bot(&bot_token, &guild_id))
    }
}

impl Get<Arc<DiffCalculator>> for Injector {
    fn get(&self) -> Arc<DiffCalculator> {
        let api: Arc<DiscordApi> = self.get();
        Arc::from(DiffCalculator::new(api))
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
        <Self as Get<Arc<DiscordApi>>>::get(self)
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
