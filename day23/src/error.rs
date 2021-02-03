use common::error::CommonError;
use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unknown Address: {0}")]
    UnknownAddress(i64),

    #[error("Node Stoped unexpectedly")]
    NodeStopped,

    #[error("Common: {source}")]
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
