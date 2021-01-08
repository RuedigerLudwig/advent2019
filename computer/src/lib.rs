pub mod computer;
pub mod computer_error;
pub mod input;

use input::ListInput;

pub use crate::computer::Computer as RawComputer;

pub type Computer = RawComputer<ListInput>;
