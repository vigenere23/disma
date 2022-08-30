use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::{
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
};

use super::{
    category::CategoryConfig,
    role::{RoleConfig, RoleConfigAssembler},
};

#[derive(Serialize, Deserialize)]
pub struct GuildConfig {
    roles: Vec<RoleConfig>,
    categories: Vec<CategoryConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplatesConfig {
    roles: Option<Vec<RoleConfig>>,
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles = guild.roles.items().iter().map(|role| role.into()).collect();

        let categories = guild
            .categories
            .iter()
            .map(|category| CategoryConfig::from(category))
            .collect();

        Self { roles, categories }
    }
}

pub struct GuildConfigAssembler {
    role_assembler: Arc<RoleConfigAssembler>,
}

impl GuildConfigAssembler {
    pub fn new(role_assembler: Arc<RoleConfigAssembler>) -> Self {
        Self { role_assembler }
    }

    pub fn to_awaiting(&self, config: &GuildConfig) -> AwaitingGuild {
        let roles: Vec<AwaitingRole> = config
            .roles
            .iter()
            .map(|role_config| self.role_assembler.to_awaiting(role_config))
            .collect();

        AwaitingGuild {
            roles: RolesList::from(roles),
        }
    }
}
