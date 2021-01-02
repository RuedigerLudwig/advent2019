use common::CommonError;

#[derive(Debug)]
pub enum WireError {
    MessageError(String),
    ParseError(String),
    ParseIntError(std::num::ParseIntError),
    ReadError { source: std::io::Error },
    IOError(std::io::Error),
    CommonError(CommonError),
}

impl std::error::Error for WireError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WireError::MessageError(_) => None,
            WireError::ParseError(_) => None,
            WireError::ReadError { ref source } => Some(source),
            WireError::ParseIntError(err) => Some(err),
            WireError::IOError(err) => Some(err),
            WireError::CommonError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for WireError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WireError::MessageError(message) => {
                write!(f, "{}", message)
            }

            WireError::ParseError(input) => {
                write!(f, "Could not parse the given string: {}", input)
            }

            WireError::ParseIntError(err) => {
                write!(f, "Error while parsing WireTurn:\n{}", err)
            }

            WireError::ReadError { .. } => {
                write!(f, "Read Error")
            }
            WireError::IOError(ref err) => err.fmt(f),
            WireError::CommonError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for WireError {
    fn from(err: std::io::Error) -> WireError {
        WireError::IOError(err)
    }
}

impl From<std::num::ParseIntError> for WireError {
    fn from(err: std::num::ParseIntError) -> WireError {
        WireError::ParseIntError(err)
    }
}

impl From<CommonError> for WireError {
    fn from(err: CommonError) -> WireError {
        WireError::CommonError(err)
    }
}
