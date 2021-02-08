use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Passwords need a range of two")]
    RangeError,

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
}
