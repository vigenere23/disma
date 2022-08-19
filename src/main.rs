#![allow(dead_code)]

mod application;
mod domain;
mod infra;
mod injector;
mod utils;
use std::sync::Arc;

use application::{apply_changes::ApplyChanges, save_guild::SaveExistingGuild};
use injector::{Get, Injector};

fn main() {
    let injector = Injector::new("config.json");
    apply_changes(injector, true, false);
    //load_existing_guild(injector);
}

fn apply_changes(injector: Injector, dry_run: bool, force: bool) {
    let service: Arc<ApplyChanges> = injector.get();
    service.run(dry_run, force);
}

fn load_existing_guild(injector: Injector) {
    let service: Arc<SaveExistingGuild> = injector.get();
    service.run();
}
