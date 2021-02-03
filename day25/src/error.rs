use common::error::CommonError;
use computer::ComputerError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DroneError {
    #[error("Invalid Room Description")]
    InvalidRoomDescription,

    #[error("Did not find a Security Checkpoint")]
    NoSecurityCheckpoint,

    #[error("Security Checkpoint does not have an exit for me")]
    NoSecurityExit,

    #[error("Did not find a fitting weight")]
    NoWeightFits,

    #[error("Unknown Security Check Message")]
    UnknownSecurityMessage,

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

    #[error("IO: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },
}
