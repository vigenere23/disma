use crate::{
    category::AwaitingCategory,
    channel::{AwaitingChannel, ChannelType},
    permission::{PermissionsOverwrite, PermissionsOverwritesList},
    role::AwaitingRole,
};

pub struct AwaitingChannelFixture {
    name: String,
    overwrites: PermissionsOverwritesList<AwaitingRole>,
    topic: Option<String>,
    channel_type: ChannelType,
    category: Option<AwaitingCategory>,
}

impl AwaitingChannelFixture {
    pub fn new() -> Self {
        Self {
            name: "abc".to_string(),
            overwrites: PermissionsOverwritesList::from(Vec::new()),
            topic: None,
            channel_type: ChannelType::TEXT,
            category: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_category(mut self, category: &AwaitingCategory) -> Self {
        self.category = Some(category.clone());
        self
    }

    pub fn with_permissions_overwrites(
        mut self,
        overwrites: Vec<PermissionsOverwrite<AwaitingRole>>,
    ) -> Self {
        self.overwrites = PermissionsOverwritesList::from(overwrites);
        self
    }

    pub fn build(self) -> AwaitingChannel {
        AwaitingChannel {
            name: self.name,
            overwrites: self.overwrites,
            topic: self.topic,
            channel_type: self.channel_type,
            category: self.category,
        }
    }
}
