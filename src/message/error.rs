/*
Copyright (C) 2019  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}
