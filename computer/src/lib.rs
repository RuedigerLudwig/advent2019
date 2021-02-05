#![warn(rust_2018_idioms, missing_debug_implementations)]
#![feature(peekable_next_if)]
mod code;
mod cpu;
mod error;
mod input;
mod input_converter;
mod list_input;
mod output;
mod text_output;
mod vm;

use cpu::{MTCpuWrapper, STCpuWrapper};
use output::RawOutput;

pub use code::Code;
pub use cpu::{debug_codes, StepResult};
pub use error::ComputerError;
pub use input::{ComputerInput, NoInput, OnceInput};
pub use input_converter::InputConverter;
pub use list_input::ListInput;
pub use text_output::{MTTextOutput, STTextOutput};
pub use vm::{MTVirtualMachine, STVirtualMachine};

pub type STOutput<'a> = RawOutput<STCpuWrapper<'a>>;
pub type MTOutput<'a> = RawOutput<MTCpuWrapper<'a>>;
