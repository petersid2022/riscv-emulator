use crate::emulator::CPU;
use object::{Object, ObjectSection};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

mod emulator;

fn disassembly(dir: &str) -> Result<(), Box<dyn Error>> {
    let bin_data = fs::read(dir)?;
    let obj_file = object::File::parse(&*bin_data)?;
    if let Some(section) = obj_file.section_by_name(".text") {
        if let Ok(section_data) = section.data() {
            for i in (0..section_data.len()).step_by(4) {
                if i + 4 <= section_data.len() {
                    let instruction = &section_data[i..i + 4];
                    print!(
                        "{:#034b}\n",
                        u32::from_le_bytes([
                            instruction[0],
                            instruction[1],
                            instruction[2],
                            instruction[3]
                        ])
                    );
                }
            }
        } else {
            eprintln!("Failed to get section data");
        }
    } else {
        eprintln!("section not available");
    }
    Ok(())
}

fn main() {
    let mut cpu = CPU::new();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please specify the path to the RISC-V (RV32I) binary that you wish to emulate");
        process::exit(0);
    }
    let dir = &args[1];
    let _ = disassembly(dir);

    //CPU::emulate_cycle(&mut cpu, out);
}
