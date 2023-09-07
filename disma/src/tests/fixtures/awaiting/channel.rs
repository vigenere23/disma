use crate::{
    category::AwaitingCategory,
    channel::{AwaitingChannel, ChannelType},
    permission::PermissionsOverwritesList,
};

pub struct AwaitingChannelFixture {
    name: String,
    overwrites: PermissionsOverwritesList,
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
