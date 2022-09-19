use std::sync::Arc;

use crate::{
    application::services::changes::ChangesService,
    domain::{
        guild::GuildQuerier,
        services::{loader::AwaitingGuildLoader, saver::ExistingGuildSaver},
    },
    injector::MainInjector,
    interfaces::cli::services::{
        apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
        save_guild::SaveExistingGuild,
    },
    utils::{
        injector::Get,
        io::{Deserializer, Serializer},
    },
};

pub struct CliInjector {
    pub base: MainInjector,
}

impl CliInjector {
    pub fn new(base: MainInjector) -> Self {
        Self { base }
    }
}

impl Get<Arc<ExistingGuildSaver>> for CliInjector {
    fn get(&self) -> Arc<ExistingGuildSaver> {
        Arc::from(ExistingGuildSaver::new(self.get()))
    }
}

impl Get<Arc<AwaitingGuildLoader>> for CliInjector {
    fn get(&self) -> Arc<AwaitingGuildLoader> {
        Arc::from(AwaitingGuildLoader::new(self.get()))
    }
}

impl Get<Arc<Deserializer>> for CliInjector {
    fn get(&self) -> Arc<Deserializer> {
        Arc::from(Deserializer())
    }
}

impl Get<Arc<Serializer>> for CliInjector {
    fn get(&self) -> Arc<Serializer> {
        Arc::from(Serializer())
    }
}

impl Get<Arc<ApplyChanges>> for CliInjector {
    fn get(&self) -> Arc<ApplyChanges> {
        Arc::from(ApplyChanges::new(self.get(), self.get()))
    }
}

impl Get<Arc<SaveExistingGuild>> for CliInjector {
    fn get(&self) -> Arc<SaveExistingGuild> {
        let querier: Arc<dyn GuildQuerier> = self.base.get();

        Arc::from(SaveExistingGuild::new(querier, self.get()))
    }
}

impl Get<Arc<ListGuilds>> for CliInjector {
    fn get(&self) -> Arc<ListGuilds> {
        Arc::from(ListGuilds::new(self.base.get()))
    }
}

impl Get<Arc<CompileConfig>> for CliInjector {
    fn get(&self) -> Arc<CompileConfig> {
        Arc::from(CompileConfig::new(self.get()))
    }
}

impl Get<Arc<ChangesService>> for CliInjector {
    fn get(&self) -> Arc<ChangesService> {
        Arc::from(ChangesService::new(
            self.base.get(),
            self.base.get(),
            self.base.get(),
        ))
    }
}
