use thiserror::Error;

#[derive(Debug, Error)]
pub enum WireError {
    #[error("No crossing found")]
    NoCrossing,

    #[error("Need two wires to run")]
    NoTwoWires,

    #[error("Could not parse the given string: {0}")]
    ParseError(String),

    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("IoError: {source:?}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
