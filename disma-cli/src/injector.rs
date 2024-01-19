use std::sync::Arc;

use disma::{
    api::{ApplyChangesUseCase, ListChangesUseCase},
    core::{
        changes::{
            category::CategoryChangesService, channel::ChannelChangesService,
            role::RoleChangesService,
        },
        events::ChangeEventListenerRef,
    },
    guild::{GuildCommander, GuildQuerier},
    impls::discord::{api::DiscordApi, HttpGuildCommander, HttpGuildQuerier},
};

use crate::{
    commands::{
        apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
        save_guild::SaveExistingGuild,
    },
    infra::diff::{
        event::CliChangeEventListener,
        formatter::{DiffFormater, DiffFormaterRef},
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

impl Get<ChangeEventListenerRef> for Injector {
    fn get(&self) -> ChangeEventListenerRef {
        Arc::from(CliChangeEventListener {})
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
        Arc::from(ApplyChanges::new(
            self.get(),
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

impl Get<Arc<ListChangesUseCase>> for Injector {
    fn get(&self) -> Arc<ListChangesUseCase> {
        Arc::from(ListChangesUseCase::new(
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        ))
    }
}

impl Get<Arc<ApplyChangesUseCase>> for Injector {
    fn get(&self) -> Arc<ApplyChangesUseCase> {
        Arc::from(ApplyChangesUseCase::new(
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
            self.get(),
        ))
    }
}

impl Get<Arc<RoleChangesService>> for Injector {
    fn get(&self) -> Arc<RoleChangesService> {
        Arc::from(RoleChangesService {})
    }
}

impl Get<Arc<CategoryChangesService>> for Injector {
    fn get(&self) -> Arc<CategoryChangesService> {
        Arc::from(CategoryChangesService {})
    }
}

impl Get<Arc<ChannelChangesService>> for Injector {
    fn get(&self) -> Arc<ChannelChangesService> {
        Arc::from(ChannelChangesService {})
    }
}
