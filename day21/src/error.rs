use common::error::CommonError;
use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpringError {
    #[error("Did not get any data from robot")]
    NoData,

    #[error("Does not finish")]
    DoesNotFinish,

    #[error("Common: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },
}
