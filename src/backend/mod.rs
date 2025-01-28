use std::usize;

use crate::commands::{Command, Instruction};
use crate::nibble::Nibble;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy)]
pub struct StatusBits {
    pub carry: bool,
    pub negative: bool,
    pub zero: bool,
}

#[derive(Clone, Default, Debug)]
pub struct Runtime {
    accumulator: Nibble,
    register: Vec<Nibble>,
    ram: Vec<Command>,
    instruction_counter: usize,
}

#[allow(unused)]
impl Runtime {
    pub fn new(register_size: usize, ram: Vec<Command>) -> Self {
        Self {
            accumulator: Nibble::from(0),
            register: vec![Nibble::from(0); register_size],
            ram,
            instruction_counter: 0,
        }
    }

    pub fn get_next_line(&self) -> usize {
        if self.instruction_counter as usize >= self.ram.len() {
            return self.ram.len();
        } else {
            return self.ram[self.instruction_counter as usize].line;
        }
    }

    fn load_into_accumulator<T: Into<Nibble>>(&mut self, value: T) {
        self.accumulator = value.into()
    }

    pub fn get_accumulator(&self) -> Nibble {
        self.accumulator
    }

    fn store_into_register(&mut self, index: usize) {
        self.register[index] = self.accumulator
    }

    pub fn get_register_value(&self, index: usize) -> Nibble {
        self.register[index]
    }

    fn set_register_value<T: Into<Nibble>>(&mut self, index: usize, value: T) {
        self.register[index] = value.into();
    }

    fn increase_instruction_counter(&mut self, steps: usize) {
        self.instruction_counter += steps;
    }

    fn set_instruction_counter(&mut self, value: usize) {
        self.instruction_counter = value;
    }

    fn get_instruction_counter(&self) -> usize {
        self.instruction_counter
    }

    pub fn get_register(&self) -> &Vec<Nibble> {
        &self.register
    }

    pub fn get_status_bits(&self) -> StatusBits {
        StatusBits {
            carry: self.accumulator.has_carry(),
            negative: self.accumulator.has_negative(),
            zero: self.accumulator.is_zero(),
        }
    }

    fn execute_command(&mut self) {
        let command = self.ram[self.instruction_counter as usize];
        match command.instruction {
            Instruction::NOP => (),
            Instruction::LoadFix => self.load_into_accumulator(command.operant),
            Instruction::LoadFromRegister => {
                self.load_into_accumulator(self.get_register_value(command.operant))
            }
            Instruction::SaveToRegister => {
                self.set_register_value(command.operant, self.get_accumulator())
            }
            Instruction::AddFix => {
                self.load_into_accumulator(self.get_accumulator() + command.operant.into())
            }
            Instruction::AddFromRegister => self.load_into_accumulator(
                self.get_accumulator() + self.get_register_value(command.operant).into(),
            ),
            Instruction::SubFix => {
                self.load_into_accumulator(self.get_accumulator() - command.operant.into())
            }
            Instruction::SubFromRegister => self.load_into_accumulator(
                self.get_accumulator() - self.get_register_value(command.operant).into(),
            ),
            Instruction::JMP => {
                self.set_instruction_counter(command.operant);
                return;
            }
            Instruction::BRZ => {
                if self.accumulator.is_zero() {
                    self.increase_instruction_counter(command.operant);
                } else {
                    self.increase_instruction_counter(1);
                }
                return;
            }
            Instruction::BRC => {
                if self.accumulator.has_carry() {
                    self.increase_instruction_counter(command.operant);
                } else {
                    self.increase_instruction_counter(1);
                }
                return;
            }
            Instruction::BRN => {
                if self.accumulator.has_negative() {
                    self.increase_instruction_counter(command.operant);
                } else {
                    self.increase_instruction_counter(1);
                }
                return;
            }
        }
        self.increase_instruction_counter(1);
    }

    pub fn tick(&mut self) -> bool {
        self.execute_command();
        (self.instruction_counter as usize) < self.ram.len()
    }
}
