use std::sync::Arc;

use crate::{domain::diff::Diff, guild::GuildCommanderRef};

pub trait Command {
    fn execute(&self, guild_commander: &GuildCommanderRef);
    fn describe(&self) -> CommandDescription;
}
pub type CommandRef = Arc<dyn Command>;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandDescription {
    Create(CommandEntity, CommandEntityName),
    Delete(CommandEntity, CommandEntityName),
    Update(CommandEntity, CommandEntityName, Vec<Diff>),
}

type CommandEntityName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandEntity {
    Role,
    Category,
    Channel,
}
