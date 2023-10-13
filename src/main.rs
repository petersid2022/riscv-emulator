use crate::emulator::{ emulate_cycle, CPU };

mod emulator;

fn main(){
    let mut cpu = CPU::new();

    let instructions: u32 = 0b00011101100011100101001010010111;
    let result = emulate_cycle(instructions, &mut cpu);

    println!("{:?}", result);
}
