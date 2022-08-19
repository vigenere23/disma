pub mod roles;

pub trait GuildCommand {
    fn execute(&self);
    fn describe(&self) -> String;
}
