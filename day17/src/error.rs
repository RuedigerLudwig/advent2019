use common::{error::CommonError, Direction, Pos};
use computer::ComputerError;
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

    #[error("CommonError: {source}")]
    CommonError {
        #[from]
        source: CommonError,
    },

    #[error("ComputerError: {source}")]
    ComputerError {
        #[from]
        source: ComputerError,
    },
}
