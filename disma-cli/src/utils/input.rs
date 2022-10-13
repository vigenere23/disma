use colored::Colorize;
use std::{
    io::{self, Write},
    process::exit,
};

pub fn ask_user_confirmation(message: &str) -> bool {
    print!("{}", format!("\n🡲 ❔ {message} (y/N) ").bold());
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap_or_else(|_| panic!("{}", "🡲 ❌ Unable to read user input".bold()));

    input.trim().to_lowercase() == "y"
}

pub fn abort() {
    println!("{}", "🡲 ❌ ABORTED.".bold());
    exit(1);
}
