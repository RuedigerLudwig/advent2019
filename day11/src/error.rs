use common::error::CommonError;
use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaintError {
    #[error("Unknown Color: {0}")]
    UnknownColor(i64),

    #[error("Unknown Turn: {0}")]
    UnknownTurn(i64),

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },
}
