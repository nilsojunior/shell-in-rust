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
            input if input.starts_with("echo") => {
                let args = parse_args(input.trim());
                for arg in args {
                    print!("{arg} ");
                }
                println!();
            }
            cmd => eprintln!("{cmd}: command not found"),
        };
    }
}

fn parse_args(input: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut input = input.split_whitespace();
    input.next();

    for arg in input {
        v.push(arg.to_string());
    }

    v
}
