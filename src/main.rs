mod application;
mod domain;
mod infra;
mod injector;
mod interfaces;
mod utils;

use interfaces::cli::main::execute;

fn main() {
    execute()
}
