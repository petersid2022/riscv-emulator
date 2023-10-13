pub const NUM_REGISTERS: usize = 31;

pub struct CPU {
    x0: u32,
    x: [u32; NUM_REGISTERS],
    pc: u32,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            x0: 0,
            x: [0; NUM_REGISTERS],
            pc: 0,
        }
    }
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

    // -- Decode the opcode that you got --
    // The opcode is decoded by using a match statement.
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
            let imm = ((instruction >> 20) & 0xFFFF) as i32;
            let extended_imm = (imm as u32) as i32;
            cpu.x[rd] = ((extended_imm as u32) + cpu.pc) as u32;
        }

        0x13 => {
            let funct3 = ((instruction >> 12) & 0x7) as usize;
            match funct3 {
                // addi (add immediate) instruction
                // Implementation x[rd] = x[rs1] + sext(immediate)
                0x0 => {
                    let rd = ((instruction >> 7) & 0x1F) as usize;
                    let rs1 = ((instruction >> 15) & 0x1F) as usize;
                    let imm = ((instruction >> 20) & 0xFFF) as usize;
                    let extended_imm = (imm as u32) as i32;
                    cpu.x[rd] = cpu.x[rs1] + (extended_imm as u32);
                }

                // slti (set less than imediate)
                // Implementation x[rd] = x[rs1] <s sext(immediate)
                0x2 => {
                    let rd = ((instruction >> 7) & 0x1F) as usize;
                    let rs1 = ((instruction >> 15) & 0x1F) as usize;
                    let imm = ((instruction >> 20) & 0xFFF) as usize;
                    let extended_imm = (imm as u32) as i32;
                    cpu.x[rd] = if (cpu.x[rs1] as i32) < extended_imm {
                        1
                    } else {
                        0
                    };
                }

                // sltiu (set less than imediate unsigned)
                // Implementation x[rd] = x[rs1] <u sext(immediate)
                0x3 => {
                    let rd = ((instruction >> 7) & 0x1F) as usize;
                    let rs1 = ((instruction >> 15) & 0x1F) as usize;
                    let imm = ((instruction >> 20) & 0xFFF) as usize;
                    cpu.x[rd] = if cpu.x[rs1] < (imm as u32) { 1 } else { 0 };
                }

                // xori
                // Implementation x[rd] = x[rs1] ^ sext(immediate)
                //0x4 => {
                //    let rd = ((instruction >> 7) & 0x1F) as usize;
                //    let rs1 = ((instruction >> 15) & 0x1F) as usize;
                //    let imm = ((instruction >> 20) & 0xFFF) as usize;
                //    let extended_imm = (imm as u32) as i32;
                //    cpu.x[rd] = cpu.x[rs1] ^ extended_imm;
                //}

                // slli
                // Implementation x[rd] = x[rs1] << shamt
                // 0x1 => {
                //     let rd = ((instruction >> 7) & 0x1F) as usize;
                //     let rs1 = ((instruction >> 15) & 0x1F) as usize;
                //     let shamt = ((instruction >> 20) & 0x1F) as usize;
                // }

                _ => {
                    println!("ur fucked");
                }
            }
        }

        _ => {
            println!("error no opcode matched");
        }
    }

    cpu.x
}
