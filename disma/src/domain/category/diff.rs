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
