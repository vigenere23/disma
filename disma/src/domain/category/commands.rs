use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, ExistingCategory},
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::Differ,
    guild::{ExistingGuild, GuildCommanderRef},
    role::{ExistingRole, RolesList},
};

use super::{AwaitingCategoriesList, ExtraCategoriesStrategy};

impl CommandFactory for AwaitingCategoriesList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut diffs: Vec<CommandRef> = Vec::new();

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

pub struct AddCategory {
    category: AwaitingCategory,
    roles: RolesList<ExistingRole>,
}

impl AddCategory {
    pub fn new(category: AwaitingCategory, roles: RolesList<ExistingRole>) -> Self {
        Self { category, roles }
    }
}

impl Command for AddCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.add_category(&self.category, &self.roles);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Create(CommandEntity::Category, self.category.name.clone())
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

impl Command for UpdateCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_category(
            &self.existing_category.id,
            &self.awaiting_category,
            &self.roles,
        );
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Update(
            CommandEntity::Category,
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

impl Command for DeleteCategory {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_category(&self.category.id);
    }

    fn describe(&self) -> CommandDescription {
        CommandDescription::Delete(CommandEntity::Category, self.category.name.clone())
    }
}
