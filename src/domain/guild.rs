use super::role::{AwaitingRole, ExistingRole, RolesList};

pub trait GuildQuerier {
    fn guild(&self) -> ExistingGuild;
}

pub trait GuildCommander {
    fn add_role(&self, role: &AwaitingRole);
    fn update_role(&self, id: &str, role: &AwaitingRole);
    fn delete_role(&self, id: &str);
}

#[derive(Debug)]
pub struct ExistingGuild {
    pub roles: RolesList<ExistingRole>,
}

pub struct AwaitingGuild {
    pub roles: RolesList<AwaitingRole>,
}
