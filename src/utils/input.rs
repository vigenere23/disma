use std::{
    io::{self, Write},
    process::exit,
};

pub fn ask_user_confirmation() -> bool {
    print!("\n❔ Do you still want to proceeed? (y/N) ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Unable to read user input");

    input.trim().to_lowercase() == "y"
}

pub fn abort() {
    println!("❌ ABORTED.");
    exit(1);
}
