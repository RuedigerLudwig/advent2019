use common::error::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FftError {
    #[error("Not a digit: {0}")]
    NotADigit(char),

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },
}
