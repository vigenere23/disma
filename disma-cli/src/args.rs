use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "Disma", about)]
pub struct ArgParser {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(name = "save", about = "Save existing guild to config")]
    Save(SaveArgs),

    #[clap(name = "apply", about = "Apply guild changes from config")]
    Apply(ApplyArgs),

    #[clap(name = "list", about = "List guilds accessible by bot")]
    ListGuilds,

    #[clap(name = "compile", about = "Compile YAML config")]
    CompileConfig(CompileConfigArgs),
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

#[derive(Debug, Args)]
pub struct CompileConfigArgs {
    #[clap(short, long, help = "Input config template file")]
    pub template: String,

    #[clap(short, long, help = "Template variables")]
    pub vars: String,

    #[clap(short, long, help = "Compiled config output path")]
    pub output: String,

    #[clap(short, long, help = "Bypass user input confirmation")]
    pub force: bool,
}
