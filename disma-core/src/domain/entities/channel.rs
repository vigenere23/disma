use crate::category::{AwaitingCategory, ExistingCategory};

#[derive(Debug)]
pub enum ChannelType {
    Text,
    Voice,
}

pub struct AwaitingChannel {
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<AwaitingCategory>,
    // pub overwrites: PermissionsOverwritesList<AwaitingRole>,
}

#[derive(Debug)]
pub struct ExistingChannel {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    pub channel_type: ChannelType,
    pub category: Option<ExistingCategory>,
    // pub overwrites: PermissionsOverwritesList<ExistingRole>,
}
