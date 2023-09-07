use std::sync::Arc;

use crate::{
    category::AwaitingCategory,
    channel::{ExtraChannelsStrategy, RemoveExtraChannels},
    permission::PermissionsOverwritesList,
};

pub struct AwaitingCategoryFixture {
    name: String,
    overwrites: PermissionsOverwritesList,
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

    pub fn build(self) -> AwaitingCategory {
        AwaitingCategory {
            name: self.name,
            overwrites: self.overwrites,
            extra_channels_strategy: self.extra_channels_strategy,
        }
    }
}
