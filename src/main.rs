use crate::emulator::CPU;
use object::{Object, ObjectSection};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

mod emulator;

enum DisassemblerResult {
    Success(Vec<u32>),
    Failure(u32),
}

fn disassembly(dir: &str) -> Result<DisassemblerResult, Box<dyn Error>> {
    let bin_data = fs::read(dir)?;
    let obj_file = object::File::parse(&*bin_data)?;
    if let Some(section) = obj_file.section_by_name(".text") {
        if let Ok(section_data) = section.data() {
            let mut instructions = Vec::new();
            for i in (0..section_data.len()).step_by(4) {
                if i + 4 <= section_data.len() {
                    let instruction = &section_data[i..i + 4];
                    let binary_string = format!(
                        "{:#034b}",
                        u32::from_le_bytes([
                            instruction[0],
                            instruction[1],
                            instruction[2],
                            instruction[3]
                        ])
                    );
                    let out = u32::from_str_radix(&binary_string[2..], 2)
                        .map_err(|e| format!("Failed to parse binary string: {}", e))?;
                    instructions.push(out);
                }
            }
            return Ok(DisassemblerResult::Success(instructions));
        } else {
            return Err("Failed to get section data".into());
        }
    } else {
        return Err("Section not available".into());
    }
}

fn main() {
    let mut cpu = CPU::new();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please specify the path to the RISC-V (RV32I) binary that you wish to emulate");
        process::exit(0);
    }

    let dir = &args[1];
    match disassembly(dir) {
        Ok(binary_instructions) => match binary_instructions {
            DisassemblerResult::Success(instructions) => {
                for instruction in instructions {
                    println!("{:032b}", instruction);
                    CPU::emulate_cycle(&mut cpu, instruction);
                }
            }
            DisassemblerResult::Failure(err) => {
                eprintln!("Error: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
