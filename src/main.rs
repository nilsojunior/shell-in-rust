#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading line: {e}");
        }

        match input.trim() {
            "exit" => return,
            cmd => eprintln!("{cmd}: command not found"),
        };
    }
}
