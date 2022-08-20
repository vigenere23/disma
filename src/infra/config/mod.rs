use serde::{Deserialize, Serialize};

use crate::domain::{
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
};

use self::role::RoleConfig;

pub mod role;

#[derive(Serialize, Deserialize)]
pub struct GuildConfig {
    roles: Vec<RoleConfig>,
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles = guild.roles.items().iter().map(|role| role.into()).collect();

        Self { roles }
    }
}

impl From<&AwaitingGuild> for GuildConfig {
    fn from(guild: &AwaitingGuild) -> Self {
        let roles = guild.roles.items().iter().map(|role| role.into()).collect();

        Self { roles }
    }
}

impl Into<AwaitingGuild> for GuildConfig {
    fn into(self) -> AwaitingGuild {
        let roles: Vec<AwaitingRole> = self.roles.iter().map(|role| role.into()).collect();

        AwaitingGuild {
            roles: RolesList::from(&roles),
        }
    }
}
