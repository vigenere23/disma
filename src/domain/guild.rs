use super::{
    category::{AwaitingCategory, ExistingCategory},
    role::{AwaitingRole, ExistingRole, RolesList},
};

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
    pub categories: Vec<ExistingCategory>,
    // pub channels: Vec<ExistingChannel>,
}

pub struct AwaitingGuild {
    pub roles: RolesList<AwaitingRole>,
    pub categories: Vec<AwaitingCategory>,
}

pub struct GuildSummary {
    pub name: String,
    pub id: String,
}
