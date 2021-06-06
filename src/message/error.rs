use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    Xml(#[from] xmltree::ParseError),
    #[error(transparent)]
    Int(#[from] ParseIntError),
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
