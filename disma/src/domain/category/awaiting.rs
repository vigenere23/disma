use std::sync::Arc;

use crate::{channel::ExtraChannelsStrategy, permission::PermissionsOverwritesList};

use super::{CategoriesList, Category, ExtraCategoriesStrategy};

#[derive(Clone, Debug)]
pub struct AwaitingCategoriesList {
    pub items: CategoriesList<AwaitingCategory>,
    pub extra_items_strategy: Arc<dyn ExtraCategoriesStrategy>,
}

impl PartialEq for AwaitingCategoriesList {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
            && self.extra_items_strategy._type() == other.extra_items_strategy._type()
    }
}

#[derive(Clone, Debug)]
pub struct AwaitingCategory {
    pub name: String,
    pub overwrites: PermissionsOverwritesList,
    pub extra_channels_strategy: Arc<dyn ExtraChannelsStrategy>,
}

impl PartialEq for AwaitingCategory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.overwrites == other.overwrites
            && self.extra_channels_strategy._type() == other.extra_channels_strategy._type()
    }
}

impl ToString for AwaitingCategory {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Category for AwaitingCategory {
    fn name(&self) -> &str {
        &self.name
    }
}
