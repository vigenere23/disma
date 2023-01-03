use std::sync::Arc;

use crate::{
    diff::{
        base::{Diff, DiffCommandFactory, DiffCommandRef, Differ},
        category::{AddCategory, DeleteCategory, UpdateCategory},
    },
    utils::misc::IfThen,
};

use super::{AwaitingCategoriesList, AwaitingCategory, ExistingCategory, ExtraCategoriesStrategy};

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

impl DiffCommandFactory for AwaitingCategoriesList {
    fn diff_commands_for(
        &self,
        existing_guild: &crate::guild::ExistingGuild,
    ) -> Vec<crate::diff::base::DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

        for awaiting_category in self.items.to_list() {
            match existing_guild
                .categories
                .find_by_name(&awaiting_category.name)
            {
                Some(existing_category) => {
                    if existing_category != awaiting_category {
                        let command = UpdateCategory::new(
                            existing_category.clone(),
                            awaiting_category.clone(),
                            existing_guild.roles.clone(),
                        );
                        diffs.push(Arc::from(command));
                    }
                }
                None => {
                    let command =
                        AddCategory::new(awaiting_category.clone(), existing_guild.roles.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        if self.extra_items.strategy == ExtraCategoriesStrategy::Remove {
            for existing_category in existing_guild.categories.to_list() {
                if self.items.find_by_name(&existing_category.name).is_none() {
                    let command = DeleteCategory::new(existing_category.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        diffs
    }
}
