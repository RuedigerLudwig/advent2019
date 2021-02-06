use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimpleError {
    #[error("No numbers found")]
    NoNumbersFound,

    #[error("ComputerError: {source}")]
    CompurerError {
        #[from]
        source: ComputerError,
    },
}
