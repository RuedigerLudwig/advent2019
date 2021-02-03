use common::error::CommonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsteroidError {
    #[error("There is no single best center in this field")]
    NoBestCenter,

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },
}
