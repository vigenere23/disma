use std::{env, sync::Arc};

use dac::{
    application::changes::ChangesService,
    domain::{
        guild::{GuildCommander, GuildQuerier},
        services::{diff::DiffCalculator, executor::CommandsExecutor},
    },
    infra::discord::{
        api::DiscordApi,
        client::{DiscordClient, DiscordGuildClient},
    },
};

use crate::{
    interfaces::cli::services::{
        apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
        save_guild::SaveExistingGuild,
    },
    interfaces::cli::utils::io::{Deserializer, Serializer},
};

use super::infra::executor::CliCommandsExecutor;

pub trait Get<T> {
    fn get(&self) -> T;
}

pub struct Injector {
    guild_id: Option<String>,
}

impl Injector {
    pub fn new(guild_id: Option<String>) -> Self {
        Self { guild_id }
    }
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
        Arc::from(DiffCalculator {})
    }
}

impl Get<Arc<dyn CommandsExecutor>> for Injector {
    fn get(&self) -> Arc<dyn CommandsExecutor> {
        Arc::from(CliCommandsExecutor())
    }
}

impl Get<Arc<dyn GuildQuerier>> for Injector {
    fn get(&self) -> Arc<dyn GuildQuerier> {
        <Self as Get<Arc<DiscordClient>>>::get(self)
    }
}

impl Get<Arc<dyn GuildCommander>> for Injector {
    fn get(&self) -> Arc<dyn GuildCommander> {
        <Self as Get<Arc<DiscordGuildClient>>>::get(self)
    }
}

impl Get<Arc<ChangesService>> for Injector {
    fn get(&self) -> Arc<ChangesService> {
        Arc::from(ChangesService::new(
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        ))
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

impl Get<Arc<ApplyChanges>> for Injector {
    fn get(&self) -> Arc<ApplyChanges> {
        Arc::from(ApplyChanges::new(self.get(), self.get()))
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
