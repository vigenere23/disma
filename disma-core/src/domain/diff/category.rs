use crate::{
    diff::base::Entity,
    domain::entities::{
        category::{AwaitingCategory, ExistingCategory},
        role::{ExistingRole, RolesList},
    },
    guild::GuildCommanderRef,
};

use super::base::{DiffCommand, EntityChange};

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

    fn describe(&self) -> EntityChange {
        EntityChange::Create(Entity::Category, self.category.name.clone())
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

    fn describe(&self) -> EntityChange {
        EntityChange::Update(
            Entity::Category,
            self.existing_category.name.clone(),
            self.existing_category.diffs_with(&self.awaiting_category),
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

    fn describe(&self) -> EntityChange {
        EntityChange::Delete(Entity::Category, self.category.name.clone())
    }
}
