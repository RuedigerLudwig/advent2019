use crate::computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TractorError {
    #[error("Did not get any data from the droid")]
    NoData,

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
}
