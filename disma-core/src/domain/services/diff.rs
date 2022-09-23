use std::sync::Arc;

use crate::domain::{
    commands::{
        category::{AddCategory, DeleteCategory, UpdateCategory},
        roles::{AddRole, DeleteRole, UpdateRole},
        GuildCommand,
    },
    entities::guild::{AwaitingGuild, ExistingGuild},
};

pub struct DiffCalculator {}

impl DiffCalculator {
    pub fn create_role_commands(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<Arc<dyn GuildCommand>> {
        let mut commands: Vec<Arc<dyn GuildCommand>> = Vec::new();

        for awaiting_role in awaiting_guild.roles.items() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(role) => {
                    if awaiting_role != role {
                        let command = UpdateRole::new(role.clone(), awaiting_role.clone());
                        commands.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddRole::new(awaiting_role.clone());
                    commands.push(Arc::from(command));
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
                commands.push(Arc::from(command));
            }
        }

        commands
    }

    pub fn create_category_commands(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<Arc<dyn GuildCommand>> {
        let mut commands: Vec<Arc<dyn GuildCommand>> = Vec::new();

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
                        commands.push(Arc::from(command));
                    }
                }
                None => {
                    let command =
                        AddCategory::new(awaiting_category.clone(), existing_guild.roles.clone());
                    commands.push(Arc::from(command));
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
                commands.push(Arc::from(command));
            }
        }

        commands
    }
}
