use std::sync::Arc;

use crate::{
    category::AwaitingCategory,
    channel::{ExtraChannelsStrategy, RemoveExtraChannels},
    permission::{PermissionsOverwrite, PermissionsOverwritesList},
    role::AwaitingRole,
};

pub struct AwaitingCategoryFixture {
    name: String,
    overwrites: PermissionsOverwritesList<AwaitingRole>,
    extra_channels_strategy: Arc<dyn ExtraChannelsStrategy>,
}

impl AwaitingCategoryFixture {
    pub fn new() -> Self {
        Self {
            name: "abc".to_string(),
            overwrites: PermissionsOverwritesList::from(Vec::new()),
            extra_channels_strategy: Arc::from(RemoveExtraChannels {}),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_permissions_overwrites(
        mut self,
        overwrites: Vec<PermissionsOverwrite<AwaitingRole>>,
    ) -> Self {
        self.overwrites = PermissionsOverwritesList::from(overwrites);
        self
    }

    pub fn build(self) -> AwaitingCategory {
        AwaitingCategory {
            name: self.name,
            overwrites: self.overwrites,
            extra_channels_strategy: self.extra_channels_strategy,
        }
    }
}
