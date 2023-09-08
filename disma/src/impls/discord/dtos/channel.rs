use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, ChannelType, ExistingChannel},
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
    pub topic: Option<String>,
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
            topic: None,
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
            topic: channel.topic.clone(),
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
    pub fn into_category(self, roles: &RolesList<ExistingRole>) -> ExistingCategory {
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

    pub fn into_channel(
        self,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> ExistingChannel {
        let channel_type = match self._type {
            0 => ChannelType::TEXT,
            2 => ChannelType::VOICE,
            _ => panic!(
                "Cannot convert a Discord type {} channel into a Disma channel",
                &self._type
            ),
        };

        let category = self.parent_id.map(|category_id| {
            categories.find_by_id(&category_id).unwrap_or_else(|| {
                panic!(
                    "Could not create channel from non-existant category with id '{category_id}'"
                )
            })
        });

        let permission_overwrites = self
            .permission_overwrites
            .into_iter()
            .filter_map(|permissions| {
                let result = permissions._try_into(roles);
                match result {
                    Ok(overwrites) => Some(overwrites),
                    Err(message) => {
                        eprintln!(
                            "Error while parsing permissions overwrites for channel {}: {}",
                            &self.name, message
                        );
                        None
                    }
                }
            })
            .collect::<Vec<PermissionsOverwrite>>();

        ExistingChannel {
            id: self.id,
            name: self.name,
            channel_type,
            topic: self.topic.clone(),
            category: category.cloned(),
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
                topic: None,
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
                topic: Some("some topic".to_string()),
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

    mod response {
        use crate::{
            category::{CategoriesList, ExistingCategory},
            channel::{ChannelType, ExistingChannel},
            impls::discord::dtos::{
                channel::ChannelResponse, permissions::PermissionOverwritesResponse,
            },
            permission::{PermissionsList, PermissionsOverwrite, PermissionsOverwritesList},
            role::RolesList,
            tests::fixtures::existing::{ExistingCategoryFixture, ExistingRoleFixture},
        };

        #[test]
        fn can_be_converted_into_existing_category() {
            let existing_role = ExistingRoleFixture::new().build();

            let response = ChannelResponse {
                id: "a_category_id".to_string(),
                name: "a category".to_string(),
                topic: Some("some topic".to_string()),
                _type: 4,
                parent_id: None,
                permission_overwrites: vec![PermissionOverwritesResponse {
                    role_or_member_id: existing_role.id.clone(),
                    _type: 0,
                    allow: "2113536".to_string(),
                    deny: "2113536".to_string(),
                }],
            };

            let expected_category = ExistingCategory {
                id: "a_category_id".to_string(),
                name: "a category".to_string(),
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                    name: existing_role.name.clone(),
                    allow: PermissionsList::from("2113536"),
                    deny: PermissionsList::from("2113536"),
                }]),
            };

            let category = response.into_category(&RolesList::from(vec![existing_role]));

            assert_eq!(category, expected_category);
        }

        #[test]
        #[should_panic]
        fn given_non_existant_role_when_converting_into_existing_category_should_panic() {
            let response = ChannelResponse {
                id: "a_category_id".to_string(),
                name: "a category".to_string(),
                topic: Some("some topic".to_string()),
                _type: 4,
                parent_id: None,
                permission_overwrites: vec![PermissionOverwritesResponse {
                    role_or_member_id: "non-existant role id".to_string(),
                    _type: 0,
                    allow: "2113536".to_string(),
                    deny: "2113536".to_string(),
                }],
            };

            response.into_category(&RolesList::new());
        }

        #[test]
        #[should_panic]
        fn given_non_category_response_when_converting_into_existing_category_should_panic() {
            let channel_response = ChannelResponse {
                id: "a_channel_id".to_string(),
                name: "a channel".to_string(),
                topic: Some("some topic".to_string()),
                _type: 0,
                parent_id: None,
                permission_overwrites: vec![],
            };

            channel_response.into_category(&RolesList::new());
        }

        #[test]
        fn can_be_converted_into_existing_channel() {
            let existing_role = ExistingRoleFixture::new().build();
            let existing_category = ExistingCategoryFixture::new().build();

            let response = ChannelResponse {
                id: "a_category_id".to_string(),
                name: "a category".to_string(),
                topic: Some("some topic".to_string()),
                _type: 0,
                parent_id: Some(existing_category.id.clone()),
                permission_overwrites: vec![PermissionOverwritesResponse {
                    role_or_member_id: existing_role.id.clone(),
                    _type: 0,
                    allow: "2113536".to_string(),
                    deny: "2113536".to_string(),
                }],
            };

            let expected_channel = ExistingChannel {
                id: "a_category_id".to_string(),
                name: "a category".to_string(),
                topic: Some("some topic".to_string()),
                channel_type: ChannelType::TEXT,
                overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                    name: existing_role.name.clone(),
                    allow: PermissionsList::from("2113536"),
                    deny: PermissionsList::from("2113536"),
                }]),
                category_name: Some(existing_category.name.clone()),
            };

            let channel = response.into_channel(
                &RolesList::from(vec![existing_role]),
                &CategoriesList::from(vec![existing_category]),
            );

            assert_eq!(channel, expected_channel);
        }

        #[test]
        #[should_panic]
        fn given_non_existant_role_when_converting_into_existing_channel_should_panic() {
            let response = ChannelResponse {
                id: "a_channel_id".to_string(),
                name: "a channel".to_string(),
                topic: Some("some topic".to_string()),
                _type: 0,
                parent_id: None,
                permission_overwrites: vec![PermissionOverwritesResponse {
                    role_or_member_id: "non-existant role id".to_string(),
                    _type: 0,
                    allow: "2113536".to_string(),
                    deny: "2113536".to_string(),
                }],
            };

            response.into_channel(&RolesList::new(), &CategoriesList::new());
        }

        #[test]
        #[should_panic]
        fn given_non_existant_category_when_converting_into_existing_channel_should_panic() {
            let response = ChannelResponse {
                id: "a_channel_id".to_string(),
                name: "a channel".to_string(),
                topic: Some("some topic".to_string()),
                _type: 0,
                parent_id: Some("non-existant-caegory-id".to_string()),
                permission_overwrites: vec![],
            };

            response.into_channel(&RolesList::new(), &CategoriesList::new());
        }

        #[test]
        #[should_panic]
        fn given_non_channel_response_when_converting_into_existing_channel_should_panic() {
            let channel_response = ChannelResponse {
                id: "a_category_id".to_string(),
                name: "a category".to_string(),
                topic: Some("some topic".to_string()),
                _type: 4,
                parent_id: None,
                permission_overwrites: vec![],
            };

            channel_response.into_channel(&RolesList::new(), &CategoriesList::new());
        }
    }
}
