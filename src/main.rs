use std::io::{self, Write};

use crate::commands::Command;

mod commands;
mod exec;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading line: {e}");
        }

        let cmd = match Command::eval(&input) {
            Ok(v) => v,
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
