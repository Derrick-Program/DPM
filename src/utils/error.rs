use colored::Colorize;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl MyError {
    pub fn new(message: &str) -> MyError {
        MyError {
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.red())
    }
}

impl Error for MyError {}
