use computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Not a valid tile: {0}")]
    NoValidTile(i64),

    #[error("Did not get any score before end of game")]
    NoScore,

    #[error("ComputerError: {source}")]
    CompurerError {
        #[from]
        source: ComputerError,
    },
}
