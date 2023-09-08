use std::sync::Arc;

use crate::{
    category::{AwaitingCategory, CategoriesList, ExistingCategory},
    channel::{AwaitingChannel, ExistingChannel},
    role::{AwaitingRole, ExistingRole, RolesList},
};

#[cfg_attr(test, mock_it::mock_it)]
pub trait GuildCommander {
    fn add_role(&self, role: &AwaitingRole) -> Result<ExistingRole, String>;
    fn update_role(&self, id: &str, role: &AwaitingRole) -> Result<ExistingRole, String>;
    fn delete_role(&self, id: &str) -> Result<(), String>;
    fn add_category(
        &self,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) -> Result<ExistingCategory, String>;
    fn update_category(
        &self,
        id: &str,
        category: &AwaitingCategory,
        roles: &RolesList<ExistingRole>,
    ) -> Result<ExistingCategory, String>;
    fn delete_category(&self, id: &str) -> Result<(), String>;
    fn add_channel(
        &self,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Result<ExistingChannel, String>;
    fn update_channel(
        &self,
        id: &str,
        channel: &AwaitingChannel,
        roles: &RolesList<ExistingRole>,
        categories: &CategoriesList<ExistingCategory>,
    ) -> Result<ExistingChannel, String>;
    fn delete_channel(&self, id: &str) -> Result<(), String>;
}
pub type GuildCommanderRef = Arc<dyn GuildCommander>;
