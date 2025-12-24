use ::std::{env, ffi, process};
use std::{os::unix::fs::PermissionsExt, path::PathBuf};

use crate::errors::ShellError;

pub fn find_executable_in_path(s: &str) -> Result<String, ShellError> {
    let bin = ffi::OsStr::new(s);
    let key = "PATH";
    let paths = env::var_os(key).ok_or_else(|| ShellError::EnvVarNotSet(key.to_string()))?;

    let result = String::new();

    let entries = env::split_paths(&paths);
    for entry in entries {
        let path = entry.join(bin);
        // Check here
        if is_executable(&path) {
            // We check the str before so this is safe
            // result.push_str(&format!("{s} is {}\n", path.to_str().unwrap())); // Safe unwrap
            return Ok(path.to_str().unwrap().to_string());
        }
    }

    if result.is_empty() {
        Err(ShellError::NotFound(s.to_string()))
    } else {
        Ok(result)
    }
}

const BUILTIN_COMMANDS: [&str; 4] = ["exit", "echo", "type", "pwd"];

pub fn type_cmd(s: &str) -> Result<String, ShellError> {
    if BUILTIN_COMMANDS.contains(&s) {
        return Ok(format!("{s} is a shell builtin"));
    }

    Ok(format!("{s} is {}", find_executable_in_path(s)?))
}

fn is_executable(path: &PathBuf) -> bool {
    let path = match path.metadata() {
        Ok(data) => data,
        Err(_) => return false,
    };

    if path.is_file() && path.permissions().mode() & &0o111 != 0 {
        true
    } else {
        false
    }
}

pub fn bin(bin: &str, args: &Vec<String>) -> Result<(), ShellError> {
    let bin = find_executable_in_path(bin)?;

    let _ = process::Command::new(bin).args(args).spawn()?.wait();

    Ok(())
}

pub fn pwd() -> Result<String, ShellError> {
    Ok(env::current_dir()?.display().to_string())
}
