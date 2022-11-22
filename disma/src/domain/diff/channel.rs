use crate::{
    category::CategoriesList,
    channel::{AwaitingChannel, Channel, ExistingChannel},
    diff::base::Entity,
    domain::entities::{
        category::ExistingCategory,
        role::{ExistingRole, RolesList},
    },
    guild::GuildCommanderRef,
};

use super::base::{DiffCommand, Differ, EntityChange};

pub struct AddChannel {
    channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
}

impl AddChannel {
    pub fn new(
        channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Self {
        Self {
            channel,
            roles,
            categories,
        }
    }
}

impl DiffCommand for AddChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.add_channel(&self.channel, &self.roles, &self.categories);
    }

    fn describe(&self) -> EntityChange {
        EntityChange::Create(Entity::Channel, self.channel.unique_name())
    }
}

pub struct UpdateChannel {
    existing_channel: ExistingChannel,
    awaiting_channel: AwaitingChannel,
    roles: RolesList<ExistingRole>,
    categories: CategoriesList<ExistingCategory>,
}

impl UpdateChannel {
    pub fn new(
        existing_channel: ExistingChannel,
        awaiting_channel: AwaitingChannel,
        roles: RolesList<ExistingRole>,
        categories: CategoriesList<ExistingCategory>,
    ) -> Self {
        Self {
            existing_channel,
            awaiting_channel,
            roles,
            categories,
        }
    }
}

impl DiffCommand for UpdateChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.update_channel(
            &self.existing_channel.id,
            &self.awaiting_channel,
            &self.roles,
            &self.categories,
        );
    }

    fn describe(&self) -> EntityChange {
        EntityChange::Update(
            Entity::Channel,
            self.existing_channel.unique_name(),
            self.existing_channel.diffs_with(&self.awaiting_channel),
        )
    }
}

pub struct DeleteChannel {
    channel: ExistingChannel,
}

impl DeleteChannel {
    pub fn new(channel: ExistingChannel) -> Self {
        Self { channel }
    }
}

impl DiffCommand for DeleteChannel {
    fn execute(&self, guild: &GuildCommanderRef) {
        guild.delete_category(&self.channel.id);
    }

    fn describe(&self) -> EntityChange {
        EntityChange::Delete(Entity::Channel, self.channel.unique_name())
    }
}
