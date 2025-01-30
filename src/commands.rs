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

pub fn get_instruction_attribute(str: &str) -> Option<InstructionAttribute> {
    Some(match str.to_uppercase().as_str() {
        "NOP" => InstructionAttribute::from(0b00000000),
        "LDA" => InstructionAttribute::from(0b00000011),
        "STA" => InstructionAttribute::from(0b00000010),
        "ADD" => InstructionAttribute::from(0b00000011),
        "SUB" => InstructionAttribute::from(0b00000011),
        "JMP" => InstructionAttribute::from(0b00000101),
        "BRZ" => InstructionAttribute::from(0b00000001),
        "BRC" => InstructionAttribute::from(0b00000001),
        "BRN" => InstructionAttribute::from(0b00000001),
        _ => return None,
    })
}

/// ```plaintext
/// +-------------------------+
/// |    0 b 0 0 0 0 0 0 0 0  |
/// |                  ↑ ↑ ↑  |
/// |                  | | |  |
/// |       Label    <━┛ | |  |
/// |       Address    <━┛ |  |
/// |       Fixed number <━┛  |
/// +-------------------------+
/// ```
///
/// Bit Meaning:
/// - Bit 0 (1st bit): Fixed numbers are allowed (0b00000001)
/// - Bit 1 (2nd bit): Addresses are allowed (0b00000010)
/// - Bit 2 (3rd bit): Labels are allowed (0b00000100)
/// - Bit 3 (4th bit): Don't care
/// - Bit 4 (5th bit): Don't care
/// - Bit 5 (6th bit): Don't care
/// - Bit 6 (7th bit): Don't care
/// - Bit 7 (8th bit): Don't care
///
pub(crate) struct InstructionAttribute(u8);

impl From<u8> for InstructionAttribute {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl InstructionAttribute {
    pub fn allow_no_operant(&self) -> bool {
        self.0 == 0b00000000
    }
    
    pub fn allow_fixed_number(&self) -> bool {
        (self.0 & 0b00000001) != 0
    }

    pub fn allow_address(&self) -> bool {
        (self.0 & 0b00000010) != 0
    }

    pub fn allow_label(&self) -> bool {
        (self.0 & 0b00000100) != 0
    }
}
