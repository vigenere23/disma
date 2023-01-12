use disma::channel::{ChannelType, ChannelsList, ExistingChannel};

use crate::infra::config::permission::PermissionsOverwritesConfig;

use super::{ChannelConfig, ChannelConfigType, ChannelConfigsList};

impl From<&ChannelsList<ExistingChannel>> for ChannelConfigsList {
    fn from(channels: &ChannelsList<ExistingChannel>) -> Self {
        let items = channels.to_list().iter().map(Into::into).collect();

        ChannelConfigsList {
            items,
            ..Default::default()
        }
    }
}

impl From<&ExistingChannel> for ChannelConfig {
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
            .map(PermissionsOverwritesConfig::from)
            .collect::<Vec<PermissionsOverwritesConfig>>();

        Self {
            name: channel.name.clone(),
            topic: channel.topic.clone(),
            _type,
            category,
            permissions_overwrites,
        }
    }
}

impl From<ChannelType> for ChannelConfigType {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::TEXT => Self::TEXT,
            ChannelType::VOICE => Self::VOICE,
        }
    }
}

#[cfg(test)]
mod tests {
    use disma::{
        category::ExistingCategory,
        channel::{ChannelType, ChannelsList, ExistingChannel},
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::ExistingRole,
    };

    use crate::infra::config::{
        channel::{ChannelConfig, ChannelConfigType, ChannelConfigsList},
        permission::PermissionsOverwritesConfig,
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

    fn given_matching_existing_and_config(
        name: &str,
        role: &ExistingRole,
        category: &ExistingCategory,
    ) -> (ExistingChannel, ChannelConfig) {
        let existing = ExistingChannel {
            id: "something".to_string(),
            name: name.to_string(),
            category: Some(category.clone()),
            channel_type: ChannelType::VOICE,
            topic: Some("A nice winter".to_string()),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: role.clone(),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
            }]),
        };

        let config = ChannelConfig {
            name: name.to_string(),
            category: Some(category.name.clone()),
            _type: ChannelConfigType::VOICE,
            topic: Some("A nice winter".to_string()),
            permissions_overwrites: vec![PermissionsOverwritesConfig {
                role: role.name.clone(),
                allow: vec![Permission::ADMINISTRATOR],
                deny: vec![Permission::SEND_MESSAGES],
            }],
        };

        (existing, config)
    }

    fn given_matching_existing_list_and_config_list(
        name: &str,
        role: &ExistingRole,
        category: &ExistingCategory,
    ) -> (ChannelsList<ExistingChannel>, ChannelConfigsList) {
        let (existing_item, config_item) = given_matching_existing_and_config(name, role, category);

        let existing_list = ChannelsList::from(vec![existing_item]);

        let config_list = ChannelConfigsList {
            items: vec![config_item],
            ..Default::default()
        };

        (existing_list, config_list)
    }

    #[test]
    fn can_convert_existing_entity_to_config() {
        let name = "channel_1";
        let role = given_existing_role("role_1");
        let category = given_existing_category("category_1");
        let (existing, expected_config) =
            given_matching_existing_and_config(name, &role, &category);

        let config = ChannelConfig::from(&existing);

        assert_eq!(config, expected_config);
    }

    #[test]
    fn can_convert_existing_entities_list_to_config_list() {
        let name = "channel_1";
        let role = given_existing_role("role_1");
        let category = given_existing_category("category_1");
        let (existing_list, expected_config_list) =
            given_matching_existing_list_and_config_list(name, &role, &category);

        let config = ChannelConfigsList::from(&existing_list);

        assert_eq!(config, expected_config_list);
    }
}
