use std::io::{self, Write};

use crate::commands::Command;

mod commands;
mod errors;
mod exec;

fn parse_args(input: &str) -> (&str, &str) {
    let input = input.trim();
    input.split_once(" ").unwrap_or((input, ""))
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading line: {e}");
        }

        let (bin, args) = parse_args(&input);

        if bin.is_empty() {
            continue;
        }

        let cmd = match Command::eval(&bin, &args) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        match cmd.handle() {
            Ok(s) => s,
            Err(e) => eprintln!("{e}"),
        }
    }
}
