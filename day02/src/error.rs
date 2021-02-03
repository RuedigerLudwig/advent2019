use common::error::CommonError;
use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SimpleError {
    #[error("No numbers found")]
    NoNumbersFound,

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },

    #[error("ComputerError: {source}")]
    CompurerError {
        #[from]
        source: ComputerError,
    },
}
