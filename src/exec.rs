use ::std::{env, ffi};
use std::os::unix::fs::PermissionsExt;

pub fn type_cmd(s: &str) -> Result<String, String> {
    const BUILTIN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

    if BUILTIN_COMMANDS.contains(&s) {
        return Ok(format!("{s} is a shell builtin"));
    }

    let bin = ffi::OsStr::new(s);
    let key = "PATH";
    let paths = env::var_os(key).ok_or("{key} is not defined in the environment.")?;

    let mut result = String::new();

    let entries = env::split_paths(&paths);
    for entry in entries {
        let path = entry.join(bin);
        // Check here
        if path.is_file() && path.metadata().unwrap().permissions().mode() & &0o111 != 0 {
            // We check the str before so this is safe
            // result.push_str(&format!("{s} is {}\n", path.to_str().unwrap())); // Safe unwrap
            return Ok(format!("{s} is {}", path.to_str().unwrap()));
        }
    }

    if result.is_empty() {
        Err(format!("{s} not found"))
    } else {
        Ok(result)
    }
}
