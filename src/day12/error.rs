use thiserror::Error;

#[derive(Error, Debug)]
pub enum JupiterError {
    #[error("Not a valid moon: {0}")]
    NoValidMoon(String),

    #[error("RegexError: {source}")]
    ParseError {
        #[from]
        source: regex::Error,
    },

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
