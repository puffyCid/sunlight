use std::fmt;

#[derive(Debug)]
pub enum SunlightError {
    Parser,
}

impl std::error::Error for SunlightError {}

impl fmt::Display for SunlightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunlightError::Parser => write!(f, "Could not parse provided protobuf bytes"),
        }
    }
}
