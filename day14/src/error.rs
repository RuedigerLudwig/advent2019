use thiserror::Error;

#[derive(Debug, Error)]
pub enum FactoryError {
    #[error("Did not produce any ore")]
    NoOre,

    #[error("Unknonw Ingredient: {0}")]
    IngredientError(String),

    #[error("Unknonw Reaction: {0}")]
    ReactionError(String),

    #[error("IoEror: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("ParserIntError {source:?}")]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },
}
