use crate::params::{
    category::{CategoryParams, CategoryParamsExtraChannelsStrategy},
    permission::PermissionsOverwriteParams,
};

pub struct CategoryParamsFixture {
    name: String,
    permissions_overwrites: Vec<PermissionsOverwriteParams>,
    extra_channels: CategoryParamsExtraChannelsStrategy,
}

impl CategoryParamsFixture {
    pub fn new() -> Self {
        Self {
            name: "abc".to_string(),
            permissions_overwrites: Vec::new(),
            extra_channels: CategoryParamsExtraChannelsStrategy::default(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_permissions_overwrite(
        mut self,
        permissions_overwrite: PermissionsOverwriteParams,
    ) -> Self {
        self.permissions_overwrites.push(permissions_overwrite);
        self
    }

    pub fn keep_extra_channels(mut self) -> Self {
        self.extra_channels = CategoryParamsExtraChannelsStrategy::Keep;
        self
    }

    pub fn build(self) -> CategoryParams {
        CategoryParams {
            name: self.name,
            permissions_overwrites: self.permissions_overwrites,
            extra_channels: self.extra_channels,
        }
    }
}
