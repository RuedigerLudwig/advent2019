use common::error::CommonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JupiterError {
    #[error("Not a valid moon: {0}")]
    NoValidMoon(String),

    #[error("Not a valid component for moon: {0}")]
    ComponentOutOfBounds(usize),

    #[error("RegexError: {source}")]
    ParseError {
        #[from]
        source: regex::Error,
    },

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },

    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
}
