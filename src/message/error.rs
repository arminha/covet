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
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    XmlParseError(#[from] xmltree::ParseError),
    #[error(transparent)]
    IntParseError(#[from] ParseIntError),
    #[error("missing element {name}")]
    MissingElement { name: &'static str },
    #[error("unknown {name}: {value}")]
    UnknownEnumValue { name: &'static str, value: String },
}

impl ParseError {
    pub fn missing_element(name: &'static str) -> Self {
        ParseError::MissingElement { name }
    }

    pub fn unknown_enum_value(name: &'static str, value: &str) -> Self {
        ParseError::UnknownEnumValue {
            name,
            value: value.to_owned(),
        }
    }
}
