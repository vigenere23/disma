use crate::{
    category::AwaitingCategoriesList, channel::AwaitingChannelsList, role::AwaitingRolesList,
};

#[derive(Debug)]
pub struct AwaitingGuild {
    pub roles: AwaitingRolesList,
    pub categories: AwaitingCategoriesList,
    pub channels: AwaitingChannelsList,
}
