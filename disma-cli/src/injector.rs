use std::sync::Arc;

use disma::{
    changes::ChangesService,
    commands::CommandEventListenerRef,
    guild::{GuildCommander, GuildQuerier},
    impls::discord::{api::DiscordApi, HttpGuildCommander, HttpGuildQuerier},
};

use crate::{
    infra::diff::{
        event::CliCommandEventListener,
        formatter::{DiffFormater, DiffFormaterRef},
    },
    services::{
        apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
        save_guild::SaveExistingGuild,
    },
    utils::{
        env::required_env,
        io::{Deserializer, Serializer},
    },
};

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
        let bot_token = required_env("DISCORD_BOT_TOKEN");
        Arc::from(DiscordApi::from_bot(&bot_token))
    }
}

impl Get<Arc<dyn GuildQuerier>> for Injector {
    fn get(&self) -> Arc<dyn GuildQuerier> {
        Arc::from(HttpGuildQuerier::new(self.get()))
    }
}

impl Get<Arc<dyn GuildCommander>> for Injector {
    fn get(&self) -> Arc<dyn GuildCommander> {
        let guild_id = self.guild_id.clone().expect("Missing guild id.");
        Arc::from(HttpGuildCommander::new(self.get(), &guild_id))
    }
}

impl Get<CommandEventListenerRef> for Injector {
    fn get(&self) -> CommandEventListenerRef {
        Arc::from(CliCommandEventListener {})
    }
}

impl Get<Arc<ChangesService>> for Injector {
    fn get(&self) -> Arc<ChangesService> {
        Arc::from(ChangesService::new(self.get(), self.get(), self.get()))
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

impl Get<DiffFormaterRef> for Injector {
    fn get(&self) -> DiffFormaterRef {
        Arc::from(DiffFormater::new())
    }
}

impl Get<Arc<ApplyChanges>> for Injector {
    fn get(&self) -> Arc<ApplyChanges> {
        Arc::from(ApplyChanges::new(self.get(), self.get(), self.get()))
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
