use std::sync::Arc;

use crate::{injector::MainInjector, utils::injector::Get};

use super::{
    injector::CliInjector,
    services::{
        apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
        save_guild::SaveExistingGuild,
    },
};

pub fn apply_changes(guild_id: &str, file_path: &str, dry_run: bool, force: bool) {
    let injector = CliInjector::new(MainInjector::new(Some(guild_id.to_string())));
    let service: Arc<ApplyChanges> = injector.get();
    service.run(guild_id, file_path, dry_run, force);
}

pub fn save_existing_guild(guild_id: &str, file_path: &str, force: bool) {
    let injector = CliInjector::new(MainInjector::new(Some(guild_id.to_string())));
    let service: Arc<SaveExistingGuild> = injector.get();
    service.run(guild_id, file_path, force);
}

pub fn list_guilds() {
    let injector = CliInjector::new(MainInjector::new(None));
    let service: Arc<ListGuilds> = injector.get();
    service.run();
}

pub fn compile_config(template_file: &str, vars_file: &str, output_file: &str, force: bool) {
    let injector = CliInjector::new(MainInjector::new(None));
    let service: Arc<CompileConfig> = injector.get();
    service.run(template_file, vars_file, output_file, force)
}
