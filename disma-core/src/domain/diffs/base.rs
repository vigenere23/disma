use std::sync::Arc;

use crate::guild::GuildCommanderRef;

pub trait Diff {
    fn execute(&self, guild_commander: GuildCommanderRef);
    fn describe(&self) -> DiffDescription;
}
pub type DiffRef = Arc<dyn Diff>;

pub struct DiffDescription {
    pub summary: String,
    pub details: Vec<Change>,
}

pub enum Change {
    Add(ChangeData),
    Remove(ChangeData),
}

pub struct ChangeData {
    pub property: String,
    pub value: String,
}
