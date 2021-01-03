use common::CommonError;

#[derive(Debug)]
pub enum OrbitError {
    OnlyTwoPerLine,
    NoCenterFound,
    NoPathError(String, String),
    CommonError(CommonError),
}

impl std::error::Error for OrbitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OrbitError::NoCenterFound => None,
            OrbitError::OnlyTwoPerLine => None,
            OrbitError::NoPathError(_, _) => None,
            OrbitError::CommonError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for OrbitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OrbitError::OnlyTwoPerLine => write!(
                f,
                "There needs to be exactly one body around another per line"
            ),
            OrbitError::NoCenterFound => write!(f, "Could not find a single center in this system"),
            OrbitError::NoPathError(fst, snd) => {
                write!(f, "Could not find a path from {} to {}", fst, snd)
            }

            OrbitError::CommonError(ref err) => err.fmt(f),
        }
    }
}

impl From<CommonError> for OrbitError {
    fn from(err: CommonError) -> OrbitError {
        OrbitError::CommonError(err)
    }
}
