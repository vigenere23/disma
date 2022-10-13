use disma::diff::{base::EntityChange, event::DiffEventListener};

pub struct CliDiffEventListener {}

impl DiffEventListener for CliDiffEventListener {
    fn after_change_executed(&self, change: EntityChange) {
        match change {
            EntityChange::Create(entity, name) => println!("- 🆕 Added {:?} {name}", entity),
            EntityChange::Delete(entity, name) => println!("- 🗑️  Removed {:?} {name}", entity),
            EntityChange::Update(entity, name, _) => println!("- 🔄 Updated {:?} {name}", entity),
        }
    }
}
