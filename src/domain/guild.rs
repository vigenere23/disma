use super::role::{AwaitingRole, RolesList};

pub trait GuildRepo {
    fn guild(&self) -> ExistingGuild;
}

pub trait AwaitingGuild {
    fn add_role(&self, role: &AwaitingRole);
    // fn update_role(&self, id: &str, role: AwaitingRole);
    fn delete_role(&self, id: &str);
}

#[derive(Debug)]
pub struct ExistingGuild {
    pub roles: RolesList,
}
