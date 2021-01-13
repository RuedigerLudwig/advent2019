use common::CommonError;

#[derive(Debug)]
pub enum ComputerError {
    MessageError(String),
    CommonError(CommonError),

    UnknownOperation(u8),
    UnknownMode(i64),
    InputEmpty,

    NotValidAsciiChar(char),
    NotValidAsciiInt(i64),

    IllegalAddress(String),
}

impl std::error::Error for ComputerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComputerError::MessageError(_) => None,
            ComputerError::UnknownOperation(_) => None,
            ComputerError::UnknownMode(_) => None,
            ComputerError::IllegalAddress(_) => None,
            ComputerError::NotValidAsciiChar(_) => None,
            ComputerError::NotValidAsciiInt(_) => None,
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
            ComputerError::UnknownMode(mode) => write!(f, "Unknown Mode: {}", mode),
            ComputerError::InputEmpty => write!(f, "Input unexpectedly empty"),
            ComputerError::NotValidAsciiChar(ch) => {
                write!(f, "Not a valid Ascci Char: {}", ch)
            }
            ComputerError::NotValidAsciiInt(num) => {
                write!(f, "Not a valid Ascci int: {}", num)
            }
            ComputerError::IllegalAddress(message) => {
                write!(f, "Illegal address {}", message)
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
