use serde::{Deserialize, Serialize};

use dac::domain::{
    category::{AwaitingCategory, CategoriesList},
    guild::{AwaitingGuild, ExistingGuild},
    role::{AwaitingRole, RolesList},
};

use super::{category::CategoryConfig, role::RoleConfig};

#[derive(Serialize, Deserialize)]
pub struct GuildConfig {
    roles: Vec<RoleConfig>,
    categories: Vec<CategoryConfig>,
}

impl From<&ExistingGuild> for GuildConfig {
    fn from(guild: &ExistingGuild) -> Self {
        let roles = guild.roles.items().iter().map(|role| role.into()).collect();

        let categories = guild
            .categories
            .items()
            .iter()
            .map(CategoryConfig::from)
            .collect();

        Self { roles, categories }
    }
}

impl Into<AwaitingGuild> for GuildConfig {
    fn into(self) -> AwaitingGuild {
        let roles: Vec<AwaitingRole> = self
            .roles
            .into_iter()
            .map(|role_config| role_config.into())
            .collect();

        let roles_list = RolesList::from(roles);

        let categories: Vec<AwaitingCategory> = self
            .categories
            .into_iter()
            .map(|category| category.into(&roles_list))
            .collect();

        AwaitingGuild {
            roles: roles_list,
            categories: CategoriesList::from(categories),
        }
    }
}
