use serde::{Deserialize, Serialize};

use crate::domain::category::ExistingCategory;

#[derive(Serialize, Deserialize)]
pub struct CategoryConfig {
    pub name: String,
}

impl From<&ExistingCategory> for CategoryConfig {
    fn from(category: &ExistingCategory) -> Self {
        Self {
            name: category.name.clone(),
        }
    }
}
