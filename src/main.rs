//use crate::emulator::CPU;
use goblin::elf::Elf;
use goblin::error::Result;
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
    process::exit(0);
}

pub fn parse(buffer: &[u8]) -> Result<()> {
    let elf = Elf::parse(&buffer)?;

    for ph in elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            if ph.p_filesz > 0 {
                let start = ph.p_offset as usize;
                let end = (ph.p_offset + ph.p_filesz) as usize;

                let bytes = buffer
                    .get(start..end)
                    .ok_or_else(|| goblin::error::Error::Malformed("Invalid range".to_string()))?;

                for &byte in bytes {
                    println!("{:#010b}", byte);
                }
            }
        }
    }
    Ok(())
}

fn read_directory(dir: &str) -> Result<()> {
    let buffer = fs::read(&dir)?;
    parse(&buffer)?;
    Ok(())
}

fn main() {
    //let mut cpu = CPU::new();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        help();
    }

    let dir = &args[1];
    if let Err(err) = read_directory(&dir) {
        eprintln!("Error reading directory: {}", err);
        process::exit(0);
    }

    //CPU::emulate_cycle(&mut cpu, out);
}
