use crate::commands::{Command, Instruction};

use super::compile;

const TOKEN_TEST_CODE: &str = "
low: 
     ADD (2)
     ADD (15) // test
     BRC 1
up:  STA (15)
     LDA #2
     JMP 0
";

#[test]
fn test_compiled_commands() {
    let expected_commands = vec![
        Command::new(Instruction::AddFromRegister, 2, 2),
        Command::new(Instruction::AddFromRegister, 15, 3),
        Command::new(Instruction::BRC, 1, 4),
        Command::new(Instruction::SaveToRegister, 15, 5),
        Command::new(Instruction::LoadFix, 2, 6),
        Command::new(Instruction::JMP, 0, 7),
    ];

    let text = TOKEN_TEST_CODE;
    let commands = compile(&text);

    assert_eq!(expected_commands, commands.unwrap(),)
}

#[test]
fn test_single_line_commpile() {
    let code = "ADD #1";
    let expected_command = vec![Command::new(Instruction::AddFix, 1, 0)];

    let command = compile(&code).unwrap();
    assert_eq!(expected_command, command);
}
