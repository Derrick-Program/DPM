use colored::Colorize;
use std::error::Error;
use std::fmt;
use std::io;

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

#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::IoError(ref err) => write!(f, "IO Error: {}", err),
        }
    }
}
impl Error for MyError {}
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::IoError(err)
    }
}
