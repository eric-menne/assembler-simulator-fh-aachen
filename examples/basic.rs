use std::io::stdin;

use asim::{self, compile, Runtime};

const EXAMPLE_CODE: &str = "
    LDA #3
start:
    BRZ 4
    STA (1)
    SUB #1
    JMP start
    NOP 0
";

fn main() {
    print!("\x1B[?1049h"); // Enter alternate screen
    
    let raw_lines = EXAMPLE_CODE.lines().collect();

    let commands = compile(EXAMPLE_CODE).expect("Unable to compile");

    let mut runtime = Runtime::new(16, commands);

    loop {
        print!("\x1B[2J\x1B[H"); // Clear screen and move to (0|0) with the cursor
        let bits = runtime.get_status_bits();

        print_commands(&raw_lines, runtime.get_next_line());

        println!("Accumator: {}", runtime.get_accumulator());
        println!("Register: {:#?}", runtime.get_register());
        println!("Carry: {} Negativ: {} Zero: {}", bits.carry, bits.negative, bits.zero);

        wait_for_enter();

        if !runtime.tick() {
            break;
        };
    }

    wait_for_enter();
    println!("\x1B[?1049l"); // Leave alternate screen
}

fn wait_for_enter() {
    let mut s = String::new();
    println!("Press enter");
    stdin().read_line(&mut s).unwrap();
}

fn print_commands(raw_lines: &Vec<&str>, active_line: usize) {
    for (index, line) in raw_lines.iter().enumerate() {
        if index == active_line {
            print!(">>");
        }
        println!("{}", line);
    }
}
