use std::sync::Arc;

use crate::guild::GuildCommanderRef;

pub trait DiffCommand {
    fn execute(&self, guild_commander: &GuildCommanderRef);
    fn describe(&self) -> EntityChange;
}
pub type DiffCommandRef = Arc<dyn DiffCommand>;

pub enum EntityChange {
    Create(Entity, String),
    Delete(Entity, String),
    Update(Entity, String, Vec<Diff>),
}

#[derive(Debug)]
pub enum Entity {
    Role,
    Category,
}

pub enum Diff {
    Add(String),
    Remove(String),
    Update(String, Vec<Diff>),
}
