use super::interface::Report;
use crate::computer::ComputerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DroidError {
    #[error("Unnown Tile: {0}")]
    UnknownTile(i64),

    #[error("Droid error by backtracking into {0}")]
    BacktracingInto(Report),

    #[error("Droid error by backtracking to start")]
    BacktracingToStart,

    #[error("This maze has already been explore")]
    AlreadyExplored,

    #[error("Oxygen not found or already oxygenized")]
    NotReadyToOxygenize,

    #[error("Tile is not oxygenized")]
    NotOxygenized,

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },
}
