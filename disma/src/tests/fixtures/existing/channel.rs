use crate::{
    category::ExistingCategory,
    channel::{ChannelType, ExistingChannel},
    permission::PermissionsOverwritesList,
};

pub struct ExistingChannelFixture {
    id: String,
    name: String,
    overwrites: PermissionsOverwritesList,
    topic: Option<String>,
    channel_type: ChannelType,
    category: Option<ExistingCategory>,
}

impl ExistingChannelFixture {
    pub fn new() -> Self {
        Self {
            id: "123".to_string(),
            name: "abc".to_string(),
            overwrites: PermissionsOverwritesList::from(Vec::new()),
            topic: None,
            channel_type: ChannelType::TEXT,
            category: None,
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn build(self) -> ExistingChannel {
        ExistingChannel {
            id: self.id,
            name: self.name,
            overwrites: self.overwrites,
            topic: self.topic,
            channel_type: self.channel_type,
            category: self.category,
        }
    }
}
