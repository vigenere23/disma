use disma::commands::{CommandDescription, CommandEventListener};

pub struct CliCommandEventListener {}

impl CommandEventListener for CliCommandEventListener {
    fn before_command_execution(&self, description: CommandDescription) {
        match description {
            CommandDescription::Create(entity, name) => {
                print!("- 🆕 Adding {:?} {name}...", entity)
            }
            CommandDescription::Delete(entity, name) => {
                print!("- 🗑️  Removing {:?} {name}...", entity)
            }
            CommandDescription::Update(entity, name, _diff) => {
                print!("- 🔄 Updating {:?} {name}...", entity)
            }
        }
    }

    fn after_command_execution(&self, _: CommandDescription) {
        println!("Done")
    }
}
