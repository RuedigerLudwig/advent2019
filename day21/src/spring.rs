use crate::error::SpringError;
use computer::{Code, TextInput, TextOutput, VirtualMachine};
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ReadRegister {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    T,
    J,
}

impl Display for ReadRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReadRegister::A => write!(f, "A"),
            ReadRegister::B => write!(f, "B"),
            ReadRegister::C => write!(f, "C"),
            ReadRegister::D => write!(f, "D"),
            ReadRegister::E => write!(f, "E"),
            ReadRegister::F => write!(f, "F"),
            ReadRegister::G => write!(f, "G"),
            ReadRegister::H => write!(f, "H"),
            ReadRegister::I => write!(f, "I"),
            ReadRegister::T => write!(f, "T"),
            ReadRegister::J => write!(f, "J"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WriteRegister {
    TOut,
    JOut,
}

impl Display for WriteRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WriteRegister::TOut => write!(f, "T"),
            WriteRegister::JOut => write!(f, "J"),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    AND(ReadRegister, WriteRegister),
    OR(ReadRegister, WriteRegister),
    NOT(ReadRegister, WriteRegister),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Instruction::AND(r, w) => write!(f, "AND {} {}", r, w),
            Instruction::OR(r, w) => write!(f, "OR {} {}", r, w),
            Instruction::NOT(r, w) => write!(f, "NOT {} {}", r, w),
        }
    }
}

pub struct SpringBotComputer<'a> {
    vm: VirtualMachine<'a, TextInput>,
    input: TextInput,
}

impl<'a> SpringBotComputer<'a> {
    pub fn new(code: &'a Code) -> SpringBotComputer<'_> {
        let input = TextInput::new();
        let vm = VirtualMachine::new(code, &input);
        SpringBotComputer { vm, input }
    }

    fn go(
        &mut self,
        instructions: &[Instruction],
        enter: &str,
        be_silent: bool,
    ) -> Result<i64, SpringError> {
        self.vm.restart();
        let output = TextOutput::new(self.vm.get_output());

        if let Some(question) = output.read_line()? {
            if !be_silent {
                println!("{}", question);
            }
            for inst in instructions {
                self.input.write_input(&inst.to_string())?;
                if !be_silent {
                    println!("{}", inst.to_string());
                }
            }

            self.input.write_input(enter)?;
            if !be_silent {
                println!("{}", enter)
            }
        }

        while let Some(line) = output.read_line()? {
            if !be_silent {
                println!("{}", line);
            }
        }

        if let Some(result) = output.int_value()? {
            if result == 10 {
                Err(SpringError::DoesNotFinish)?
            } else {
                Ok(result)
            }
        } else {
            Err(SpringError::NoData)
        }
    }

    pub fn walk(
        &mut self,
        instructions: &[Instruction],
        be_silent: bool,
    ) -> Result<i64, SpringError> {
        self.go(instructions, "WALK", be_silent)
    }

    pub fn run(
        &mut self,
        instructions: &[Instruction],
        be_silent: bool,
    ) -> Result<i64, SpringError> {
        self.go(instructions, "RUN", be_silent)
    }
}
