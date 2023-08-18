use disma::core::events::{Change, ChangeEvent, ChangeEventListener};

pub struct CliChangeEventListener {}

impl ChangeEventListener for CliChangeEventListener {
    fn handle(&self, event: ChangeEvent) {
        match event {
            ChangeEvent::Success(change) => match change {
                Change::Create(entity, name) => {
                    println!("- 🆕 Added {entity:?} {name}")
                }
                Change::Delete(entity, name) => {
                    println!("- 🗑️  Removed {entity:?} {name}")
                }
                Change::Update(entity, name) => {
                    println!("- 🔄 Updated {entity:?} {name}")
                }
            },
            ChangeEvent::Error(change, error) => match change {
                Change::Create(entity, name) => {
                    println!("- ❌ Failed to add {entity:?} {name}. Error : {error}")
                }
                Change::Delete(entity, name) => {
                    println!("- ❌ Failed to remove {entity:?} {name}. Error : {error}")
                }
                Change::Update(entity, name) => {
                    println!("- ❌ Failed to update {entity:?} {name}. Error : {error}")
                }
            },
        }
    }
}
