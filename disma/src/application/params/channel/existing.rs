use crate::{
    channel::{ChannelType, ChannelsList, ExistingChannel},
    params::permission::PermissionsOverwriteParams,
};

use super::{
    ChannelParams, ChannelParamsChannelType, ChannelParamsPermissionsOverwritesStrategy,
    ChannelsParamsList,
};

impl From<&ChannelsList<ExistingChannel>> for ChannelsParamsList {
    fn from(channels: &ChannelsList<ExistingChannel>) -> Self {
        let items = channels.to_list().iter().map(Into::into).collect();

        ChannelsParamsList {
            items,
            ..Default::default()
        }
    }
}

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
        category::ExistingCategory,
        channel::{ChannelType, ChannelsList, ExistingChannel},
        params::{
            channel::{
                ChannelParams, ChannelParamsChannelType,
                ChannelParamsPermissionsOverwritesStrategy, ChannelsParamsList,
            },
            permission::PermissionsOverwriteParams,
        },
        permission::{
            Permission, PermissionsList, PermissionsOverwrite, PermissionsOverwritesList,
        },
        role::ExistingRole,
    };

    fn given_existing_category(name: &str) -> ExistingCategory {
        ExistingCategory {
            id: "some_id".to_string(),
            name: name.to_string(),
            overwrites: PermissionsOverwritesList::from(vec![]),
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

    fn given_matching_existing_list_and_params_list(
        name: &str,
        role: &ExistingRole,
        category: &ExistingCategory,
    ) -> (ChannelsList<ExistingChannel>, ChannelsParamsList) {
        let (existing, params) = given_matching_existing_and_params(name, role, category);

        let existing_list = ChannelsList::from(vec![existing]);

        let params_list = ChannelsParamsList {
            items: vec![params],
            ..Default::default()
        };

        (existing_list, params_list)
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

    #[test]
    fn can_convert_existing_entities_list_to_params_list() {
        let name = "channel_1";
        let role = given_existing_role("role_1");
        let category = given_existing_category("category_1");
        let (existing_list, expected_params_list) =
            given_matching_existing_list_and_params_list(name, &role, &category);

        let params = ChannelsParamsList::from(&existing_list);

        assert_eq!(params, expected_params_list);
    }
}
