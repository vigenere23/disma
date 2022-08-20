#![allow(dead_code)]

mod application;
mod cli;
mod domain;
mod infra;
mod injector;
mod utils;
use std::sync::Arc;

use application::{apply_changes::ApplyChanges, save_guild::SaveExistingGuild};
use clap::Parser;
use injector::{Get, Injector};

use crate::cli::{ArgParser, Command};

fn main() {
    let args = ArgParser::parse();
    let injector = Injector::new();

    match &args.command {
        Command::Save(args) => save_existing_guild(&injector, &args.output),
        Command::Apply(args) => apply_changes(&injector, &args.input, args.dry_run, args.force),
    }

    // apply_changes(injector, true, false);
    //load_existing_guild(injector);
}

fn apply_changes(injector: &Injector, file_path: &str, dry_run: bool, force: bool) {
    let service: Arc<ApplyChanges> = injector.get();
    service.run(&file_path, dry_run, force);
}

fn save_existing_guild(injector: &Injector, file_path: &str) {
    let service: Arc<SaveExistingGuild> = injector.get();
    service.run(&file_path);
}
