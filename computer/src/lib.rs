pub mod computer;
pub mod computer_error;
pub mod input;
pub mod runner;

use computer::Computer as RawComputer;
use input::ListInput;

pub type Computer = RawComputer<ListInput>;
