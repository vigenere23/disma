use crate::{
    domain::diff::{Diff, Differ},
    utils::misc::IfThen,
};

use super::{AwaitingCategory, ExistingCategory};

impl Differ<AwaitingCategory> for ExistingCategory {
    fn diffs_with(&self, awaiting: &AwaitingCategory) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.overwrites.diffs_with(&awaiting.overwrites).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("overwrites".into(), diffs)),
        );

        all_diffs
    }
}

impl PartialEq<AwaitingCategory> for ExistingCategory {
    fn eq(&self, other: &AwaitingCategory) -> bool {
        self.name == other.name && self.overwrites == other.overwrites
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        category::{AwaitingCategory, ExistingCategory},
        channel::KeepExtraChannels,
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

    #[test]
    fn can_diff_overwrites_updates() {
        let name = "category_a".to_string();
        let role_name = "role_a".to_string();
        let extra_channels_strategy = Arc::from(KeepExtraChannels {});

        let origin = ExistingCategory {
            id: "something".to_string(),
            name: name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: given_existing_role_with(role_name.clone()),
                allow: PermissionsList::from(vec![Permission::ADD_REACTIONS]),
                deny: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            }]),
        };

        let target = AwaitingCategory {
            name: name.clone(),
            overwrites: PermissionsOverwritesList::from(vec![PermissionsOverwrites {
                role: given_awaiting_role_with(role_name.clone()),
                allow: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
                deny: PermissionsList::from(vec![Permission::ADD_REACTIONS]),
            }]),
            extra_channels_strategy,
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
                            Diff::Remove(Permission::ADD_REACTIONS.to_string()),
                            Diff::Add(Permission::ADMINISTRATOR.to_string()),
                        ],
                    ),
                    Diff::Update(
                        "deny".to_string(),
                        vec![
                            Diff::Remove(Permission::ADMINISTRATOR.to_string()),
                            Diff::Add(Permission::ADD_REACTIONS.to_string()),
                        ],
                    ),
                ],
            )],
        )];
        assert_eq!(diffs, expected_diffs);
    }
}
