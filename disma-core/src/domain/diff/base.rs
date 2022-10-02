use std::sync::Arc;

use crate::guild::GuildCommanderRef;

pub trait DiffCommand {
    fn execute(&self, guild_commander: &GuildCommanderRef);
    fn describe(&self) -> Diff;
}
pub type DiffCommandRef = Arc<dyn DiffCommand>;

pub enum Diff {
    Add(String),
    Remove(String),
    Update(String, Vec<Diff>),
}
