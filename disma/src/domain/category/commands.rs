use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    base::ListComparison,
    category::{AwaitingCategory, ExistingCategory},
    commands::{Command, CommandDescription, CommandEntity, CommandFactory, CommandRef},
    diff::{Diff, Differ},
    guild::{ExistingGuild, GuildCommanderRef},
    role::{ExistingRole, RolesList},
};

use super::AwaitingCategoriesList;

impl CommandFactory for AwaitingCategoriesList {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef> {
        let mut commands: Vec<CommandRef> = Vec::new();

        let ListComparison {
            extra_other: extra_existing,
            extra_self: extra_awaiting,
            same,
        } = self.items.compare_by_name(&existing_guild.categories);

        for awaiting_category in extra_awaiting.into_iter() {
            let command = AddCategory::new(awaiting_category.clone(), existing_guild.roles.clone());
            commands.push(Arc::from(command));
        }

        for (awaiting_category, existing_category) in same.into_iter() {
            // TODO replace with try_new
            if let Ok(command) = UpdateCategory::try_new(
                existing_category.clone(),
                awaiting_category.clone(),
                existing_guild.roles.clone(),
            ) {
                commands.push(Arc::from(command));
            }
        }

        for existing_category in extra_existing.into_iter() {
            self.extra_items_strategy
                .handle_extra_category(existing_category, &mut commands);
        }

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
    diffs: Vec<Diff>,
}

impl UpdateCategory {
    pub fn try_new(
        existing_category: ExistingCategory,
        awaiting_category: AwaitingCategory,
        roles: RolesList<ExistingRole>,
    ) -> Result<Self, String> {
        let diffs = existing_category.diffs_with(&awaiting_category);

        if diffs.is_empty() {
            return Err(format!(
                "No diffs between categories {} and {}",
                existing_category.name, awaiting_category.name
            ));
        }

        Ok(Self {
            existing_category,
            awaiting_category,
            roles,
            diffs,
        })
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
            self.diffs.clone(),
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
    fn handle_extra_category(
        &self,
        extra_category: &ExistingCategory,
        commands: &mut Vec<CommandRef>,
    );
}

#[derive(Debug, PartialEq)]
pub enum ExtraCategoriesStrategyType {
    Keep,
    Remove,
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

    fn handle_extra_category(
        &self,
        extra_category: &ExistingCategory,
        commands: &mut Vec<CommandRef>,
    ) {
        let command = DeleteCategory::new(extra_category.clone());
        commands.push(Arc::from(command));
    }
}

pub struct KeepExtraCategories {}

impl ExtraCategoriesStrategy for KeepExtraCategories {
    fn _type(&self) -> ExtraCategoriesStrategyType {
        ExtraCategoriesStrategyType::Keep
    }

    fn handle_extra_category(
        &self,
        _extra_category: &ExistingCategory,
        _commands: &mut Vec<CommandRef>,
    ) {
    }
}
