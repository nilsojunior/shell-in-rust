use crate::exec;

use std::process;

pub enum Command {
    Exit,
    Echo(String),
    Type(String),
}

impl Command {
    pub fn eval(input: &str) -> Result<Self, String> {
        let input: Vec<&str> = input.split_whitespace().collect();

        match input[0] {
            "exit" => Ok(Command::Exit),
            "echo" => {
                if input.len() < 2 {
                    Err(format!("{}: requires a argument", input[0]))
                } else {
                    Ok(Command::Echo(input[1..].join(" ")))
                }
            }
            "type" => {
                if input.len() < 2 {
                    Err(format!("{}: requires a argument", input[0]))
                } else {
                    Ok(Command::Type(input[1].to_string()))
                }
            }
            _ => Err(format!("{}: command not found", input[0])),
        }
    }

    pub fn handle(&self) -> Result<(), String> {
        match self {
            Command::Exit => {
                process::exit(0);
            }
            Command::Echo(s) => {
                println!("{s}");
            }
            Command::Type(s) => {
                println!("{}", exec::type_cmd(s)?);
            }
        };

        Ok(())
    }
}
