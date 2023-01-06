use std::sync::Arc;

use crate::{
    channel::ExtraChannelsStrategy, overwrites::PermissionsOverwritesList, role::AwaitingRole,
};

use super::{CategoriesList, Category, ExtraCategoriesStrategy};

#[derive(Clone, Debug)]
pub struct AwaitingCategoriesList {
    pub items: CategoriesList<AwaitingCategory>,
    pub extra_items_strategy: Arc<dyn ExtraCategoriesStrategy>,
}

#[derive(Clone, Debug)]
pub struct AwaitingCategory {
    pub name: String,
    pub overwrites: PermissionsOverwritesList<AwaitingRole>,
    pub extra_channels_strategy: Arc<dyn ExtraChannelsStrategy>,
}

impl PartialEq for AwaitingCategory {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.overwrites.eq(&other.overwrites)
    }
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
