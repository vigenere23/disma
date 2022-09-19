use std::{path::Path, sync::Arc};

use crate::{
    application::config::guild::GuildConfig, domain::guild::AwaitingGuild, utils::io::Deserializer,
};

// TODO This should be an interface (FileAwaitingGuildLoader)
pub struct AwaitingGuildLoader {
    deserializer: Arc<Deserializer>,
}

impl AwaitingGuildLoader {
    pub fn new(deserializer: Arc<Deserializer>) -> Self {
        Self { deserializer }
    }

    pub fn load_awaiting_guild(&self, file_path: &Path) -> AwaitingGuild {
        println!(
            "🛠️  Loading guild config from '{}'...",
            &file_path.as_os_str().to_str().unwrap()
        );

        let config: GuildConfig = self.deserializer.deserialize(file_path);

        config.into()
    }
}
