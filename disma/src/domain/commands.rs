use std::sync::Arc;

use crate::{
    domain::diff::Diff,
    guild::{ExistingGuild, GuildCommanderRef},
};

pub trait Command {
    fn execute(&self, guild_commander: &GuildCommanderRef);
    fn describe(&self) -> CommandDescription;
}
pub type CommandRef = Arc<dyn Command>;

pub trait CommandFactory {
    fn commands_for(&self, existing_guild: &ExistingGuild) -> Vec<CommandRef>;
}

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

#[derive(Debug, PartialEq)]
pub enum CommandEventType {
    BeforeExecution,
    AfterExecution,
}

#[cfg_attr(test, mock_it::mock_it)]
pub trait CommandEventListener {
    fn handle(&self, event_type: CommandEventType, description: CommandDescription);
}
pub type CommandEventListenerRef = Arc<dyn CommandEventListener>;

pub struct NullCommandEventListener {}

impl CommandEventListener for NullCommandEventListener {
    fn handle(&self, _event_type: CommandEventType, _description: CommandDescription) {}
}
