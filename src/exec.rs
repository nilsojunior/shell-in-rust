use ::std::{env, ffi, process};
use std::{os::unix::fs::PermissionsExt, path::PathBuf};

use crate::errors::ShellError;

pub fn find_executable_in_path(s: &str) -> Result<String, ShellError> {
    const BUILTIN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

    if BUILTIN_COMMANDS.contains(&s) {
        return Ok(format!("{s} is a shell builtin"));
    }

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
            return Ok(format!("{s} is {}", path.to_str().unwrap()));
        }
    }

    if result.is_empty() {
        Err(ShellError::NotFound(s.to_string()))
    } else {
        Ok(result)
    }
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
    // Fix: this is a war crime
    let bin: String = find_executable_in_path(bin)?
        .split_whitespace()
        .next()
        .map(String::from)
        .unwrap();

    let output = process::Command::new(bin).args(args).output()?;

    if output.status.success() {
        print!("{}", String::from_utf8(output.stdout)?);
    } else {
        eprint!("{}", String::from_utf8(output.stderr)?);
    }

    Ok(())
}
