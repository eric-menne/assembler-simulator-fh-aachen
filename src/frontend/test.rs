use crate::commands::{Command, Instruction};

use super::compile;

#[test]
fn test_single_line_compile() {
    let code = "
low: 
     ADD (2)
     ADD (15) // test
     BRC 1
up:  STA (15)
     LDA #2
     JMP 0
";

    let expected_commands = vec![
        Command::new(Instruction::AddFromRegister, 2, 2),
        Command::new(Instruction::AddFromRegister, 15, 3),
        Command::new(Instruction::BRC, 1, 4),
        Command::new(Instruction::SaveToRegister, 15, 5),
        Command::new(Instruction::LoadFix, 2, 6),
        Command::new(Instruction::JMP, 0, 7),
    ];

    let result = compile(&code);

    match result {
        Ok(commands) => {
            assert_eq!(expected_commands, commands)
        }
        Err(err) => panic!("{:#?}", err),
    }
}

#[test]
fn test_compile_single_line() {
    let code = "ADD #1";
    let expected_command = vec![Command::new(Instruction::AddFix, 1, 0)];

    let result = compile(&code);

    match result {
        Ok(commands) => {
            assert_eq!(expected_command, commands)
        }
        Err(err) => panic!("{:#?}", err),
    }
}

#[test]
fn test_empty_line_compile() {
    let code = "";
    let expected_command: Vec<Command> = vec![];

    let result = compile(&code);

    match result {
        Ok(commands) => {
            assert_eq!(expected_command, commands)
        }
        Err(err) => panic!("{:#?}", err),
    }
}

#[test]
fn test_compile_invalid_multi_line_code() {
    let code = "
ADD #2 ADD #3
";

    assert!(
        compile(&code).is_err(),
        "Expected compilation to fail for multi-line instructions"
    );
}

#[test]
fn test_compile_optional_operant() {
    let code = "
ADD #1
NOP";
    let expected_commands = vec![
        Command::new(Instruction::AddFix, 1, 1),
        Command::new(Instruction::NOP, 0, 2),
        Command::new(Instruction::JMP, 0, 3)
    ];

    let result = compile(code);

    match result {
        Ok(commands) => {
            assert_eq!(expected_commands, commands)
        }
        Err(err) => panic!("{:#?}", err),
    }
}
