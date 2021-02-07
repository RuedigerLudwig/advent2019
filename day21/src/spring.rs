use crate::error::SpringError;
use computer::{Code, Input, ListInput, TextVM};
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Read {
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

impl Display for Read {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Read::A => write!(f, "A"),
            Read::B => write!(f, "B"),
            Read::C => write!(f, "C"),
            Read::D => write!(f, "D"),
            Read::E => write!(f, "E"),
            Read::F => write!(f, "F"),
            Read::G => write!(f, "G"),
            Read::H => write!(f, "H"),
            Read::I => write!(f, "I"),
            Read::T => write!(f, "T"),
            Read::J => write!(f, "J"),
        }
    }
}

#[derive(Debug)]
pub enum Write {
    T,
    J,
}

impl Display for Write {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Write::T => write!(f, "T"),
            Write::J => write!(f, "J"),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    AND(Read, Write),
    OR(Read, Write),
    NOT(Read, Write),
}

impl Input for &Instruction {
    fn get_data(&self) -> Result<Vec<i64>, computer::ComputerError> {
        self.to_string().get_data()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::AND(r, w) => write!(f, "AND {} {}", r, w),
            Instruction::OR(r, w) => write!(f, "OR {} {}", r, w),
            Instruction::NOT(r, w) => write!(f, "NOT {} {}", r, w),
        }
    }
}

pub struct SpringBotComputer<'a> {
    input: ListInput,
    vm: TextVM<'a>,
}

impl<'a> SpringBotComputer<'a> {
    pub fn new(code: Code) -> SpringBotComputer<'a> {
        let input = ListInput::new();
        let vm = TextVM::new(code, input.clone());
        SpringBotComputer { input, vm }
    }

    fn go(
        &mut self,
        instructions: &[Instruction],
        enter: &str,
        be_silent: bool,
    ) -> Result<i64, SpringError> {
        self.vm.restart();
        self.input.clear();

        if let Some(question) = self.vm.read_line()? {
            if !be_silent {
                println!("{}", question);
            }
            for inst in instructions {
                if !be_silent {
                    println!("{}", inst.to_string());
                }
                self.input.provide(inst)?;
            }

            if !be_silent {
                println!("{}", enter)
            }
            self.input.provide(enter)?;
        }

        while let Some(line) = self.vm.read_line()? {
            if !be_silent {
                println!("{}", line);
            }
        }

        if let Some(result) = self.vm.next()? {
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
