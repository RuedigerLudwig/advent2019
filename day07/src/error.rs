use common::error::CommonError;
use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmplifierError {
    #[error("Did not get exactly one result")]
    NotExactlyOne,

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },

    #[error("ComputerError: {source}")]
    CompurerError {
        #[from]
        source: ComputerError,
    },
}
