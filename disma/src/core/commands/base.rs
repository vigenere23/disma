use std::sync::Arc;

use crate::{
    core::events::ChangeEventListener,
    guild::{ExistingGuild, GuildCommander},
};

pub trait Command {
    fn execute(
        &self,
        commander: &dyn GuildCommander,
        event_listener: &dyn ChangeEventListener,
        existing_guild: &mut ExistingGuild,
    );
}
pub type CommandRef = Arc<dyn Command>;
