use crate::{errors::ShellError, exec};

use std::process;

#[derive(Debug)]
pub enum Command {
    Exit,
    Pwd(),
    Echo(String),
    Type(String),
    Bin(String, String),
    Cd(String),
}

impl Command {
    pub fn eval(bin: &str, args: &str) -> Result<Self, ShellError> {
        let args = args.to_string();
        let bin = bin.to_string();
        match bin.as_str() {
            "exit" => Ok(Self::Exit),
            "pwd" => Ok(Self::Pwd()),
            "echo" => {
                if args.is_empty() {
                    Err(ShellError::NotEnoughArgs(bin))
                } else {
                    Ok(Self::Echo(args))
                }
            }
            "type" => {
                if args.is_empty() {
                    Err(ShellError::NotEnoughArgs(bin))
                } else {
                    Ok(Self::Type(args))
                }
            }
            "cd" => {
                if args.is_empty() {
                    Err(ShellError::NotEnoughArgs(bin))
                } else {
                    Ok(Self::Cd(args))
                }
            }
            _ => Ok(Self::Bin(bin, args)),
        }
    }

    pub fn handle(&self) -> Result<(), ShellError> {
        match self {
            Self::Exit => {
                process::exit(0);
            }
            Self::Echo(s) => {
                exec::echo(s);
            }
            Self::Type(s) => {
                println!("{}", exec::type_cmd(s)?);
            }
            Self::Bin(bin, args) => {
                exec::bin(bin, args)?;
            }
            Self::Pwd() => {
                println!("{}", exec::get_current_dir()?);
            }
            Self::Cd(s) => {
                exec::cd(s)?;
            }
        };

        Ok(())
    }
}
