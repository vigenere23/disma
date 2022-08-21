use super::role::{AwaitingRole, ExistingRole, RolesList};

pub trait GuildQuerier {
    fn get_guild(&self, guild_id: &str) -> ExistingGuild;
    fn list_guilds(&self) -> Vec<GuildSummary>;
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

pub struct GuildSummary {
    pub name: String,
    pub id: String,
}
