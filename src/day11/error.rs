use crate::computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaintError {
    #[error("Unknown Color: {0}")]
    UnknownColor(i64),

    #[error("Unknown Turn: {0}")]
    UnknownTurn(i64),

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },
}
