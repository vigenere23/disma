use crate::{
    base::ListComparison,
    category::{AwaitingCategory, ExistingCategory},
    guild::{AwaitingGuild, ExistingGuild},
};

pub enum CategoryChange {
    Create(AwaitingCategory),
    Update(ExistingCategory, AwaitingCategory),
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
            .map(|(awaiting, existing)| CategoryChange::Update(existing.clone(), awaiting.clone()));
        let to_delete = extra_existing
            .into_iter()
            .map(|existing| CategoryChange::Delete(existing.clone()));

        to_create.chain(to_update).chain(to_delete).collect()
    }
}
