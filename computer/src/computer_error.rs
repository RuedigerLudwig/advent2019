use common::CommonError;

#[derive(Debug)]
pub enum ComputerError {
    MessageError(String),
    CommonError(CommonError),
    UnknownOperation(i32),
    IllegalAddress(usize, String),
}

impl std::error::Error for ComputerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComputerError::MessageError(_) => None,
            ComputerError::UnknownOperation(_) => None,
            ComputerError::IllegalAddress(_, _) => None,
            ComputerError::CommonError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for ComputerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ComputerError::MessageError(message) => write!(f, "{}", message),
            ComputerError::UnknownOperation(op_code) => write!(f, "Unknown OpCode: {}", op_code),
            ComputerError::IllegalAddress(addr, message) => {
                write!(f, "Illegal address {} for {}", addr, message)
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
