use crate::{
    base::ListComparison,
    category::{AwaitingCategory, ExistingCategory},
    diff::{Diff, Differ},
    guild::{AwaitingGuild, ExistingGuild},
};

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
            .compare_by_name(&existing_guild.categories);

        let to_create = extra_awaiting
            .into_iter()
            .map(|awaiting| CategoryChange::Create(awaiting.clone()));
        let to_update = same
            .into_iter()
            .map(|(awaiting, existing)| {
                let diffs = existing.diffs_with(&awaiting);
                match diffs.is_empty() {
                    true => None,
                    false => Some(CategoryChange::Update(
                        existing.clone(),
                        awaiting.clone(),
                        diffs,
                    )),
                }
            })
            .filter(|change| change.is_some())
            .map(|change| change.unwrap());
        let to_delete = extra_existing
            .into_iter()
            .map(|existing| CategoryChange::Delete(existing.clone()));

        to_create.chain(to_update).chain(to_delete).collect()
    }
}
