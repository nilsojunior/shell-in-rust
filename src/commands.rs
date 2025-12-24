use crate::{errors::ShellError, exec};

use std::process;

pub enum Command {
    Exit,
    Pwd(),
    Echo(Vec<String>),
    Type(Vec<String>),
    Bin(String, Vec<String>),
    Cd(String),
}

impl Command {
    pub fn eval(input: &str) -> Result<Option<Self>, ShellError> {
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
                    Err(ShellError::NotEnoughArgs(bin.to_string()))
                } else {
                    Ok(Command::Echo(args))
                }
            }
            "type" => {
                if args.is_empty() {
                    Err(ShellError::NotEnoughArgs(bin.to_string()))
                } else {
                    Ok(Command::Type(args))
                }
            }
            "pwd" => Ok(Command::Pwd()),
            "cd" => {
                if args.is_empty() {
                    Err(ShellError::NotEnoughArgs(bin.to_string()))
                } else {
                    Ok(Command::Cd(args.iter().next().unwrap().to_string()))
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
                println!("{}", exec::type_cmd(&s[0])?);
            }
            Command::Bin(bin, args) => {
                exec::bin(bin, args)?;
            }
            Command::Pwd() => {
                println!("{}", exec::get_current_dir()?);
            }
            Command::Cd(s) => {
                exec::cd(s)?;
            }
        };

        Ok(())
    }
}
