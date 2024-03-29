use crate::{
    category::{AwaitingCategory, ExistingCategory},
    core::{
        diffs::{Diff, Differ},
        ListComparison,
    },
    guild::{AwaitingGuild, ExistingGuild},
};

#[derive(PartialEq, Debug)]
pub enum CategoryChange {
    Create(AwaitingCategory),
    Update(ExistingCategory, AwaitingCategory, Vec<Diff>),
    Delete(ExistingCategory),
}

pub struct CategoryChangesService {}

impl CategoryChangesService {
    pub fn list_changes(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<CategoryChange> {
        let ListComparison {
            extra_self: extra_awaiting,
            extra_other: extra_existing,
            same,
        } = awaiting_guild
            .categories
            .items
            .compare_by_name(existing_guild.categories());

        let to_create = extra_awaiting
            .into_iter()
            .map(|awaiting| CategoryChange::Create(awaiting.clone()));

        let to_update = same.into_iter().filter_map(|(awaiting, existing)| {
            let diffs = existing.diffs_with(awaiting);
            match diffs.is_empty() {
                true => None,
                false => Some(CategoryChange::Update(
                    existing.clone(),
                    awaiting.clone(),
                    diffs,
                )),
            }
        });

        let mut to_delete: Vec<CategoryChange> = Vec::new();
        for existing in extra_existing.into_iter() {
            awaiting_guild
                .categories
                .extra_items_strategy
                .handle_extra_category(existing, &mut to_delete);
        }

        to_create.chain(to_update).chain(to_delete).collect()
    }
}
