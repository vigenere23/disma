use super::role::{AwaitingRole, RolesList};

pub trait GuildRepo {
    fn guild(&self) -> ExistingGuild;
}

pub trait AwaitingGuild {
    fn add_role(&self, role: AwaitingRole);
    fn update_role(&self, id: String, role: AwaitingRole);
}

#[derive(Debug)]
pub struct ExistingGuild {
    pub roles: RolesList,
}
