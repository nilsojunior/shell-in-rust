#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading line: {e}");
        }

        match input.trim() {
            "exit" => process::exit(0),
            _ => {
                println!("{}: command not found", input.trim());
            }
        };
    }
}
