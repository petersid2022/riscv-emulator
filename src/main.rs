use std::env;
use std::process;

mod disassembly;
mod emulator;

fn main() {
    let mut cpu = emulator::CPU::new();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please specify the path to the RISC-V (RV32I) binary that you wish to emulate");
        process::exit(0);
    }

    let dir = &args[1];
    match disassembly::disassembly(dir) {
        Ok(binary_instructions) => {
            for instruction in binary_instructions {
                println!("{:032b}", instruction);
                emulator::CPU::emulate_cycle(&mut cpu, instruction);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
