use std::{fs, path::Path, sync::Arc};

use handlebars::Handlebars;
use serde_yaml::Value;

use crate::utils::{
    input::{abort, ask_user_confirmation},
    io::Deserializer,
};

pub struct CompileConfig {
    deserializer: Arc<Deserializer>,
}

impl CompileConfig {
    const TEMPLATE_NAME: &'static str = "base";

    pub fn new(deserializer: Arc<Deserializer>) -> Self {
        Self { deserializer }
    }

    pub fn run(&self, template_file: &str, vars_file: &str, output_file: &str, force: bool) {
        let template = fs::read_to_string(template_file).unwrap();
        let context: Value = self.deserializer.deserialize(Path::new(vars_file));

        println!("⚙️ Compiling guild config...");
        let mut renderer = Handlebars::new();
        renderer
            .register_template_string(Self::TEMPLATE_NAME, &template)
            .unwrap();

        let rendered = renderer.render(Self::TEMPLATE_NAME, &context).unwrap();

        let output_path = Path::new(output_file);
        println!(
            "\n💾 Saving compiled guild config to '{}'...",
            output_path.as_os_str().to_str().unwrap()
        );

        if !force && output_path.exists() {
            println!("A file named '{}' already exists.", output_file);

            if !ask_user_confirmation() {
                abort();
            }
        }

        fs::write(output_file, &rendered).unwrap();

        println!("\n✨ DONE.");
    }
}
