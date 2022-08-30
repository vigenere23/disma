mod application;
mod cli;
mod domain;
mod infra;
mod injector;
mod utils;

use std::sync::Arc;

use application::{
    apply_changes::ApplyChanges, compile_config::CompileConfig, list_guilds::ListGuilds,
    save_guild::SaveExistingGuild,
};
use clap::Parser;
use injector::{Get, Injector};

use crate::cli::{ArgParser, Command};

fn main() {
    let args = ArgParser::parse();

    match &args.command {
        Command::Save(args) => save_existing_guild(&args.guild, &args.output, args.force),
        Command::Apply(args) => apply_changes(&args.guild, &args.input, args.dry_run, args.force),
        Command::ListGuilds => list_guilds(),
        Command::CompileConfig(args) => {
            compile_config(&args.template, &args.vars, &args.output, args.force)
        }
    }
}

fn apply_changes(guild_id: &str, file_path: &str, dry_run: bool, force: bool) {
    let injector = Injector::new(Some(guild_id.to_string()));
    let service: Arc<ApplyChanges> = injector.get();
    service.run(guild_id, file_path, dry_run, force);
}

fn save_existing_guild(guild_id: &str, file_path: &str, force: bool) {
    let injector = Injector::new(Some(guild_id.to_string()));
    let service: Arc<SaveExistingGuild> = injector.get();
    service.run(guild_id, file_path, force);
}

fn list_guilds() {
    let injector = Injector::new(None);
    let service: Arc<ListGuilds> = injector.get();
    service.run();
}

fn compile_config(template_file: &str, vars_file: &str, output_file: &str, force: bool) {
    let injector = Injector::new(None);
    let service: Arc<CompileConfig> = injector.get();
    service.run(template_file, vars_file, output_file, force)
}
