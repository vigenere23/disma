use disma::commands::{CommandDescription, CommandEventListener, CommandEventType};

pub struct CliCommandEventListener {}

impl CommandEventListener for CliCommandEventListener {
    fn handle(&self, event_type: CommandEventType, description: CommandDescription) {
        match event_type {
            CommandEventType::BeforeExecution => match description {
                CommandDescription::Create(entity, name) => {
                    print!("- 🆕 Adding {:?} {name}...", entity)
                }
                CommandDescription::Delete(entity, name) => {
                    print!("- 🗑️  Removing {:?} {name}...", entity)
                }
                CommandDescription::Update(entity, name, _diff) => {
                    print!("- 🔄 Updating {:?} {name}...", entity)
                }
            },
            CommandEventType::AfterExecution => println!("Done"),
        }
    }
}
