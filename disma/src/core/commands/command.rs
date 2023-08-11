use crate::{core::events::ChangeEventListenerRef, guild::GuildCommanderRef};

pub trait Command {
    fn execute(&self, commander: &GuildCommanderRef, event_listener: &ChangeEventListenerRef);
}
