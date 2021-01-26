#[derive(Debug)]
pub enum SpringError {
    NoData,
    DoesNotFinish,
}

impl std::error::Error for SpringError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpringError::NoData => None,
            SpringError::DoesNotFinish => None,
        }
    }
}

impl std::fmt::Display for SpringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpringError::DoesNotFinish => {
                write!(f, "Does not finish")
            }
            SpringError::NoData => {
                write!(f, "Did not get any data from robot")
            }
        }
    }
}
