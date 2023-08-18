use crate::domain::diff::Diff;

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
