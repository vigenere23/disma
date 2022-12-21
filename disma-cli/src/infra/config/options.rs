use disma::guild::{
    AwaitingCategoriesOptions, AwaitingChannelsOptions, AwaitingGuildOptions, AwaitingRolesOptions,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct OptionsConfig {
    #[serde(default = "RolesOptionsConfig::default")]
    pub roles: RolesOptionsConfig,
    #[serde(default = "CategoriesOptionsConfig::default")]
    pub categories: CategoriesOptionsConfig,
    #[serde(default = "ChannelsOptionsConfig::default")]
    pub channels: ChannelsOptionsConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RolesOptionsConfig {
    #[serde(default = "bool::default")]
    pub allow_extra: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CategoriesOptionsConfig {
    #[serde(default = "bool::default")]
    pub allow_extra: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ChannelsOptionsConfig {
    #[serde(default = "bool::default")]
    pub allow_extra: bool,
}

impl Into<AwaitingGuildOptions> for OptionsConfig {
    fn into(self) -> AwaitingGuildOptions {
        AwaitingGuildOptions {
            roles: self.roles.into(),
            categories: self.categories.into(),
            channels: self.channels.into(),
        }
    }
}

impl Into<AwaitingRolesOptions> for RolesOptionsConfig {
    fn into(self) -> AwaitingRolesOptions {
        AwaitingRolesOptions {
            allow_extra: self.allow_extra,
        }
    }
}

impl Into<AwaitingCategoriesOptions> for CategoriesOptionsConfig {
    fn into(self) -> AwaitingCategoriesOptions {
        AwaitingCategoriesOptions {
            allow_extra: self.allow_extra,
        }
    }
}

impl Into<AwaitingChannelsOptions> for ChannelsOptionsConfig {
    fn into(self) -> AwaitingChannelsOptions {
        AwaitingChannelsOptions {
            allow_extra: self.allow_extra,
        }
    }
}
