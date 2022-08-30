pub mod apply_changes;
pub mod compile_config;
pub mod list_guilds;
pub mod save_guild;

pub trait ApplicationCommand {
    fn run(&self);
}
