use crate::{
    api::params::permission::PermissionsOverwriteParams,
    channel::{ChannelType, ExistingChannel},
};

use super::{ChannelParams, ChannelParamsChannelType, ChannelParamsPermissionsOverwritesStrategy};

impl From<&ExistingChannel> for ChannelParams {
    fn from(channel: &ExistingChannel) -> Self {
        let _type = channel.channel_type.clone().into();

        let category = channel
            .category
            .as_ref()
            .map(|category| category.name.clone());

        let permissions_overwrites = channel
            .overwrites
            .to_list()
            .iter()
            .map(PermissionsOverwriteParams::from)
            .collect::<Vec<PermissionsOverwriteParams>>();

        Self {
            name: channel.name.clone(),
            topic: channel.topic.clone(),
            _type,
            category,
            permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy::Manual {
                items: permissions_overwrites,
            },
        }
    }
}

impl From<ChannelType> for ChannelParamsChannelType {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::TEXT => Self::TEXT,
            ChannelType::VOICE => Self::VOICE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::params::{
            channel::{
                ChannelParams, ChannelParamsChannelType, ChannelParamsPermissionsOverwritesStrategy,
            },
            permission::PermissionsOverwriteParams,
        },
        category::ExistingCategory,
        channel::{ChannelType, ExistingChannel},
        permission::{
            Permission, PermissionsList, PermissionsOverwrite, PermissionsOverwritesList,
        },
        role::ExistingRole,
    };

    fn given_existing_category(name: &str) -> ExistingCategory {
        ExistingCategory {
            id: "some_id".to_string(),
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::new(),
        }
    }

    fn given_existing_role(name: &str) -> ExistingRole {
        ExistingRole {
            id: "bob".to_string(),
            name: name.to_string(),
            permissions: PermissionsList::from(vec![Permission::VIEW_CHANNEL]),
            color: Some("123456".to_string()),
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    fn given_matching_existing_and_params(
        name: &str,
        role: &ExistingRole,
        category: &ExistingCategory,
    ) -> (ExistingChannel, ChannelParams) {
        let existing = ExistingChannel {
            id: "something".to_string(),
            name: name.to_string(),
            category: Some(category.clone()),
            channel_type: ChannelType::VOICE,
            topic: Some("A nice winter".to_string()),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
            }]),
        };

        let params = ChannelParams {
            name: name.to_string(),
            category: Some(category.name.clone()),
            _type: ChannelParamsChannelType::VOICE,
            topic: Some("A nice winter".to_string()),
            permissions_overwrites: ChannelParamsPermissionsOverwritesStrategy::Manual {
                items: vec![PermissionsOverwriteParams {
                    role: role.name.clone(),
                    allow: vec![Permission::ADMINISTRATOR],
                    deny: vec![Permission::SEND_MESSAGES],
                }],
            },
        };

        (existing, params)
    }

    #[test]
    fn can_convert_existing_entity_to_params() {
        let name = "channel_1";
        let role = given_existing_role("role_1");
        let category = given_existing_category("category_1");
        let (existing, expected_params) =
            given_matching_existing_and_params(name, &role, &category);

        let params = ChannelParams::from(&existing);

        assert_eq!(params, expected_params);
    }
}
