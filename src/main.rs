//use crate::emulator::CPU;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::env;

mod emulator;

fn help() {
    let help = "Usage:
    your_program_name <path_to_riscv_binary>
    
    Description:
    Specify the path to a RISC-V (RV32I) binary that you wish to emulate.";
    println!("{}", help);
}

fn main() {
    //let mut cpu = CPU::new();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        help();
        return;
    }

    let dir = &args[1];
    let my_buf = BufReader::new(File::open(dir).unwrap());

    for byte_or_error in my_buf.bytes() {
        let byte = byte_or_error.unwrap();
        print!("{:b}", byte);
    }

    //CPU::emulate_cycle(&mut cpu, bytes[0]);
}
