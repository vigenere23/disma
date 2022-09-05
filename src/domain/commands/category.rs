use std::sync::Arc;

use crate::domain::{
    category::AwaitingCategory,
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
