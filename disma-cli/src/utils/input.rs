use colored::Colorize;
use std::{
    io::{self, Write},
    process::exit,
};

pub fn ask_user_confirmation(message: &str) -> bool {
    print!("{}", format!("\nš”² ā {message} (y/N) ").bold());
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| panic!("{}", "š”² ā Unable to read user input".bold()));

    input.trim().to_lowercase() == "y"
}

pub fn abort() {
    println!("{}", "š”² ā ABORTED.".bold());
    exit(1);
}
