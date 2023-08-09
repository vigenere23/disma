use crate::params::{
    category::{CategoriesParamsList, CategoryParams},
    channel::{ChannelParams, ChannelsParamsList},
    guild::GuildParams,
    role::{RoleParams, RolesParamsList},
};

pub struct GuildParamsFixture {
    roles: RolesParamsList,
    categories: CategoriesParamsList,
    channels: ChannelsParamsList,
}

impl GuildParamsFixture {
    pub fn new() -> Self {
        Self {
            roles: RolesParamsList::default(),
            categories: CategoriesParamsList::default(),
            channels: ChannelsParamsList::default(),
        }
    }

    pub fn with_role(mut self, role: RoleParams) -> Self {
        self.roles.items.push(role);
        self
    }

    pub fn with_category(mut self, category: CategoryParams) -> Self {
        self.categories.items.push(category);
        self
    }

    pub fn with_channel(mut self, channel: ChannelParams) -> Self {
        self.channels.items.push(channel);
        self
    }

    pub fn build(self) -> GuildParams {
        GuildParams {
            roles: self.roles,
            categories: self.categories,
            channels: self.channels,
        }
    }
}