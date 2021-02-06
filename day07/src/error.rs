use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmplifierError {
    #[error("Did not get exactly one result")]
    NotExactlyOne,

    #[error("ComputerError: {source}")]
    CompurerError {
        #[from]
        source: ComputerError,
    },
}
