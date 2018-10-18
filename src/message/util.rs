/*
Copyright (C) 2017  Armin Häberling

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
use xmltree::Element;

use std::str::FromStr;

use message::error::ParseError;

/// Reads the value of a child element with the given name
pub fn read_child_value<'a>(element: &'a Element, name: &str) -> Result<&'a String, ParseError> {
    element
        .get_child(name)
        .and_then(|v| v.text.as_ref())
        .ok_or_else(|| ParseError::new(format!("missing {}", name)))
}

/// Reads the value of a child element with the given name and parses it
pub fn parse_child_value<T>(element: &Element, name: &str) -> Result<T, ParseError>
where
    T: FromStr<Err = ParseError>,
{
    read_child_value(element, name).and_then(|v| v.parse())
}
