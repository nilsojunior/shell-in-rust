use crate::{errors::ShellError, exec};

use std::process;

pub enum Command {
    Exit,
    Echo(Vec<String>),
    Type(Vec<String>),
    Bin(String, Vec<String>),
}

impl Command {
    pub fn eval(input: &str) -> Result<Option<Self>, String> {
        let mut input = input.split_whitespace();

        let bin = match input.next() {
            Some(v) => v,
            None => return Ok(None),
        };

        // Call String::from on every string
        let args: Vec<String> = input.map(String::from).collect();

        let cmd = match bin {
            "exit" => Ok(Command::Exit),
            "echo" => {
                if args.is_empty() {
                    Err(format!("{bin}: requires a argument"))
                } else {
                    Ok(Command::Echo(args))
                }
            }
            "type" => {
                if args.is_empty() {
                    Err(format!("{bin}: requires a argument"))
                } else {
                    Ok(Command::Type(args))
                }
            }
            _ => Ok(Command::Bin(bin.to_string(), args)),
        };

        Ok(Some(cmd?))
    }

    pub fn handle(&self) -> Result<(), ShellError> {
        match self {
            Command::Exit => {
                process::exit(0);
            }
            Command::Echo(s) => {
                println!("{}", s.join(" "))
            }
            Command::Type(s) => {
                println!("{}", exec::find_executable_in_path(&s[0])?);
            }
            Command::Bin(bin, args) => {
                exec::bin(bin, args)?;
            }
        };

        Ok(())
    }
}
