use disma::commands::{CommandDescription, CommandEventListener, CommandEventType};

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
