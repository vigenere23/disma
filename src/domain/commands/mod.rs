use std::sync::Arc;

pub mod diff;
pub mod executor;
pub mod roles;

pub trait GuildCommand {
    fn execute(&self);
    fn describe(&self) -> String;
}

// TODO implementation more complicated than expected
pub trait CommandRepository {
    fn find_all(&self) -> Vec<Arc<dyn GuildCommand>>;
    fn save(&mut self, command: Arc<dyn GuildCommand>);
}
