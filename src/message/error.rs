use xmltree;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    desc: String,
}

impl ParseError {
    pub fn new<S: Into<String>>(desc: S) -> ParseError {
        ParseError { desc: desc.into() }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> Self {
        ParseError::new(err)
    }
}

impl From<&'static str> for ParseError {
    fn from(err: &'static str) -> Self {
        ParseError::new(err)
    }
}

impl From<xmltree::ParseError> for ParseError {
    fn from(err: xmltree::ParseError) -> Self {
        ParseError::new(err.to_string())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}
