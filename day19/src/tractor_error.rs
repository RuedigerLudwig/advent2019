#[derive(Debug)]
pub enum TractorError {
    NoData,
}

impl std::error::Error for TractorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TractorError::NoData => None,
        }
    }
}

impl std::fmt::Display for TractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TractorError::NoData => {
                write!(f, "Did not get any data from the droid")
            }
        }
    }
}
