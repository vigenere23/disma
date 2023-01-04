use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, ExistingCategory},
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::Differ,
    guild::{ExistingGuild, GuildCommanderRef},
    role::{ExistingRole, RolesList},
};

use super::{AwaitingCategoriesList, CategoriesList};

impl CommandFactory for AwaitingCategoriesList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut commands: Vec<CommandRef> = Vec::new();

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
                        commands.push(Arc::from(command));
                    }
                }
                None => {
                    let command =
                        AddCategory::new(awaiting_category.clone(), existing_guild.roles.clone());
                    commands.push(Arc::from(command));
                }
            }
        }

        self.extra_items_strategy.handle_extra_roles(
            &self.items,
            &existing_guild.categories,
            &mut commands,
        );

        commands
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

pub trait ExtraCategoriesStrategy {
    fn _type(&self) -> ExtraCategoriesStrategyType;
    fn handle_extra_roles(
        &self,
        awaiting_categories: &CategoriesList<AwaitingCategory>,
        existing_categories: &CategoriesList<ExistingCategory>,
        commands: &mut Vec<CommandRef>,
    );
}

#[derive(Debug, PartialEq)]
pub enum ExtraCategoriesStrategyType {
    Keep,
    Remove,
}

impl PartialEq for dyn ExtraCategoriesStrategy {
    fn eq(&self, other: &Self) -> bool {
        self._type().eq(&other._type())
    }
}

impl Debug for dyn ExtraCategoriesStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type())
    }
}

pub struct RemoveExtraCategories {}

impl ExtraCategoriesStrategy for RemoveExtraCategories {
    fn _type(&self) -> ExtraCategoriesStrategyType {
        ExtraCategoriesStrategyType::Remove
    }

    fn handle_extra_roles(
        &self,
        awaiting_categories: &CategoriesList<AwaitingCategory>,
        existing_categories: &CategoriesList<ExistingCategory>,
        commands: &mut Vec<CommandRef>,
    ) {
        for existing_category in existing_categories.to_list() {
            if awaiting_categories
                .find_by_name(&existing_category.name)
                .is_none()
            {
                let command = DeleteCategory::new(existing_category.clone());
                commands.push(Arc::from(command));
            }
        }
    }
}

pub struct KeepExtraCategories {}

impl ExtraCategoriesStrategy for KeepExtraCategories {
    fn _type(&self) -> ExtraCategoriesStrategyType {
        ExtraCategoriesStrategyType::Keep
    }

    fn handle_extra_roles(
        &self,
        _awaiting_categories: &CategoriesList<AwaitingCategory>,
        _existing_categories: &CategoriesList<ExistingCategory>,
        _commands: &mut Vec<CommandRef>,
    ) {
    }
}
