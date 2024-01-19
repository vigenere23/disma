mod actions;
mod args;
mod commands;
mod infra;
mod injector;
mod utils;

use clap::Parser;

use crate::{
    actions::{apply_changes, compile_config, list_guilds, save_existing_guild},
    args::{ArgParser, Command},
};

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
