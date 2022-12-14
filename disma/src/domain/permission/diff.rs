use crate::{
    diff::{Diff, Differ},
    role::{AwaitingRole, ExistingRole},
    utils::misc::IfThen,
};

use super::{PermissionsList, PermissionsOverwrites, PermissionsOverwritesList};

impl Differ<PermissionsList> for PermissionsList {
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        self.items().diffs_with(&target.items())
    }
}

impl Differ<PermissionsOverwrites<AwaitingRole>> for PermissionsOverwrites<ExistingRole> {
    fn diffs_with(&self, target: &PermissionsOverwrites<AwaitingRole>) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.allow.diffs_with(&target.allow).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("allow".into(), diffs)),
        );

        self.deny.diffs_with(&target.deny).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("deny".into(), diffs)),
        );

        all_diffs
    }
}

impl Differ<PermissionsOverwritesList<AwaitingRole>> for PermissionsOverwritesList<ExistingRole> {
    fn diffs_with(&self, target: &PermissionsOverwritesList<AwaitingRole>) -> Vec<Diff> {
        let mut all_diffs = vec![];

        for existing_overwrite in self.to_list().iter() {
            match target.find_by_role_name(&existing_overwrite.role.name) {
                Some(awaiting_overwrite) => {
                    existing_overwrite.diffs_with(awaiting_overwrite).if_then(
                        |diffs| !diffs.is_empty(),
                        |diffs| {
                            all_diffs
                                .push(Diff::Update(existing_overwrite.role.name.clone(), diffs))
                        },
                    );
                }
                None => all_diffs.push(Diff::Remove(existing_overwrite.role.name.clone())),
            }
        }

        for awaiting_role in target.to_list().iter() {
            if self.find_by_role_name(&awaiting_role.role.name).is_none() {
                all_diffs.push(Diff::Add(awaiting_role.role.name.clone()))
            }
        }

        all_diffs
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        diff::{Diff, Differ},
        permission::{
            Permission, PermissionsList, PermissionsOverwrites, PermissionsOverwritesList,
        },
        role::{AwaitingRole, ExistingRole},
    };

    fn given_awaiting_role_with_name(name: String) -> AwaitingRole {
        AwaitingRole {
            name: name.clone(),
            permissions: PermissionsList::from(vec![Permission::ADD_REACTIONS]),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    fn given_existing_role_with_name(name: String) -> ExistingRole {
        ExistingRole {
            id: "something".to_string(),
            name: name.clone(),
            permissions: PermissionsList::from(vec![Permission::ADD_REACTIONS]),
            color: None,
            is_mentionable: true,
            show_in_sidebar: false,
        }
    }

    #[test]
    fn can_diff_permissions_list_update() {
        let origin = PermissionsList::from(vec![Permission::USE_VAD]);
        let target = PermissionsList::from(vec![Permission::CREATE_PUBLIC_THREADS]);

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![
            Diff::Remove(Permission::USE_VAD.to_string()),
            Diff::Add(Permission::CREATE_PUBLIC_THREADS.to_string()),
        ];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_permissions_overwites_update() {
        let role_name = "role_a".to_string();

        let origin = PermissionsOverwrites {
            role: given_existing_role_with_name(role_name.clone()),
            allow: PermissionsList::from(vec![Permission::USE_VAD]),
            deny: PermissionsList::from(vec![Permission::CREATE_PUBLIC_THREADS]),
        };
        let target = PermissionsOverwrites {
            role: given_awaiting_role_with_name(role_name.clone()),
            allow: PermissionsList::from(vec![Permission::CREATE_PUBLIC_THREADS]),
            deny: PermissionsList::from(vec![Permission::USE_VAD]),
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![
            Diff::Update(
                "allow".to_string(),
                vec![
                    Diff::Remove(Permission::USE_VAD.to_string()),
                    Diff::Add(Permission::CREATE_PUBLIC_THREADS.to_string()),
                ],
            ),
            Diff::Update(
                "deny".to_string(),
                vec![
                    Diff::Remove(Permission::CREATE_PUBLIC_THREADS.to_string()),
                    Diff::Add(Permission::USE_VAD.to_string()),
                ],
            ),
        ];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_permissions_overwrites_list_update() {
        let role_name = "role_a".to_string();

        let origin = PermissionsOverwritesList::from(vec![PermissionsOverwrites {
            role: given_existing_role_with_name(role_name.clone()),
            allow: PermissionsList::from(vec![Permission::USE_VAD]),
            deny: PermissionsList::from(vec![Permission::CREATE_PUBLIC_THREADS]),
        }]);

        let target = PermissionsOverwritesList::from(vec![PermissionsOverwrites {
            role: given_awaiting_role_with_name(role_name.clone()),
            allow: PermissionsList::from(vec![Permission::CREATE_PUBLIC_THREADS]),
            deny: PermissionsList::from(vec![Permission::USE_VAD]),
        }]);

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            role_name.clone(),
            vec![
                Diff::Update(
                    "allow".to_string(),
                    vec![
                        Diff::Remove(Permission::USE_VAD.to_string()),
                        Diff::Add(Permission::CREATE_PUBLIC_THREADS.to_string()),
                    ],
                ),
                Diff::Update(
                    "deny".to_string(),
                    vec![
                        Diff::Remove(Permission::CREATE_PUBLIC_THREADS.to_string()),
                        Diff::Add(Permission::USE_VAD.to_string()),
                    ],
                ),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }
}
