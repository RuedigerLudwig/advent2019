use common::CommonError;

#[derive(Debug)]
pub enum JupiterError {
    MessageError(String),
    ParseError(regex::Error),
    CommonError(CommonError),
}

impl std::error::Error for JupiterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JupiterError::MessageError(_) => None,
            JupiterError::ParseError(err) => Some(err),
            JupiterError::CommonError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for JupiterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JupiterError::MessageError(message) => {
                write!(f, "{}", message)
            }

            JupiterError::ParseError(ref err) => err.fmt(f),
            JupiterError::CommonError(ref err) => err.fmt(f),
        }
    }
}

impl From<regex::Error> for JupiterError {
    fn from(err: regex::Error) -> JupiterError {
        JupiterError::ParseError(err)
    }
}

impl From<CommonError> for JupiterError {
    fn from(err: CommonError) -> JupiterError {
        JupiterError::CommonError(err)
    }
}
