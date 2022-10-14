use disma::diff::{base::EntityChange, event::DiffEventListener};

pub struct CliDiffEventListener {}

impl DiffEventListener for CliDiffEventListener {
    fn before_change_executed(&self, change: EntityChange) {
        match change {
            EntityChange::Create(entity, name) => print!("- 🆕 Adding {:?} {name}...", entity),
            EntityChange::Delete(entity, name) => print!("- 🗑️  Removing {:?} {name}...", entity),
            EntityChange::Update(entity, name, _diff) => {
                print!("- 🔄 Updating {:?} {name}...", entity)
            }
        }
    }

    fn after_change_executed(&self, _change: EntityChange) {
        println!("Done")
    }
}
