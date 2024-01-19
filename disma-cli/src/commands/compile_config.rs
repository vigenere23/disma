use colored::Colorize;
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
    pub fn new(deserializer: Arc<Deserializer>) -> Self {
        Self { deserializer }
    }

    pub fn run(&self, template_file: &str, vars_file: &str, output_file: &str, force: bool) {
        let template = fs::read_to_string(template_file).unwrap();
        // TODO context should probably be a Hash instead of a serde_yaml::Value
        let context: Value = self.deserializer.deserialize(Path::new(vars_file));

        println!();
        println!("{}", "➜ ⚙️ Compiling guild config...".bold());
        let mut renderer = Handlebars::new();
        renderer.register_escape_fn(|s| s.to_string()); // preventing default HTML escaping

        let rendered = renderer.render_template(&template, &context).unwrap();

        let output_path = Path::new(output_file);
        println!(
            "{}",
            format!(
                "➜ 💾 Saving compiled guild config to '{}'...",
                output_path.as_os_str().to_str().unwrap()
            )
            .bold()
        );

        if !force && output_path.exists() {
            println!(
                "{}",
                format!("➜ ❗ A file named '{output_file}' already exists.").bold()
            );

            if !ask_user_confirmation("Do you still want to proceeed?") {
                abort();
            }
        }

        fs::write(output_file, rendered).unwrap();

        println!("{}", "➜ ✨ DONE.".bold());
    }
}
