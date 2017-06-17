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
