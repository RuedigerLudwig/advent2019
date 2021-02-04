#![warn(rust_2018_idioms, missing_debug_implementations)]
#![feature(peekable_next_if)]
mod code;
mod common;
mod cpu;
mod cpu_wrapper;
mod error;
mod input;
mod input_converter;
mod list_input;
mod modes;
mod output;
mod text_output;
mod vm;

pub use code::Code;
pub use cpu::{debug, StepResult};
pub use error::ComputerError;
pub use input::{ComputerInput, Input, NoInput, OnceInput};
pub use input_converter::InputConverter;
pub use list_input::ListInput;
pub use output::{MTOutput, STOutput};
pub use text_output::{MTTextOutput, STTextOutput};
pub use vm::{MTVirtualMachine, STVirtualMachine};
