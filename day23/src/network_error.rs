use computer::ComputerError;

#[derive(Debug)]
pub enum NetworkError {
    UnknownAddress(i64),
    NodeStopped,
    ComputerError(ComputerError),
}

impl std::error::Error for NetworkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NetworkError::UnknownAddress(_) => None,
            NetworkError::NodeStopped => None,
            NetworkError::ComputerError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkError::UnknownAddress(adress) => {
                write!(f, "Unknown Address: {}", adress)
            }
            NetworkError::NodeStopped => {
                write!(f, "Node Stoped unexpectedly")
            }

            NetworkError::ComputerError(ref err) => err.fmt(f),
        }
    }
}

impl From<ComputerError> for NetworkError {
    fn from(err: ComputerError) -> NetworkError {
        NetworkError::ComputerError(err)
    }
}
