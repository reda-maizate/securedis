use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
    InvalidCommand { message: String },
    InvalidNumberOfArguments { message: String },
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandError::InvalidCommand { message } => write!(f, "{}", message),
            CommandError::InvalidNumberOfArguments { message } => write!(f, "{}", message),
        }
    }
}

pub enum FileError {
    NotFound { file_name: String },
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileError::NotFound { file_name } => {
                write!(f, "File {} not found in the current directory", file_name)
            }
        }
    }
}
