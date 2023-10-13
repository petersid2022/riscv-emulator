const NUM_REGISTERS: usize = 31;

pub struct CPU {
    pub x0: u32,
    pub x: [u32; NUM_REGISTERS],
    pub pc: u32,
}

pub fn cpu_init(cpu: &mut CPU) {
    cpu.x0 = 0;
    for i in 0..NUM_REGISTERS {
        cpu.x[i] = 0;
    }
    cpu.pc = 0;
}

pub fn emulate_cycle(instruction: u32, cpu: &mut CPU) -> [u32; NUM_REGISTERS] {
    // Emulation cycle: Fetch -> Decode -> Execute
    // Every cycle, the method emulate_cycle is called which emulates one cycle of the Risc-V CPU.
    // During this cycle, the emulator will Fetch, Decode and Execute one Opcode.

    // -- Get opcode from instruction --
    // (The 7 Least Significant Bits of the base ISA instructions)
    // With the 2 Least Significant bits on all of the 32-bit instructions
    // in the base ISA having always being set to 11
    const OPCODE_MASK: u32 = 0x7F;
    let opcode = (instruction & OPCODE_MASK) as u8;

    match opcode {
        // LUI (Load Upper Immediate) instruction
        // Implementation: x[rd] = sext(immediate[31:12] << 12)
        0x37 => {
            let rd = ((instruction >> 7) & 0x1F) as usize;
            let imm = (instruction & 0xFFFFF000) as i32;
            let extended_imm = (imm as u32) as i32;
            cpu.x[rd] = extended_imm as u32;
        }
        // auipc (add upper immediate to pc) instruction
        // Implementation: x[rd] = pc + sext(immediate[31:12] << 12)
        0x17 => {
            let rd = ((instruction >> 7) & 0x1F) as usize;
            let imm = (instruction & 0xFFFFF000) as i32;
            let extended_imm = (imm as u32) as i32;
            cpu.x[rd] = ((extended_imm as u32) + cpu.pc) as u32;
        }
        // addi (add immediate) instruction
        // Implementation x[rd] = x[rs1] + sext(immediate)
        0x13 => {
            let rd = ((instruction >> 7) & 0x1F) as usize;
            let funct3 = ((instruction >> 12) & 0x00) as usize;
            let rs1 = ((instruction >> 15) & 0x1F) as usize;
            let imm = (instruction & 0xFFF0000) as i32;
            let extended_imm = (imm as u32) as i32;
            cpu.x[rd] = ((extended_imm as u32) + rs1) as u32;

        }
        // sub instruction
        // Subs the register rs2 from rs1 and stores the result in rd. Arithmetic overflow is ignored and the result is simply the low XLEN bits of the result.
        0x33 => {
            let rd = ((instruction >> 7) & 0x1F) as usize;
            let rs1 = ((instruction >> 15) & 0x1F) as usize;
            let rs2 = ((instruction >> 20) & 0x1F) as usize;
            cpu.x[rd] = (cpu.x[rs1] as i32 - cpu.x[rs2] as i32) as u32;
        }



        _ => {
            // Handle unknown opcode or other instructions
            // You might want to add appropriate error handling here.
        }
    }

    cpu.registers
}
