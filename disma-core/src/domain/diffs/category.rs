use std::sync::Arc;

use crate::domain::entities::{
    category::{AwaitingCategory, ExistingCategory},
    guild::GuildCommander,
    role::{ExistingRole, RolesList},
};

use super::base::{Diff, DiffDescription};

pub struct AddCategory {
    category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl AddCategory {
    pub fn new(category: AwaitingCategory, roles: RolesList<ExistingRole>) -> Self {
        Self { category, roles }
    }
}

impl Diff for AddCategory {
    fn execute(&self, guild: Arc<dyn GuildCommander>) {
        guild.add_category(&self.category, &self.roles);
    }

    fn describe(&self) -> DiffDescription {
        DiffDescription {
            summary: format!("🆕 Adding category {}", &self.category.name),
            details: vec![],
        }
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

impl Diff for UpdateCategory {
    fn execute(&self, guild: Arc<dyn GuildCommander>) {
        guild.update_category(
            &self.existing_category.id,
            &self.awaiting_category,
            &self.roles,
        );
    }

    fn describe(&self) -> DiffDescription {
        DiffDescription {
            summary: format!(
                "🔄 Updating role {}\nfrom :{:#?}\nto :{:#?}",
                &self.existing_category.name, &self.existing_category, &self.awaiting_category
            ),
            details: vec![],
        }
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

impl Diff for DeleteCategory {
    fn execute(&self, guild: Arc<dyn GuildCommander>) {
        guild.delete_category(&self.category.id);
    }

    fn describe(&self) -> DiffDescription {
        DiffDescription {
            summary: format!("🗑️  Deleting category {}", &self.category.name),
            details: vec![],
        }
    }
}
