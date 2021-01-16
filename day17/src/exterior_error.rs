use common::{Direction, Pos};

#[derive(Debug)]
pub enum ExteriorError {
    NoRobot,
    NoPath,
    NoData,
    NoScaffold(Pos<i32>, Direction),
}

impl std::error::Error for ExteriorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ExteriorError::NoScaffold(_, _) => None,
            ExteriorError::NoRobot => None,
            ExteriorError::NoPath => None,
            ExteriorError::NoData => None,
        }
    }
}

impl std::fmt::Display for ExteriorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExteriorError::NoScaffold(position, direction) => {
                write!(f, "Can turn from {} facing {:?}", position, direction)
            }
            ExteriorError::NoRobot => {
                write!(f, "Could not find a robot")
            }
            ExteriorError::NoPath => {
                write!(f, "Could not find a path for the robot")
            }
            ExteriorError::NoData => {
                write!(f, "Did not get any data from robot")
            }
        }
    }
}
