#[derive(Debug, Clone)]
pub enum ComputerError {
    MessageError(String),

    UnknownInstruction(i64, usize),
    UnknownMode(i64),
    InputEmpty,

    NotValidAsciiChar(char),
    NotValidAsciiInt(i64),

    IllegalAddress(String),

    Terminated,
}

impl std::error::Error for ComputerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ComputerError::MessageError(_) => None,
            ComputerError::UnknownInstruction(_, _) => None,
            ComputerError::UnknownMode(_) => None,
            ComputerError::IllegalAddress(_) => None,
            ComputerError::NotValidAsciiChar(_) => None,
            ComputerError::NotValidAsciiInt(_) => None,
            ComputerError::InputEmpty => None,
            ComputerError::Terminated => None,
        }
    }
}

impl std::fmt::Display for ComputerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ComputerError::MessageError(message) => write!(f, "{}", message),
            ComputerError::UnknownInstruction(instruction, pointer) => {
                write!(f, "Unknown instruction {} at {}", instruction, pointer)
            }
            ComputerError::UnknownMode(mode) => write!(f, "Unknown Mode: {}", mode),
            ComputerError::InputEmpty => write!(f, "Input unexpectedly empty"),
            ComputerError::Terminated => write!(f, "Cpu hash already crashed"),
            ComputerError::NotValidAsciiChar(ch) => {
                write!(f, "Not a valid Ascci Char: {}", ch)
            }
            ComputerError::NotValidAsciiInt(num) => {
                write!(f, "Not a valid Ascci int: {}", num)
            }
            ComputerError::IllegalAddress(message) => {
                write!(f, "Illegal address {}", message)
            }
        }
    }
}
