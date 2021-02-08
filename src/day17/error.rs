use crate::common::{direction::Direction, pos::Pos};
use crate::computer::ComputerError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExteriorError {
    #[error("Could not find a robot")]
    NoRobot,

    #[error("Could not find a path for the robot")]
    NoPath,

    #[error("Could not get data from robot")]
    NoData,

    #[error("Can't turn from {0} facing {1}")]
    NoScaffold(Pos<i32>, Direction),

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },

    #[error("IoError: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
