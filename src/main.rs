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

        let (bin, args) = parse_input(&input);

        let cmds = ["exit", "echo", "type"];

        match bin {
            "exit" => return,
            "echo" => {
                for arg in args {
                    print!("{arg} ");
                }
                println!();
            }
            "type" => {
                let cmd = args[0];
                if cmds.contains(&cmd) {
                    println!("{cmd} is a shell builtin");
                } else {
                    println!("{cmd}: not found");
                }
            }
            cmd => eprintln!("{cmd}: command not found"),
        };
    }
}

fn parse_input(input: &str) -> (&str, Vec<&str>) {
    let mut input = input.split_whitespace();
    let bin = input.next().unwrap();

    (bin, input.collect())
}
