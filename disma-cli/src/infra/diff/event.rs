use disma::{
    commands::{CommandDescription, CommandEventListener, CommandEventType},
    core::events::{Change, ChangeEvent, ChangeEventListener},
};

pub struct CliCommandEventListener {}

impl CommandEventListener for CliCommandEventListener {
    fn handle(&self, event_type: CommandEventType, description: CommandDescription) {
        match event_type {
            CommandEventType::BeforeExecution => match description {
                CommandDescription::Create(entity, name) => {
                    print!("- 🆕 Adding {entity:?} {name}...")
                }
                CommandDescription::Delete(entity, name) => {
                    print!("- 🗑️  Removing {entity:?} {name}...")
                }
                CommandDescription::Update(entity, name, _diff) => {
                    print!("- 🔄 Updating {entity:?} {name}...")
                }
            },
            CommandEventType::AfterExecution => println!("Done"),
        }
    }
}

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
