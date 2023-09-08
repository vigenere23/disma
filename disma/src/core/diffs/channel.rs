use crate::{
    channel::{AwaitingChannel, ExistingChannel},
    core::diffs::{Diff, Differ},
    utils::misc::IfThen,
};

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

        self.category_name()
            .diffs_with(&awaiting.category_name())
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
        category::AwaitingCategory,
        channel::{AwaitingChannel, ChannelType, ExistingChannel, KeepExtraChannels},
        core::diffs::{Diff, Differ},
        permission::{
            Permission, PermissionsList, PermissionsOverwrite, PermissionsOverwritesList,
        },
    };

    fn given_awaiting_category_with_name(name: String) -> AwaitingCategory {
        AwaitingCategory {
            name,
            overwrites: PermissionsOverwritesList::new(),
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
            category_name: None,
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let target = AwaitingChannel {
            name,
            topic: Some("Not here".to_string()),
            channel_type,
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
            category_name: None,
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let target = AwaitingChannel {
            name,
            topic,
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
            category_name: Some("category_a".to_string()),
            overwrites: PermissionsOverwritesList::from(vec![]),
        };

        let target = AwaitingChannel {
            name,
            topic,
            channel_type,
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
            category_name: None,
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                name: role_name.clone(),
                allow: PermissionsList::from(vec![Permission::READ_MESSAGE_HISTORY]),
                deny: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
            }]),
        };

        let target = AwaitingChannel {
            name,
            topic,
            channel_type,
            category: None,
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrite {
                name: role_name.clone(),
                allow: PermissionsList::from(vec![Permission::SEND_MESSAGES]),
                deny: PermissionsList::from(vec![Permission::READ_MESSAGE_HISTORY]),
            }]),
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "overwrites".to_string(),
            vec![Diff::Update(
                role_name,
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
