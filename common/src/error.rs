use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("Got IO  error {source:?}")]
    IOError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum ParseError {}
