use std::sync::Arc;

use crate::domain::{
    diffs::{
        category::{AddCategory, DeleteCategory, UpdateCategory},
        roles::{AddRole, DeleteRole, UpdateRole},
    },
    entities::guild::{AwaitingGuild, ExistingGuild},
};

use super::base::DiffRef;

pub struct GuildDiffer {}
pub type GuildDifferRef = Arc<GuildDiffer>;

impl GuildDiffer {
    pub fn calculate_diffs(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffRef> {
        let role_diffs = self.calculate_role_diffs(existing_guild, awaiting_guild);
        let category_diffs = self.calculate_category_diffs(existing_guild, awaiting_guild);

        role_diffs
            .into_iter()
            .chain(category_diffs.into_iter())
            .collect()
    }

    fn calculate_role_diffs(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffRef> {
        let mut diffs: Vec<DiffRef> = Vec::new();

        for awaiting_role in awaiting_guild.roles.items() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(role) => {
                    if awaiting_role != role {
                        let command = UpdateRole::new(role.clone(), awaiting_role.clone());
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

    fn calculate_category_diffs(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffRef> {
        let mut diffs: Vec<DiffRef> = Vec::new();

        for awaiting_category in awaiting_guild.categories.items() {
            match existing_guild
                .categories
                .find_by_name(&awaiting_category.name)
            {
                Some(category) => {
                    if awaiting_category != category {
                        let command = UpdateCategory::new(
                            category.clone(),
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
