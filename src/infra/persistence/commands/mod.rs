use std::sync::Arc;

use crate::domain::commands::{CommandRepository, GuildCommand};

pub struct InMemoryCommandRepository {
    items: Vec<Arc<dyn GuildCommand>>,
}

impl InMemoryCommandRepository {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

// impl CommandRepository for InMemoryCommandRepository {
//     fn find_all(&self) -> Vec<std::sync::Arc<dyn crate::domain::commands::GuildCommand>> {
//         self.items.into_iter().map(|item| item.clone()).collect()
//     }

//     fn save(&mut self, command: std::sync::Arc<dyn crate::domain::commands::GuildCommand>) {
//         self.items.push(command)
//     }
// }
