use common::error::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CardError {
    #[error("Unknown Technique: {0}")]
    UnknownTechnique(String),

    #[error("Illegal Decksize ({0})")]
    IllegalDeckSize(i64),

    #[error("Increments must be coprime to descsize got ({0}) and ({1})")]
    NotCoprime(i64, i64),

    #[error(
        "This in only implemented for shuffles with fixpoints, i.e. decksizes with prime cards"
    )]
    NotImplemented,

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },
    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
}
