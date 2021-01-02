#[derive(Debug)]
pub enum CommonError {
    MessageError(String),
    ParseIntError(std::num::ParseIntError),
    IOError(std::io::Error),
}

impl std::error::Error for CommonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CommonError::MessageError(_) => None,
            CommonError::ParseIntError(err) => Some(err),
            CommonError::IOError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommonError::MessageError(message) => write!(f, "{}", message),

            CommonError::ParseIntError(err) => {
                write!(f, "Error while parsing Integer:\n{}", err)
            }

            CommonError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for CommonError {
    fn from(err: std::num::ParseIntError) -> CommonError {
        CommonError::ParseIntError(err)
    }
}

impl From<std::io::Error> for CommonError {
    fn from(err: std::io::Error) -> CommonError {
        CommonError::IOError(err)
    }
}
