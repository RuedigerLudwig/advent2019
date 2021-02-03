use common::error::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("Did not produce any ore")]
    NoOre,

    #[error("Unknonw Ingredient: {0}")]
    IngredientError(String),

    #[error("Unknonw Reaction: {0}")]
    ReactionError(String),

    #[error("CommonEror: {source}")]
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
