use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "Discord as Code", about)]
pub struct ArgParser {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about = "Save existing guild to config")]
    Save(SaveArgs),
    #[clap(about = "Apply guild changes from config")]
    Apply(ApplyArgs),
    #[clap(name = "list", about = "List guilds accessible by bot")]
    ListGuilds,
}

#[derive(Debug, Args)]
pub struct SaveArgs {
    #[clap(short, long, help = "Guild ID")]
    pub guild: String,

    #[clap(short, long)]
    pub output: String,

    #[clap(short, long, help = "Bypass user input confirmation")]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct ApplyArgs {
    #[clap(short, long, help = "Guild ID")]
    pub guild: String,

    #[clap(short, long, help = "Input config file")]
    pub input: String,

    #[clap(long, help = "Do not execute any action")]
    pub dry_run: bool,

    #[clap(short, long, help = "Bypass user input confirmation")]
    pub force: bool,
}
