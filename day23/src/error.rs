use std::sync::mpsc::{RecvError, SendError};

use computer::ComputerError;
use thiserror::Error;

use crate::network::ThreadResult;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unknown Address: {0}")]
    UnknownAddress(usize),

    #[error("Node Stoped unexpectedly")]
    NodeStopped,

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },

    #[error("RecvError: {source}")]
    RecvError {
        #[from]
        source: RecvError,
    },

    #[error("SendErrorThread: {source}")]
    SendErrorThread {
        #[from]
        source: SendError<ThreadResult>,
    },

    #[error("SendError3: {source}")]
    SendError3 {
        #[from]
        source: SendError<(i64, i64, i64)>,
    },
}
