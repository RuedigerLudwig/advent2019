use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum OperationResult {
    Proceed {
        pointer: usize,
    },
    Offset {
        offset: i64,
        pointer: usize,
    },
    Write {
        addr: usize,
        value: i64,
        pointer: usize,
    },
    Output {
        value: i64,
        pointer: usize,
    },
    Stop {
        pointer: usize,
    },
    WaitForInput,
}

impl Display for OperationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OperationResult::*;

        match *self {
            Write {
                addr,
                value,
                pointer,
            } => write!(
                f,
                "Write {} to [{}] and proceed with ({})",
                value, addr, pointer
            ),
            Proceed { pointer } => write!(f, "Proceed with ({})", pointer),
            Stop { pointer } => write!(f, "Stop at ({})", pointer),
            Output { value, pointer } => {
                write!(f, "Output {} and proceed with ({})", value, pointer)
            }
            Offset { offset, pointer } => {
                write!(f, "Offset to {{{}}} and proceed with ({})", offset, pointer)
            }
            WaitForInput => write!(f, "Waiting for Input"),
        }
    }
}
