use std::{error, fmt, io, string::FromUtf8Error};

#[derive(Debug)]
pub enum ShellError {
    Io(io::Error),
    NotFound(String),
    EnvVarNotSet(String),
    Utf8(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io(e) => write!(f, "{e}: not found"),
            ShellError::NotFound(file) => write!(f, "{file}: not found"),
            ShellError::EnvVarNotSet(key) => write!(f, "{key}: is not defined in the environment"),
            ShellError::Utf8(e) => write!(f, "{e}: failed to convert to UTF-8"),
        }
    }
}

impl error::Error for ShellError {}

impl From<io::Error> for ShellError {
    fn from(e: io::Error) -> Self {
        ShellError::Io(e)
    }
}

impl From<FromUtf8Error> for ShellError {
    fn from(e: FromUtf8Error) -> Self {
        ShellError::Utf8(e.to_string())
    }
}
