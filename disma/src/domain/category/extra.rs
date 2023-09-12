use core::fmt::Debug;
use std::sync::Arc;

use crate::{
    category::ExistingCategory,
    core::{
        changes::category::CategoryChange,
        commands::{category::DeleteCategory, CommandRef},
    },
};

pub trait ExtraCategoriesStrategy {
    fn _type(&self) -> ExtraCategoriesStrategyType;
    fn handle_extra_category_commands(
        &self,
        extra_existing: &ExistingCategory,
        commands: &mut Vec<CommandRef>,
    );
    fn handle_extra_category(
        &self,
        extra_existing: &ExistingCategory,
        changes: &mut Vec<CategoryChange>,
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

    fn handle_extra_category_commands(
        &self,
        extra_existing: &ExistingCategory,
        commands: &mut Vec<CommandRef>,
    ) {
        let command = DeleteCategory::new(extra_existing.clone());
        commands.push(Arc::from(command));
    }

    fn handle_extra_category(
        &self,
        extra_existing: &ExistingCategory,
        changes: &mut Vec<CategoryChange>,
    ) {
        changes.push(CategoryChange::Delete(extra_existing.clone()));
    }
}

pub struct KeepExtraCategories {}

impl ExtraCategoriesStrategy for KeepExtraCategories {
    fn _type(&self) -> ExtraCategoriesStrategyType {
        ExtraCategoriesStrategyType::Keep
    }

    fn handle_extra_category_commands(
        &self,
        _extra_existing: &ExistingCategory,
        _commands: &mut Vec<CommandRef>,
    ) {
    }

    fn handle_extra_category(
        &self,
        _extra_existing: &ExistingCategory,
        _changes: &mut Vec<CategoryChange>,
    ) {
    }
}
