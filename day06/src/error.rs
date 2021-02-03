use common::error::CommonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrbitError {
    #[error("There needs to be exactly one body around another per line")]
    OnlyTwoPerLine,

    #[error("Could not find a single center in this system")]
    NoCenterFound,

    #[error("Could not find a path from {0} to {1}")]
    NoPathError(String, String),

    #[error("Common Error: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },
}
