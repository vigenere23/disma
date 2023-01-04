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

impl PartialEq<AwaitingRole> for ExistingRole {
    fn eq(&self, other: &AwaitingRole) -> bool {
        self.name == other.name
            && self.permissions == other.permissions
            && self.color == other.color
            && self.is_mentionable == other.is_mentionable
            && self.show_in_sidebar == other.show_in_sidebar
    }
}
