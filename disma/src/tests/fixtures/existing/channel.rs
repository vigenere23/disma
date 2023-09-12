use fake::Fake;

use crate::{
    category::ExistingCategory,
    channel::{ChannelType, ExistingChannel},
    permission::PermissionsOverwritesList,
    role::ExistingRole,
};

pub struct ExistingChannelFixture {
    id: String,
    name: String,
    overwrites: PermissionsOverwritesList<ExistingRole>,
    topic: Option<String>,
    channel_type: ChannelType,
    category: Option<ExistingCategory>,
}

impl ExistingChannelFixture {
    pub fn new() -> Self {
        Self {
            id: fake::uuid::UUIDv4.fake(),
            name: fake::faker::lorem::en::Word().fake(),
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

    pub fn with_category(mut self, category: &ExistingCategory) -> Self {
        self.category = Some(category.clone());
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
