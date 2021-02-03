use common::error::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Passwords need a range of two")]
    RangeError,

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
