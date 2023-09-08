use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, ChannelType},
    permission::{PermissionsOverwrite, PermissionsOverwritesList},
    role::{ExistingRole, RolesList},
};

use super::permissions::{PermissionOverwritesRequest, PermissionOverwritesResponse};

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
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

#[derive(Debug, Serialize, PartialEq)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelResponse {
    pub id: String,
    pub name: String,
    pub topic: Option<String>,
    #[serde(rename = "type")]
    pub _type: u8,
    pub parent_id: Option<String>,
    pub permission_overwrites: Vec<PermissionOverwritesResponse>,
}

impl ChannelResponse {
    pub fn _into(self, roles: &RolesList<ExistingRole>) -> ExistingCategory {
        if self._type != 4 {
            panic!(
                "Cannot convert a Discord type {} channel into a Disma category",
                &self._type
            )
        }

        let permission_overwrites = self
            .permission_overwrites
            .into_iter()
            .filter_map(|permissions| {
                let result = permissions._try_into(roles);
                match result {
                    Ok(overwrites) => Some(overwrites),
                    Err(message) => {
                        eprintln!(
                            "Error while parsing permissions overwrites for category {}: {}",
                            &self.name, message
                        );
                        None
                    }
                }
            })
            .collect::<Vec<PermissionsOverwrite>>();

        ExistingCategory {
            id: self.id,
            name: self.name,
            overwrites: PermissionsOverwritesList::from(permission_overwrites),
        }
    }
}

#[cfg(test)]
mod tests {
    mod request {
        use crate::{
            category::CategoriesList,
            channel::{AwaitingChannel, ChannelType},
            impls::discord::dtos::{
                channel::{ChannelDtoType, ChannelRequest},
                permissions::{PermissionOverwriteType, PermissionOverwritesRequest},
            },
            permission::{PermissionsList, PermissionsOverwrite, PermissionsOverwritesList},
            role::RolesList,
            tests::fixtures::{
                awaiting::AwaitingCategoryFixture,
                existing::{ExistingCategoryFixture, ExistingRoleFixture},
            },
        };

        #[test]
        fn can_be_created_from_awaiting_category() {
            let existing_role = ExistingRoleFixture::new().build();

            let category = AwaitingCategoryFixture::new()
                .with_name("a category")
                .with_permissions_overwrites(vec![PermissionsOverwrite {
                    name: existing_role.name.clone(),
                    allow: PermissionsList::from("2113536"),
                    deny: PermissionsList::from("2113536"),
                }])
                .build();

            let expected_request = ChannelRequest {
                name: "a category".to_string(),
                topic: String::new(),
                _type: ChannelDtoType::Category,
                parent_id: None,
                permission_overwrites: vec![PermissionOverwritesRequest {
                    role_or_member_id: existing_role.id.clone(),
                    _type: PermissionOverwriteType::Role,
                    allow: "2113536".to_string(),
                    deny: "2113536".to_string(),
                }],
            };

            let request =
                ChannelRequest::from_category(&category, &RolesList::from(vec![existing_role]));

            assert_eq!(request, expected_request);
        }

        #[test]
        #[should_panic]
        fn given_non_existant_role_when_creating_from_awaiting_category_should_panic() {
            let category = AwaitingCategoryFixture::new()
                .with_name("a category")
                .with_permissions_overwrites(vec![PermissionsOverwrite {
                    name: "non-existant role".to_string(),
                    allow: PermissionsList::new(),
                    deny: PermissionsList::new(),
                }])
                .build();

            ChannelRequest::from_category(&category, &RolesList::new());
        }

        #[test]
        fn can_be_created_from_awaiting_channel() {
            let existing_role = ExistingRoleFixture::new().build();
            let existing_category = ExistingCategoryFixture::new()
                .with_name("a category")
                .build();
            let awaiting_category = AwaitingCategoryFixture::new()
                .with_name("a category")
                .build();

            let channel = AwaitingChannel {
                name: "a channel".to_string(),
                topic: Some("some topic".to_string()),
                channel_type: ChannelType::TEXT,
                category: Some(awaiting_category),
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                    name: existing_role.name.clone(),
                    allow: PermissionsList::from("2113536"),
                    deny: PermissionsList::from("2113536"),
                }]),
            };

            let expected_request = ChannelRequest {
                name: "a channel".to_string(),
                topic: "some topic".to_string(),
                _type: ChannelDtoType::Text,
                parent_id: Some(existing_category.id.clone()),
                permission_overwrites: vec![PermissionOverwritesRequest {
                    role_or_member_id: existing_role.id.clone(),
                    _type: PermissionOverwriteType::Role,
                    allow: "2113536".to_string(),
                    deny: "2113536".to_string(),
                }],
            };

            let request = ChannelRequest::from_channel(
                &channel,
                &RolesList::from(vec![existing_role]),
                &CategoriesList::from(vec![existing_category]),
            );

            assert_eq!(request, expected_request);
        }

        #[test]
        #[should_panic]
        fn given_non_existant_role_when_creating_from_awaiting_channel_should_panic() {
            let channel = AwaitingChannel {
                name: "a channel".to_string(),
                topic: Some("some topic".to_string()),
                channel_type: ChannelType::TEXT,
                category: None,
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                    name: "non-existant role".to_string(),
                    allow: PermissionsList::from("2113536"),
                    deny: PermissionsList::from("2113536"),
                }]),
            };

            ChannelRequest::from_channel(&channel, &RolesList::new(), &CategoriesList::new());
        }

        #[test]
        #[should_panic]
        fn given_non_existant_category_when_creating_from_awaiting_channel_should_panic() {
            let awaiting_category = AwaitingCategoryFixture::new()
                .with_name("a category")
                .build();

            let channel = AwaitingChannel {
                name: "a channel".to_string(),
                topic: Some("some topic".to_string()),
                channel_type: ChannelType::TEXT,
                category: Some(awaiting_category),
                overwrites: PermissionsOverwritesList::new(),
            };

            ChannelRequest::from_channel(&channel, &RolesList::new(), &CategoriesList::new());
        }
    }
}
