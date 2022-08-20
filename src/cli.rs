use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "\nDiscord as Code",
    about = "\nA tool to manage Discord settings as a config"
)]
pub struct ArgParser {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Save(SaveCommand),
    Apply(ApplyCommand),
}

#[derive(Debug, Args)]
#[clap(name = "Save existing guild config")]
pub struct SaveCommand {
    #[clap(short, long)]
    pub output: String,
}

#[derive(Debug, Args)]
#[clap(name = "Apply guild config")]
pub struct ApplyCommand {
    #[clap(short, long, help = "Input config file")]
    pub input: String,

    #[clap(long, help = "Do not execute any action")]
    pub dry_run: bool,

    #[clap(short, long, help = "Bypass user input confirmation")]
    pub force: bool,
}
