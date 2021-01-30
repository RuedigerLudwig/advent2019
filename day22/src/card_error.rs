#[derive(Debug)]
pub enum CardError {
    UnknownTechnique(String),
    RepeatForNotCoprime,
    IllegalDeckSize(i64),
    NotCoprime(i64, i64),
    IllegalIncrement(i64, i64),
    NotImplemented,
}

impl std::error::Error for CardError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CardError::UnknownTechnique(_) => None,
            CardError::NotImplemented => None,
            CardError::RepeatForNotCoprime => None,
            CardError::IllegalDeckSize(_) => None,
            CardError::NotCoprime(_, _) => None,
            CardError::IllegalIncrement(_, _) => None,
        }
    }
}

impl std::fmt::Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardError::UnknownTechnique(line) => {
                write!(f, "Unknown Technique: {}", line)
            }
            CardError::NotImplemented => {
                write!(f, "This in only implemented for shuffles with fixpoints, i.e. decksizes with prime cards")
            }
            CardError::RepeatForNotCoprime => {
                write!(f, "Repeat can only be used for coprimes")
            }
            CardError::IllegalDeckSize(deck) => {
                write!(f, "Illegal Decksize ({})", deck)
            }
            CardError::NotCoprime(card, deck) => {
                write!(
                    f,
                    "Increments must be coprime to descsize got ({}) and ({})",
                    card, deck
                )
            }
            CardError::IllegalIncrement(increment, deck) => {
                write!(f, "Illegal Increment ({}) for deck ({})", increment, deck)
            }
        }
    }
}