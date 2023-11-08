use crate::emulator::CPU;
use crate::bus::BUS;
use crate::bus::DRAM;
use crate::bus::DRAM_SIZE;
use std::env;
use std::process;

mod bus;
mod disassembly;
mod emulator;

fn main() {
    let dram = DRAM {
        mem: [0; DRAM_SIZE],
    };
    let bus = BUS { dram };

    let mut cpu = CPU::new(bus);

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
