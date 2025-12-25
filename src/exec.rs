use ::std::{env, ffi, process};
use std::{
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use crate::errors::ShellError;

fn find_executable_in_path(s: &str) -> Result<String, ShellError> {
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

const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

pub fn type_cmd(s: &str) -> Result<String, ShellError> {
    if BUILTIN_COMMANDS.contains(&s) {
        return Ok(format!("{s} is a shell builtin"));
    }

    Ok(format!("{s} is {}", find_executable_in_path(s)?))
}

fn is_executable(path: &PathBuf) -> bool {
    path.metadata()
        .map(|f| f.is_file() && f.permissions().mode() & &0o111 != 0)
        .unwrap_or(false)
}

pub fn bin(bin: &str, args: &Vec<String>) -> Result<(), ShellError> {
    find_executable_in_path(bin)?;

    let _ = process::Command::new(bin).args(args).spawn()?.wait();

    Ok(())
}

pub fn get_current_dir() -> Result<String, ShellError> {
    Ok(env::current_dir()?.display().to_string())
}

pub fn cd(path: &str) -> Result<(), ShellError> {
    let path = Path::new(path);

    if path.exists() {
        env::set_current_dir(path)?;
        Ok(())
    } else if path.starts_with("~") {
        let key = "HOME";
        let home = PathBuf::from(
            env::var_os(key).ok_or_else(|| ShellError::EnvVarNotSet(key.to_string()))?,
        );

        // Safe because the we check before
        let full_path = home.join(path.strip_prefix("~").unwrap());

        env::set_current_dir(PathBuf::from(full_path))?;
        Ok(())
    } else {
        Err(ShellError::PathDoesNotExist(
            "cd".to_string(),
            path.display().to_string(),
        ))
    }
}
