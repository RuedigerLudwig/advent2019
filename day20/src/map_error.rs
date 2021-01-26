#[derive(Debug)]
pub enum MapError {
    UnknownTile(char),
    UnknownPortal,
    NoPath,
    InvalidMap,
}

impl std::error::Error for MapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MapError::UnknownTile(_) => None,
            MapError::UnknownPortal => None,
            MapError::NoPath => None,
            MapError::InvalidMap => None,
        }
    }
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::UnknownTile(tile) => {
                write!(f, "Unknown tile {}", tile)
            }
            MapError::UnknownPortal => {
                write!(f, "Did not find Portal")
            }
            MapError::NoPath => {
                write!(f, "Could not find a path from Entrance to exit")
            }
            MapError::InvalidMap => {
                write!(f, "The map has not a valid format")
            }
        }
    }
}
