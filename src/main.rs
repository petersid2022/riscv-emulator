use crate::emulator::CPU;

mod emulator;

fn main(){
    let mut cpu = CPU::new();

    // addi x10, x0, 11; 
    let instructions: u32 = 0b00000000101100000000010100010011;

    CPU::emulate_cycle(&mut cpu, instructions);
}
