use computer::ComputerError;

#[derive(Debug)]
pub enum DroneError {
    InvalidRoomDescription,
    NoSecurityCheckpoint,
    NoSecurityExit,
    NoWeightFits,
    UnknownSecurityMessage,
    ComputerError(ComputerError),
}

impl std::error::Error for DroneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DroneError::InvalidRoomDescription => None,
            DroneError::NoSecurityCheckpoint => None,
            DroneError::NoSecurityExit => None,
            DroneError::NoWeightFits => None,
            DroneError::UnknownSecurityMessage => None,
            DroneError::ComputerError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for DroneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DroneError::InvalidRoomDescription => {
                write!(f, "Invalid Room Description")
            }
            DroneError::NoSecurityCheckpoint => {
                write!(f, "Did not find a Security Checkpoint")
            }
            DroneError::NoSecurityExit => {
                write!(f, "Security Checkpoint does not have an exit for me")
            }
            DroneError::NoWeightFits => {
                write!(f, "Did not find a fitting weight")
            }
            DroneError::UnknownSecurityMessage => {
                write!(f, "Unknown Security Check Message")
            }
            DroneError::ComputerError(ref err) => err.fmt(f),
        }
    }
}

impl From<ComputerError> for DroneError {
    fn from(err: ComputerError) -> DroneError {
        DroneError::ComputerError(err)
    }
}
