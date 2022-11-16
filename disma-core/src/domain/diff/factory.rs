use std::sync::Arc;

use crate::{
    channel::Channel,
    domain::{
        diff::{
            category::{AddCategory, DeleteCategory, UpdateCategory},
            roles::{AddRole, DeleteRole, UpdateRole},
        },
        entities::guild::{AwaitingGuild, ExistingGuild},
    },
};

use super::{
    base::{DiffCommandRef, Differ},
    channel::{AddChannel, DeleteChannel, UpdateChannel},
};

pub struct DiffCommandFactory {}
pub type DiffCommandFactoryRef = Arc<DiffCommandFactory>;

impl DiffCommandFactory {
    pub fn for_roles(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

        for awaiting_role in awaiting_guild.roles.items() {
            match existing_guild.roles.find_by_name(&awaiting_role.name) {
                Some(existing_role) => {
                    if existing_role != awaiting_role {
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

    pub fn for_categories(
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
                    if existing_category != awaiting_category {
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

    pub fn for_channels(
        &self,
        existing_guild: &ExistingGuild,
        awaiting_guild: &AwaitingGuild,
    ) -> Vec<DiffCommandRef> {
        let mut diffs: Vec<DiffCommandRef> = Vec::new();

        for awaiting_channel in awaiting_guild.channels.items() {
            let category = awaiting_channel
                .category
                .as_ref()
                .map(|category| existing_guild.categories.find_by_name_panic(&category.name));

            match existing_guild.channels.find(
                &awaiting_channel.name,
                awaiting_channel.channel_type(),
                category,
            ) {
                Some(existing_channel) => {
                    if existing_channel != awaiting_channel {
                        let command = UpdateChannel::new(
                            existing_channel.clone(),
                            awaiting_channel.clone(),
                            existing_guild.roles.clone(),
                            existing_guild.categories.clone(),
                        );
                        diffs.push(Arc::from(command));
                    }
                }
                None => {
                    let command = AddChannel::new(
                        awaiting_channel.clone(),
                        existing_guild.roles.clone(),
                        existing_guild.categories.clone(),
                    );
                    diffs.push(Arc::from(command));
                }
            }
        }

        for existing_channel in existing_guild.channels.items() {
            let category = existing_channel
                .category
                .as_ref()
                .map(|category| existing_guild.categories.find_by_name_panic(&category.name));

            if awaiting_guild
                .channels
                .find(
                    &existing_channel.name,
                    existing_channel.channel_type(),
                    category,
                )
                .is_none()
            {
                let command = DeleteChannel::new(existing_channel.clone());
                diffs.push(Arc::from(command));
            }
        }

        diffs
    }
}
