use xmltree::Element;

use std::borrow::Cow;
use std::str::FromStr;

use crate::message::error::ParseError;

/// Reads the value of a child element with the given name
pub(super) fn read_child_value<'a>(
    element: &'a Element,
    name: &'static str,
) -> Result<Cow<'a, str>, ParseError> {
    element
        .get_child(name)
        .and_then(Element::get_text)
        .ok_or_else(|| ParseError::missing_element(name))
}

/// Reads the value of a child element with the given name and parses it
pub(super) fn parse_child_value<T>(element: &Element, name: &'static str) -> Result<T, ParseError>
where
    T: FromStr<Err = ParseError>,
{
    read_child_value(element, name).and_then(|v| v.parse())
}

/// Reads the value of an optional child element with the given name and parses it
pub(super) fn parse_optional_child_value<T>(
    element: &Element,
    name: &'static str,
) -> Result<Option<T>, ParseError>
where
    T: FromStr<Err = ParseError>,
{
    let child = read_child_value(element, name).ok();
    child.map(|v| v.parse()).transpose()
}
