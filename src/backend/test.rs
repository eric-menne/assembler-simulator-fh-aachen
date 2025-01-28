use crate::commands::Instruction;
use crate::{backend::Runtime, commands::Command};

#[test]
fn test_processor_01() {
    let ram = vec![
        // Prepare the registers
        Command::new(Instruction::LoadFix, 2, 0), // 0
        Command::new(Instruction::SaveToRegister, 12, 1), // 1
        Command::new(Instruction::LoadFix, 3, 2), // 2
        Command::new(Instruction::SaveToRegister, 13, 3), // 3
        Command::new(Instruction::LoadFix, 4, 4), // 4
        Command::new(Instruction::SaveToRegister, 14, 5), // 5
        Command::new(Instruction::LoadFix, 5, 6), // 6
        Command::new(Instruction::SaveToRegister, 15, 7), // 7
        //
        Command::new(Instruction::LoadFromRegister, 13, 8), // 8
        Command::new(Instruction::AddFromRegister, 15, 9),  // 9
        Command::new(Instruction::BRC, 4, 10),              // 10
        Command::new(Instruction::SaveToRegister, 15, 1),   // 11
        Command::new(Instruction::LoadFix, 0, 2),           // 12
        Command::new(Instruction::JMP, 16, 3),              // 13
        Command::new(Instruction::SaveToRegister, 15, 4),   // 14
        Command::new(Instruction::LoadFix, 1, 5),           // 15
        Command::new(Instruction::AddFromRegister, 12, 6),  // 16
        Command::new(Instruction::AddFromRegister, 14, 7),  // 17
        Command::new(Instruction::SaveToRegister, 14, 8),   // 18
        Command::new(Instruction::NOP, 0, 9),               // 19
    ];

    let mut runtime = Runtime::new(16, ram.clone());
    let mut counter = 0;

    while (runtime.instruction_counter as usize) < runtime.ram.len() {
        if counter >= 100 {
            panic!(
                "To many interations. Infinit loop fail save actived; {:#?}",
                runtime
            );
        }
        runtime.execute_command();
        counter += 1;
    }

    assert!(runtime.get_register_value(14) == 6, "{:#?}", runtime);
    assert!(runtime.get_register_value(15) == 8, "{:#?}", runtime);
}

#[test]
fn test_processor_02() {
    let ram = vec![
        // Prepare the registers
        Command::new(Instruction::LoadFix, 2, 0), // 0
        Command::new(Instruction::SaveToRegister, 12, 1), // 1
        Command::new(Instruction::LoadFix, 8, 2), // 2
        Command::new(Instruction::SaveToRegister, 13, 3), // 3
        Command::new(Instruction::LoadFix, 1, 4), // 4
        Command::new(Instruction::SaveToRegister, 14, 5), // 5
        Command::new(Instruction::LoadFix, 9, 6), // 6
        Command::new(Instruction::SaveToRegister, 15, 7), // 7
        //
        Command::new(Instruction::LoadFromRegister, 13, 8), // 8
        Command::new(Instruction::AddFromRegister, 15, 9),  // 9
        Command::new(Instruction::BRC, 4, 10),              // 10
        Command::new(Instruction::SaveToRegister, 15, 1),   // 11
        Command::new(Instruction::LoadFix, 0, 2),           // 12
        Command::new(Instruction::JMP, 16, 3),              // 13
        Command::new(Instruction::SaveToRegister, 15, 4),   // 14
        Command::new(Instruction::LoadFix, 1, 5),           // 15
        Command::new(Instruction::AddFromRegister, 12, 6),  // 16
        Command::new(Instruction::AddFromRegister, 14, 7),  // 17
        Command::new(Instruction::SaveToRegister, 14, 8),   // 18
        Command::new(Instruction::NOP, 0, 9),               // 19
    ];

    let mut runtime = Runtime::new(16, ram.clone());
    let mut counter = 0;

    while (runtime.instruction_counter as usize) < runtime.ram.len() {
        if counter >= 100 {
            panic!(
                "To many interations. Infinit loop fail save actived; {:#?}",
                runtime
            );
        }

        runtime.execute_command();
        counter += 1;
    }

    assert!(runtime.get_register_value(14) == 4, "{:#?}", runtime);
    assert!(runtime.get_register_value(15) == 1, "{:#?}", runtime);
}
