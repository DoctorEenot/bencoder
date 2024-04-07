use std::fmt::Display;

use serde::{de, ser};
use thiserror::Error as ErrorW;

#[derive(ErrorW, Debug)]
pub enum Error {
    #[error("Serde error: {0:?}")]
    Message(String),

    #[error("{0}")]
    InvalidValue(&'static str),

    #[error("End of feed")]
    Eof,

    #[error("Expected an integer")]
    ExpectedInteger,

    #[error("Expected an unsigned integer")]
    ExpectedUnsignedInteger,

    #[error("Expected boolean")]
    ExpectedBoolean,

    #[error("Expected array")]
    ExpectedArray,

    #[error("Expected dictionary")]
    ExpectedDictionary,

    #[error("Expected string")]
    ExpectedString,

    #[error("Expected enum")]
    ExpectedEnum,

    #[error("Closing tag not found")]
    ClosingTagNotFound,

    #[error("Number is too large")]
    LargeNumber,

    #[error("String size is out of boundaries")]
    BadStringSize,

    #[error("String is too big for a char")]
    TooBigChar,

    #[error("Bad syntax")]
    Syntax,

    #[error("Trailing bytes were left unparsed")]
    TrailingBytes,
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}
impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Message(msg.to_string())
    }
}
