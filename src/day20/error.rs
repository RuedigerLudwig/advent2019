use thiserror::Error;

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Unknown tile {0}")]
    UnknownTile(char),

    #[cfg(test)]
    #[error("Test did not produce expected error")]
    ErrorNotRaised,

    #[error("Did not find Portal")]
    UnknownPortal,

    #[error("Could not find a path from Entrance to exit")]
    NoPath,

    #[error("There must be an equal number of portals with each letter")]
    EqualAmountPortals,

    #[error("The entrance must be on the outside of the maze")]
    EntranceMustBeOuter,

    #[error("The exit must be on the outside of the maze")]
    ExitMustBeOuter,

    #[error("There must be eactly one of Entrance and Exit.")]
    ExactlyOneEntranceExit,

    #[error("There must be of of each portal on the outside and inside")]
    PortalsNotEvenlyDistributed,

    #[error("The letters for portals are not correctly aligned")]
    PortalLettersNotAligned,

    #[error("The map does not seem to be a square, or has no inner room")]
    MapNotCorrectSquare,

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
