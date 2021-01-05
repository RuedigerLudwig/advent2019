use common::CommonError;

#[derive(Debug)]
pub enum ComputerError {
    MessageError(String),
    CommonError(CommonError),

    UnknownOperation(u8),
    InputEmpty,

    IllegalAddress(String),
    IllegalMode(u8),
}

impl std::error::Error for ComputerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComputerError::MessageError(_) => None,
            ComputerError::UnknownOperation(_) => None,
            ComputerError::IllegalAddress(_) => None,
            ComputerError::IllegalMode(_) => None,
            ComputerError::InputEmpty => None,
            ComputerError::CommonError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for ComputerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ComputerError::MessageError(message) => write!(f, "{}", message),
            ComputerError::UnknownOperation(op_code) => write!(f, "Unknown OpCode: {}", op_code),
            ComputerError::InputEmpty => write!(f, "Input unexpectedly empty"),
            ComputerError::IllegalAddress(message) => {
                write!(f, "Illegal address {}", message)
            }
            ComputerError::IllegalMode(addr) => {
                write!(f, "Illegal mode {}", addr)
            }
            ComputerError::CommonError(ref err) => err.fmt(f),
        }
    }
}

impl From<CommonError> for ComputerError {
    fn from(err: CommonError) -> ComputerError {
        ComputerError::CommonError(err)
    }
}
