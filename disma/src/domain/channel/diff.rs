use crate::{
    category::Category,
    diff::{Diff, Differ},
    utils::{misc::IfThen, option::OptionEq},
};

use super::{AwaitingChannel, ExistingChannel};

impl PartialEq<AwaitingChannel> for ExistingChannel {
    fn eq(&self, other: &AwaitingChannel) -> bool {
        self.name == other.name
            && self.topic == other.topic
            && self.channel_type == other.channel_type
            && self.category.option_eq(&other.category)
            && self.overwrites == other.overwrites
    }
}

impl Differ<AwaitingChannel> for ExistingChannel {
    fn diffs_with(&self, awaiting: &AwaitingChannel) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.topic.diffs_with(&awaiting.topic).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("topic".into(), diffs)),
        );

        self.channel_type
            .diffs_with(&awaiting.channel_type)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("channel_type".into(), diffs)),
            );

        self.category
            .as_ref()
            .map(|category| category.name())
            .diffs_with(&awaiting.category.as_ref().map(|category| category.name()))
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("category".into(), diffs)),
            );

        self.overwrites.diffs_with(&awaiting.overwrites).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("overwrites".into(), diffs)),
        );

        all_diffs
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        category::{AwaitingCategory, ExistingCategory},
        channel::{AwaitingChannel, ChannelType, ExistingChannel, KeepExtraChannels},
        diff::{Diff, Differ},
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::{AwaitingRole, ExistingRole},
    };

    fn given_existing_role_with(name: String) -> ExistingRole {
        ExistingRole {
            id: "something".to_string(),
            name: name.clone(),
            permissions: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
            color: Some("a3bb30".to_string()),
            is_mentionable: true,
            show_in_sidebar: true,
        }
    }

    fn given_awaiting_role_with(name: String) -> AwaitingRole {
        AwaitingRole {
            name: name.clone(),
            permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            color: None,
            is_mentionable: false,
            show_in_sidebar: false,
        }
    }

    fn given_existing_category_with_name(name: String) -> ExistingCategory {
        ExistingCategory {
            id: "something".to_string(),
            name: name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![]),
        }
    }

    fn given_awaiting_category_with_name(name: String) -> AwaitingCategory {
        AwaitingCategory {
            name: name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![]),
            extra_channels_strategy: Arc::from(KeepExtraChannels {}),
        }
    }

    #[test]
    fn can_diff_topic_update() {
        let name = "channel_a".to_string();
        let channel_type = ChannelType::TEXT;

        let origin = ExistingChannel {
            id: "something".to_string(),
            name: name.clone(),
            topic: Some("bang bang!".to_string()),
            channel_type: channel_type.clone(),
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let target = AwaitingChannel {
            name: name.clone(),
            topic: Some("Not here".to_string()),
            channel_type: channel_type.clone(),
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "topic".to_string(),
            vec![
                Diff::Remove("bang bang!".to_string()),
                Diff::Add("Not here".to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_channel_type_update() {
        let name = "channel_a".to_string();
        let topic = Some("Not here!".to_string());

        let origin = ExistingChannel {
            id: "something".to_string(),
            name: name.clone(),
            topic: topic.clone(),
            channel_type: ChannelType::TEXT,
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let target = AwaitingChannel {
            name: name.clone(),
            topic: topic.clone(),
            channel_type: ChannelType::VOICE,
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "channel_type".to_string(),
            vec![
                Diff::Remove(ChannelType::TEXT.to_string()),
                Diff::Add(ChannelType::VOICE.to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_category_update() {
        let name = "channel_a".to_string();
        let topic = Some("Not here!".to_string());
        let channel_type = ChannelType::TEXT;

        let origin = ExistingChannel {
            id: "something".to_string(),
            name: name.clone(),
            topic: topic.clone(),
            channel_type: channel_type.clone(),
            category: Some(given_existing_category_with_name("category_a".to_string())),
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let target = AwaitingChannel {
            name: name.clone(),
            topic: topic.clone(),
            channel_type: channel_type.clone(),
            category: Some(given_awaiting_category_with_name("category_b".to_string())),
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "category".to_string(),
            vec![
                Diff::Remove("category_a".to_string()),
                Diff::Add("category_b".to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_overwrites_update() {
        let name = "channel_a".to_string();
        let topic = Some("Not here!".to_string());
        let channel_type = ChannelType::TEXT;
        let role_name = "role_a".to_string();

        let origin = ExistingChannel {
            id: "something".to_string(),
            name: name.clone(),
            topic: topic.clone(),
            channel_type: channel_type.clone(),
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: given_existing_role_with(role_name.clone()),
                allow: PermissionsList::from(vec![Permission::READ_MESSAGE_HISTORY]),
                deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
            }]),
        };

        let target = AwaitingChannel {
            name: name.clone(),
            topic: topic.clone(),
            channel_type: channel_type.clone(),
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: given_awaiting_role_with(role_name.clone()),
                allow: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
                deny: PermissionsList::from(vec![Permission::READ_MESSAGE_HISTORY]),
            }]),
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "overwrites".to_string(),
            vec![Diff::Update(
                role_name.clone(),
                vec![
                    Diff::Update(
                        "allow".to_string(),
                        vec![
                            Diff::Remove(Permission::READ_MESSAGE_HISTORY.to_string()),
                            Diff::Add(Permission::SEND_MESSAGES.to_string()),
                        ],
                    ),
                    Diff::Update(
                        "deny".to_string(),
                        vec![
                            Diff::Remove(Permission::SEND_MESSAGES.to_string()),
                            Diff::Add(Permission::READ_MESSAGE_HISTORY.to_string()),
                        ],
                    ),
                ],
            )],
        )];
        assert_eq!(diffs, expected_diffs);
    }
}
