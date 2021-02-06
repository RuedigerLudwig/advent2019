use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpringError {
    #[error("Did not get any data from robot")]
    NoData,

    #[error("Does not finish")]
    DoesNotFinish,

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
