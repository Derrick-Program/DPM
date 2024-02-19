use colored::Colorize;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CommandParseError {
    message: String,
}

impl CommandParseError {
    pub fn new(message: &str) -> CommandParseError {
        CommandParseError {
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.red())
    }
}

impl Error for CommandParseError {}
