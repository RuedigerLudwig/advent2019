use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputerError {
    #[error("Unknown instruction {0} at {1}")]
    UnknownInstruction(i64, usize),

    #[error("Unknown mode {0}")]
    UnknownMode(i64),

    #[error("Input unexpectedly empty")]
    InputEmpty,

    #[error("Output unexpectedly empty")]
    OutputEmpty,

    #[error("Not a valid Ascii char: {0}")]
    NotValidAsciiChar(char),

    #[error("Not a valid Ascii int: {0}")]
    NotValidAsciiInt(i64),

    #[error("Illegal address {0}")]
    IllegalAddress(i64),

    #[error("Can not disassemble code")]
    CanNotDisassemble,

    #[error("Already terminated")]
    Terminated,

    #[error("IoError  {source:?}")]
    IoError {
        #[from]
        source: io::Error,
    },

    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
}
