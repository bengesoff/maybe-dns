use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the provided bytes could not be parsed as a DNS message")
    }
}

impl std::error::Error for ParseError {}

impl From<ParseError> for std::io::Error {
    fn from(_: ParseError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, &ParseError)
    }
}
