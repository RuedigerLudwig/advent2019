use thiserror::Error;

#[derive(Debug, Error)]
pub enum FftError {
    #[error("Not a digit: {0}")]
    NotADigit(char),

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
