use crate::computer::ComputerError;
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

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },
}
