use thiserror::Error;

use crate::computer::ComputerError;

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
