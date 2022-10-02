use crate::{
    domain::entities::{
        category::{AwaitingCategory, ExistingCategory},
        role::{ExistingRole, RolesList},
    },
    guild::GuildCommanderRef,
};

use super::base::{Diff, DiffCommand};

pub struct AddCategory {
    category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl AddCategory {
    pub fn new(category: AwaitingCategory, roles: RolesList<ExistingRole>) -> Self {
        Self { category, roles }
    }
}

impl DiffCommand for AddCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.add_category(&self.category, &self.roles);
    }

    fn describe(&self) -> Diff {
        Diff::Add(format!("category \"{}\"", &self.category.name))
    }
}

pub struct UpdateCategory {
    existing_category: ExistingCategory,
    awaiting_category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl UpdateCategory {
    pub fn new(
        existing_category: ExistingCategory,
        awaiting_category: AwaitingCategory,
        roles: RolesList<ExistingRole>,
    ) -> Self {
        Self {
            existing_category,
            awaiting_category,
            roles,
        }
    }
}

impl DiffCommand for UpdateCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_category(
            &self.existing_category.id,
            &self.awaiting_category,
            &self.roles,
        );
    }

    fn describe(&self) -> Diff {
        Diff::Update(
            format!("category \"{}\"", &self.existing_category.name),
            vec![
                Diff::Remove(format!("{:#?}", &self.existing_category)), // TODO more granular diffs
                Diff::Add(format!("{:#?}", &self.awaiting_category)),
            ],
        )
    }
}

pub struct DeleteCategory {
    category: ExistingCategory,
}

impl DeleteCategory {
    pub fn new(category: ExistingCategory) -> Self {
        Self { category }
    }
}

impl DiffCommand for DeleteCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_category(&self.category.id);
    }

    fn describe(&self) -> Diff {
        Diff::Remove(format!("category \"{}\"", &self.category.name))
    }
}
