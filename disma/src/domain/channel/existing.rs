use crate::{
    category::{Category, ExistingCategory},
    permission::PermissionsOverwritesList,
    role::ExistingRole,
};

use super::{Channel, ChannelType};

#[derive(Debug, Clone)]
pub struct ExistingChannel {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<ExistingCategory>,
    pub overwrites: PermissionsOverwritesList<ExistingRole>,
}

impl Channel for ExistingChannel {
    fn name(&self) -> &str {
        &self.name
    }

    fn category_name(&self) -> Option<&str> {
        self.category.as_ref().map(|category| category.name())
    }

    fn channel_type(&self) -> &ChannelType {
        &self.channel_type
    }
}
