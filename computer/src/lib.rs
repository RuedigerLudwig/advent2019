#![warn(rust_2018_idioms, missing_debug_implementations)]
#![feature(iter_map_while)]
mod code;
mod cpu;
mod error;
mod computer_input;
mod input;
mod list_input;
mod text_vm;
mod virtual_machine;

pub use code::Code;
pub use cpu::{debug_codes, StepResult};
pub use error::ComputerError;
pub use computer_input::{ComputerInput, NoInput, OnceInput};
pub use input::Input;
pub use list_input::ListInput;
pub use text_vm::TextVM;
pub use virtual_machine::VirtualMachine;
