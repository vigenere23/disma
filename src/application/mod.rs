pub mod apply_changes;
pub mod save_guild;

pub trait ApplicationCommand {
    fn run(&self);
}
