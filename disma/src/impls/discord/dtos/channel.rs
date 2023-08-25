use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, ChannelType},
    role::{ExistingRole, RolesList},
};

use super::permissions::{PermissionOverwritesRequest, PermissionOverwritesResponse};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ChannelDtoType {
    Text = 0,
    Voice = 2,
    Category = 4,
}

impl From<&ChannelType> for ChannelDtoType {
    fn from(_type: &ChannelType) -> Self {
        match _type {
            ChannelType::TEXT => ChannelDtoType::Text,
            ChannelType::VOICE => ChannelDtoType::Voice,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChannelRequest {
    pub name: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub _type: ChannelDtoType,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwritesRequest>,
}

impl ChannelRequest {
    pub fn from_category(category: &AwaitingCategory, roles: &RolesList<ExistingRole>) -> Self {
        let permission_overwrites = category
            .overwrites
            .to_list()
            .iter()
            .map(|permission| PermissionOverwritesRequest::from(permission, roles))
            .collect();

        Self {
            name: category.name.clone(),
            topic: String::new(),
            _type: ChannelDtoType::Category,
            parent_id: None,
            permission_overwrites,
        }
    }

    pub fn from_channel(
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Self {
        let category = channel
            .category
            .as_ref()
            .map(|category| categories.find_by_name_panic(&category.name));

        let permission_overwrites = channel
            .overwrites
            .to_list()
            .iter()
            .map(|permission| PermissionOverwritesRequest::from(permission, roles))
            .collect();

        Self {
            name: channel.name.clone(),
            topic: channel.topic.clone().unwrap_or_default(),
            _type: ChannelDtoType::from(&channel.channel_type),
            parent_id: category.map(|category| category.id.clone()),
            permission_overwrites,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    #[serde(rename = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwritesResponse>,
}
