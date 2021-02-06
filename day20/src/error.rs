use thiserror::Error;

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Unknown tile {0}")]
    UnknownTile(char),

    #[error("Did not find Portal")]
    UnknownPortal,

    #[error("Could not find a path from Entrance to exit")]
    NoPath,

    #[error("The map has not a valid format")]
    InvalidMap,

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
