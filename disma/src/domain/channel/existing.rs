use crate::permission::PermissionsOverwritesList;

use super::{Channel, ChannelType, UniqueChannelName};

#[derive(Debug, Clone)]
pub struct ExistingChannel {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category_name: Option<String>,
    pub overwrites: PermissionsOverwritesList,
}

impl ExistingChannel {
    pub fn category_name(&self) -> Option<&str> {
        self.category_name.as_deref()
    }
}

impl Channel for ExistingChannel {
    fn name(&self) -> &str {
        &self.name
    }

    fn unique_name(&self) -> UniqueChannelName {
        UniqueChannelName::from(&self.name, &self.channel_type, self.category_name())
    }
}
