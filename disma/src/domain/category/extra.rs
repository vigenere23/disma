use core::fmt::Debug;

use crate::{category::ExistingCategory, core::changes::category::CategoryChange};

pub trait ExtraCategoriesStrategy {
    fn _type(&self) -> ExtraCategoriesStrategyType;
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

    fn handle_extra_category(
        &self,
        _extra_existing: &ExistingCategory,
        _changes: &mut Vec<CategoryChange>,
    ) {
    }
}
