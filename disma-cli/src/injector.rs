use std::sync::Arc;

use disma::{
    diff::{
        differ::{GuildDiffer, GuildDifferRef},
        event::{DiffEventListenerRef, NullDiffEventListener},
    },
    diff_service::GuildDiffService,
    discord::{
        api::DiscordApi,
        client::{DiscordClient, DiscordGuildClient},
    },
    guild::{GuildCommander, GuildQuerier},
};

use crate::{
    infra::diff::formatter::{DiffFormater, DiffFormaterRef},
    services::{
        apply_diffs::ApplyDiffs, compile_config::CompileConfig, list_guilds::ListGuilds,
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

impl Get<GuildDifferRef> for Injector {
    fn get(&self) -> GuildDifferRef {
        Arc::from(GuildDiffer {})
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

impl Get<DiffEventListenerRef> for Injector {
    fn get(&self) -> DiffEventListenerRef {
        Arc::from(NullDiffEventListener {})
    }
}

impl Get<Arc<GuildDiffService>> for Injector {
    fn get(&self) -> Arc<GuildDiffService> {
        Arc::from(GuildDiffService::new(
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

impl Get<DiffFormaterRef> for Injector {
    fn get(&self) -> DiffFormaterRef {
        Arc::from(DiffFormater::new())
    }
}

impl Get<Arc<ApplyDiffs>> for Injector {
    fn get(&self) -> Arc<ApplyDiffs> {
        Arc::from(ApplyDiffs::new(self.get(), self.get(), self.get()))
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
