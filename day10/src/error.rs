use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsteroidError {
    #[error("There is no single best center in this field")]
    NoBestCenter,

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
