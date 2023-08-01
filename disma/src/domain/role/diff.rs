use super::{AwaitingRole, ExistingRole};

use crate::{
    diff::{Diff, Differ},
    utils::misc::IfThen,
};

impl Differ<AwaitingRole> for ExistingRole {
    fn diffs_with(&self, awaiting: &AwaitingRole) -> Vec<Diff> {
        let mut all_diffs = vec![];

        self.permissions.diffs_with(&awaiting.permissions).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("permissions".into(), diffs)),
        );

        self.is_mentionable
            .diffs_with(&awaiting.is_mentionable)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("is_mentionable".into(), diffs)),
            );

        self.show_in_sidebar
            .diffs_with(&awaiting.show_in_sidebar)
            .if_then(
                |diffs| !diffs.is_empty(),
                |diffs| all_diffs.push(Diff::Update("show_in_sidebar".into(), diffs)),
            );

        self.color.diffs_with(&awaiting.color).if_then(
            |diffs| !diffs.is_empty(),
            |diffs| all_diffs.push(Diff::Update("color".into(), diffs)),
        );

        all_diffs
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        diff::{Diff, Differ},
        permission::{Permission, PermissionsList},
        role::{AwaitingRole, ExistingRole},
    };

    #[test]
    fn can_diff_permissions_update() {
        let name = "role_x".to_string();
        let is_mentionable = false;
        let show_in_sidebar = false;
        let color = None;

        let origin = ExistingRole {
            id: "abc".to_string(),
            name: name.clone(),
            permissions: PermissionsList::from(vec![Permission::SEND_MESSAGES] as Vec<Permission>),
            color: color.clone(),
            is_mentionable,
            show_in_sidebar,
        };

        let target = AwaitingRole {
            name,
            permissions: PermissionsList::from(vec![Permission::ADMINISTRATOR]),
            color,
            is_mentionable,
            show_in_sidebar,
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "permissions".to_string(),
            vec![
                Diff::Remove(Permission::SEND_MESSAGES.to_string()),
                Diff::Add(Permission::ADMINISTRATOR.to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_is_mentionable_update() {
        let name = "role_x".to_string();
        let permissions = PermissionsList::from(vec![Permission::ADMINISTRATOR]);
        let show_in_sidebar = false;
        let color = None;

        let origin = ExistingRole {
            id: "abc".to_string(),
            name: name.clone(),
            permissions: permissions.clone(),
            color: color.clone(),
            is_mentionable: false,
            show_in_sidebar,
        };

        let target = AwaitingRole {
            name,
            permissions,
            color,
            is_mentionable: true,
            show_in_sidebar,
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "is_mentionable".to_string(),
            vec![
                Diff::Remove("false".to_string()),
                Diff::Add("true".to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_show_in_sidebar_update() {
        let name = "role_x".to_string();
        let permissions = PermissionsList::from(vec![Permission::ADMINISTRATOR]);
        let is_mentionable = false;
        let color = None;

        let origin = ExistingRole {
            id: "abc".to_string(),
            name: name.clone(),
            permissions: permissions.clone(),
            color: color.clone(),
            is_mentionable,
            show_in_sidebar: true,
        };

        let target = AwaitingRole {
            name,
            permissions,
            color,
            is_mentionable,
            show_in_sidebar: false,
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "show_in_sidebar".to_string(),
            vec![
                Diff::Remove("true".to_string()),
                Diff::Add("false".to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }

    #[test]
    fn can_diff_color_update() {
        let name = "role_x".to_string();
        let permissions = PermissionsList::from(vec![Permission::ADMINISTRATOR]);
        let is_mentionable = false;
        let show_in_sidebar = true;

        let origin = ExistingRole {
            id: "abc".to_string(),
            name: name.clone(),
            permissions: permissions.clone(),
            color: Some("237683".to_string()),
            is_mentionable,
            show_in_sidebar,
        };

        let target = AwaitingRole {
            name,
            permissions,
            color: Some("ab83ba".to_string()),
            is_mentionable,
            show_in_sidebar,
        };

        let diffs = origin.diffs_with(&target);

        let expected_diffs = vec![Diff::Update(
            "color".to_string(),
            vec![
                Diff::Remove("237683".to_string()),
                Diff::Add("ab83ba".to_string()),
            ],
        )];
        assert_eq!(diffs, expected_diffs);
    }
}
