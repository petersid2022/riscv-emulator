use crate::emulator::{ cpu_init, emulate_cycle, CPU };

mod emulator;

fn main(){
    cpu_init(&mut cpu);
    emulate_cycle();
    println!("Hello world!");
}
