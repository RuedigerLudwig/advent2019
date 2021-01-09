use common::CommonError;

#[derive(Debug)]
pub enum FactoryError {
    CommonError(CommonError),
    MessageError(String),
    IngredientError(String),
    ReactionError(String),
}

impl std::error::Error for FactoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FactoryError::CommonError(err) => Some(err),
            FactoryError::MessageError(_) => None,
            FactoryError::IngredientError(_) => None,
            FactoryError::ReactionError(_) => None,
        }
    }
}

impl std::fmt::Display for FactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FactoryError::CommonError(ref err) => err.fmt(f),
            FactoryError::MessageError(message) => {
                write!(f, "{}", message)
            }
            FactoryError::IngredientError(ingredient) => {
                write!(f, "Unknonw Ingredient: {}", ingredient)
            }
            FactoryError::ReactionError(reaction) => {
                write!(f, "Unknonw Reaction: {}", reaction)
            }
        }
    }
}

impl From<CommonError> for FactoryError {
    fn from(err: CommonError) -> FactoryError {
        FactoryError::CommonError(err)
    }
}
