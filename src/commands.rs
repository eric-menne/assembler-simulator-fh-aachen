/// Executable command for the runtime.
///
/// The `Command` struct encapsulates an instruction to be executed, along with its operand
/// and the line number from which it was generated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Command {
    pub instruction: Instruction,
    pub operant: usize,
    pub line: usize,
}

impl Command {
    pub fn new(instruction: Instruction, operant: usize, line: usize) -> Self {
        Self {
            instruction,
            operant,
            line,
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.instruction {
            Instruction::NOP => write!(f, "NOP"),
            Instruction::LoadFix => {
                write!(f, "LDA #{}", self.operant)
            }
            Instruction::LoadFromRegister => {
                write!(f, "LDA ({})", self.operant)
            }
            Instruction::SaveToRegister => {
                write!(f, "STA ({})", self.operant)
            }
            Instruction::AddFix => {
                write!(f, "ADD #{}", self.operant)
            }
            Instruction::AddFromRegister => {
                write!(f, "ADD ({})", self.operant)
            }
            Instruction::SubFix => {
                write!(f, "SUB #{}", self.operant)
            }
            Instruction::SubFromRegister => {
                write!(f, "SUB ({})", self.operant)
            }
            Instruction::JMP => {
                write!(f, "JMP {}", self.operant)
            }
            Instruction::BRZ => {
                write!(f, "BRZ #{}", self.operant)
            }
            Instruction::BRC => {
                write!(f, "BRC #{}", self.operant)
            }
            Instruction::BRN => {
                write!(f, "BRN #{}", self.operant)
            }
        }
    }
}

#[allow(unused)]
#[allow(clippy::style)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    NOP = 0,
    LoadFix = 1,
    LoadFromRegister = 2,
    SaveToRegister = 3,
    AddFix = 4,
    AddFromRegister = 5,
    SubFix = 6,
    SubFromRegister = 7,
    JMP = 8,
    BRZ = 9,
    BRC = 10,
    BRN = 11,
}
