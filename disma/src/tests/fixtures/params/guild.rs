use crate::api::params::{
    category::{CategoriesParamsList, CategoryParams, CategoryParamsExtraItemsStrategy},
    channel::{ChannelParams, ChannelParamsExtraItemsStrategy, ChannelsParamsList},
    guild::GuildParams,
    role::{RoleParams, RoleParamsExtraItemsStrategy, RolesParamsList},
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

    pub fn remove_extra_roles(mut self) -> Self {
        self.roles.extra_items = RoleParamsExtraItemsStrategy::Remove;
        self
    }

    pub fn with_category(mut self, category: CategoryParams) -> Self {
        self.categories.items.push(category);
        self
    }

    pub fn remove_extra_categories(mut self) -> Self {
        self.categories.extra_items = CategoryParamsExtraItemsStrategy::Remove;
        self
    }

    pub fn with_channel(mut self, channel: ChannelParams) -> Self {
        self.channels.items.push(channel);
        self
    }

    pub fn remove_extra_channels(mut self) -> Self {
        self.channels.extra_items = ChannelParamsExtraItemsStrategy::Remove;
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
