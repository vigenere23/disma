use std::sync::Arc;

use crate::domain::{
    category::{AwaitingCategory, ExistingCategory},
    guild::GuildCommander,
    role::{ExistingRole, RolesList},
};

use super::GuildCommand;

pub struct AddCategory {
    guild_commander: Arc<dyn GuildCommander>,
    category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl AddCategory {
    pub fn new(
        guild_commander: Arc<dyn GuildCommander>,
        category: AwaitingCategory,
        roles: RolesList<ExistingRole>,
    ) -> Self {
        Self {
            guild_commander,
            category,
            roles,
        }
    }
}

impl GuildCommand for AddCategory {
    fn execute(&self) {
        self.guild_commander
            .add_category(&self.category, &self.roles);
    }

    fn describe(&self) -> String {
        format!("🆕 Adding category {}", &self.category.name)
    }
}

pub struct UpdateCategory {
    guild_commander: Arc<dyn GuildCommander>,
    existing_category: ExistingCategory,
    awaiting_category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl UpdateCategory {
    pub fn new(
        guild_commander: Arc<dyn GuildCommander>,
        existing_category: ExistingCategory,
        awaiting_category: AwaitingCategory,
        roles: RolesList<ExistingRole>,
    ) -> Self {
        Self {
            guild_commander,
            existing_category,
            awaiting_category,
            roles,
        }
    }
}

impl GuildCommand for UpdateCategory {
    fn execute(&self) {
        self.guild_commander.update_category(
            &self.existing_category.id,
            &self.awaiting_category,
            &self.roles,
        );
    }

    fn describe(&self) -> String {
        format!(
            "🔄 Updating role {}\nfrom :{:#?}\nto :{:#?}",
            &self.existing_category.name, &self.existing_category, &self.awaiting_category
        )
    }
}

pub struct DeleteCategory {
    guild_commander: Arc<dyn GuildCommander>,
    category: ExistingCategory,
}

impl DeleteCategory {
    pub fn new(guild_commander: Arc<dyn GuildCommander>, category: ExistingCategory) -> Self {
        Self {
            guild_commander,
            category,
        }
    }
}

impl GuildCommand for DeleteCategory {
    fn execute(&self) {
        self.guild_commander.delete_category(&self.category.id);
    }

    fn describe(&self) -> String {
        format!("🗑️  Deleting category {}", &self.category.name)
    }
}
