use common::error::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WireError {
    #[error("No crossing found")]
    NoCrossing,

    #[error("Could not parse the given string: {0}")]
    ParseError(String),

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
