use std::sync::Arc;

use crate::{core::events::ChangeEventListener, guild::GuildCommander};

pub trait Command {
    fn execute(&self, commander: &dyn GuildCommander, event_listener: &dyn ChangeEventListener);
}
pub type CommandRef = Arc<dyn Command>;
