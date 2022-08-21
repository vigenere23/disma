use std::{path::Path, sync::Arc};

use crate::{
    domain::guild::AwaitingGuild,
    infra::config::guild::{GuildConfig, GuildConfigAssembler},
    utils::io::Deserializer,
};

pub struct AwaitingGuildLoader {
    deserializer: Arc<Deserializer>,
    guild_assembler: Arc<GuildConfigAssembler>,
}

impl AwaitingGuildLoader {
    pub fn new(
        deserializer: Arc<Deserializer>,
        guild_assembler: Arc<GuildConfigAssembler>,
    ) -> Self {
        Self {
            deserializer,
            guild_assembler,
        }
    }

    pub fn load_awaiting_guild(&self, file_path: &Path) -> AwaitingGuild {
        println!(
            "Loading guild config from '{}'...",
            &file_path.as_os_str().to_str().unwrap()
        );

        let config: GuildConfig = self.deserializer.deserialize(file_path);

        self.guild_assembler.to_awaiting(&config)
    }
}
