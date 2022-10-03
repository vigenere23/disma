use std::sync::Arc;

use crate::domain::{
    diff::{
        category::{AddCategory, DeleteCategory, UpdateCategory},
        roles::{AddRole, DeleteRole, UpdateRole},
    },
    entities::guild::{AwaitingGuild, ExistingGuild},
};

use super::base::DiffCommandRef;

pub struct GuildDiffer {}
pub type GuildDifferRef = Arc<GuildDiffer>;

pub struct DiffCommands {
    pub role: Vec<DiffCommandRef>,
    pub category: Vec<DiffCommandRef>,
}

impl GuildDiffer {
    pub fn calculate_role_diffs(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

        for awaiting_role in awaiting_guild.roles.items() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(existing_role) => {
                    if awaiting_role != existing_role {
                        let command = UpdateRole::new(
                            existing_role.clone(),
                            awaiting_role.clone(),
                            existing_role.diffs_with(awaiting_role),
                        );
                        diffs.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddRole::new(awaiting_role.clone());
                    diffs.push(Arc::from(command));
                }
            }
        }

        for existing_role in existing_guild.roles.items() {
            if awaiting_guild
                .roles
                .find_by_name(&existing_role.name)
                .is_none()
            {
                let command = DeleteRole::new(existing_role.clone());
                diffs.push(Arc::from(command));
            }
        }

        diffs
    }

    pub fn calculate_category_diffs(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

        for awaiting_category in awaiting_guild.categories.items() {
            match existing_guild
                .categories
                .find_by_name(&awaiting_category.name)
            {
                Some(existing_category) => {
                    if awaiting_category != existing_category {
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

        for existing_category in existing_guild.categories.items() {
            if awaiting_guild
                .categories
                .find_by_name(&existing_category.name)
                .is_none()
            {
                let command = DeleteCategory::new(existing_category.clone());
                diffs.push(Arc::from(command));
            }
        }

        diffs
    }
}
