use common::error::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Unknown tile {0}")]
    UnknownTile(char),

    #[error("Vault must have exactly one entrance")]
    ExactlyOneEntrance,

    #[error("Did not find a path to all keys")]
    NoPath,

    #[error("can only do this on special mazes")]
    NotSpecial,

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },
}
