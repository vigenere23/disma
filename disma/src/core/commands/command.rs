use std::sync::Arc;

use crate::{core::events::ChangeEventListenerRef, guild::GuildCommanderRef};

pub trait Command {
    fn execute(&self, commander: &GuildCommanderRef, event_listener: &ChangeEventListenerRef);
}
pub type CommandRef = Arc<dyn Command>;
