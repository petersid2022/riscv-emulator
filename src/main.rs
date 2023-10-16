use crate::emulator::CPU;
use goblin::elf::{ Elf, ProgramHeader };
use std::env;
use std::fs;
use std::process;

mod emulator;

fn help() {
    let help = "Usage:
    your_program_name <path_to_riscv_binary>
    
    Description:
    Specify the path to a RISC-V (RV32I) binary that you wish to emulate.";
    println!("{}", help);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = CPU::new();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        help();
        process::exit(0);
    }

    let dir = &args[1];
    let bytes = fs::read(&dir)?;
    let binary = Elf::parse(&bytes)

    //for (i, &byte) in bytes.iter().enumerate() {
    //    println!("{:08b}", byte);
    //    if (i + 1) % 4 == 0 {
    //        println!();
    //    }
    //}

    //CPU::emulate_cycle(&mut cpu, out);
    Ok(())
}
