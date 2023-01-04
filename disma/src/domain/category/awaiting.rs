use crate::{
    channel::ExtraChannelsOptions, overwrites::PermissionsOverwritesList, role::AwaitingRole,
};

use super::{CategoriesList, Category};

#[derive(Clone, Debug, PartialEq)]
pub struct AwaitingCategoriesList {
    pub items: CategoriesList<AwaitingCategory>,
    pub extra_items: ExtraCategoriesOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtraCategoriesOptions {
    pub strategy: ExtraCategoriesStrategy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtraCategoriesStrategy {
    Keep,
    Remove,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AwaitingCategory {
    pub name: String,
    pub overwrites: PermissionsOverwritesList<AwaitingRole>,
    pub extra_channels: ExtraChannelsOptions,
}

impl ToString for AwaitingCategory {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Category for AwaitingCategory {
    fn name(&self) -> String {
        self.name.clone()
    }
}
