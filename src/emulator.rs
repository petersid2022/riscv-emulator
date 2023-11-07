pub const NUM_REGISTERS: usize = 31;

pub struct CPU {
    x: [u32; NUM_REGISTERS],
    pc: u32,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            x: [0; NUM_REGISTERS],
            pc: 0,
        }
    }

    // Emulation cycle: Fetch -> Decode -> Execute
    // Every cycle, the method emulate_cycle is called which emulates one cycle of the Risc-V CPU.
    // During this cycle, the emulator will Fetch, Decode and Execute one opcode.
    pub fn emulate_cycle(&mut self, instruction: u32) {
        let opcode: u8 = CPU::fetch(instruction);
        CPU::decode(self, opcode, instruction);
        let out: [u32; NUM_REGISTERS] = CPU::execute(self);
        println!("{:?}", out);
    }

    // Get opcode from instruction
    // (The 7 Least Significant Bits of the base ISA instructions)
    // With the 2 Least Significant bits on all of the 32-bit instructions
    // in the base ISA having always being set to 11
    fn fetch(instruction: u32) -> u8 {
        const OPCODE_MASK: u32 = 0x7F;
        let opcode = (instruction & OPCODE_MASK) as u8;
        return opcode;
    }

    // Decode the opcode that you got
    // The opcode is decoded by using a match statement.
    // Sidenote: Except for the 5-bit immediates used in CSR instructions,
    // immediates are always sign-extended.
    fn decode(&mut self, opcode: u8, instruction: u32) {
        match opcode {
            // LUI (Load Upper Immediate)
            // x[rd] = sext(immediate[31:12] << 12)
            0x37 => {
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let imm = (instruction & 0xFFFFF000) as i32;
                let extended_imm = (imm as u32) as i32;
                self.x[rd] = extended_imm as u32;
            }

            // auipc (add upper immediate to pc)
            // x[rd] = pc + sext(immediate[31:12] << 12)
            0x17 => {
                let rd = ((instruction >> 7) & 0x1F) as usize;
                let imm = ((instruction >> 20) & 0xFFFF) as i32;
                let extended_imm = (imm as u32) as i32;
                self.x[rd] = ((extended_imm as u32) + self.pc) as u32;
            }

            0x13 => {
                let funct3 = ((instruction >> 12) & 0x7) as usize;
                match funct3 {
                    // addi (add immediate)
                    // x[rd] = x[rs1] + sext(immediate)
                    0x0 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let imm = ((instruction >> 20) & 0xFFF) as usize;
                        let extended_imm = (imm as u32) as i32;
                        self.x[rd] = self.x[rs1] + (extended_imm as u32);
                    }

                    // slti (set less than imediate)
                    // x[rd] = x[rs1] <s sext(immediate)
                    0x2 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let imm = ((instruction >> 20) & 0xFFF) as usize;
                        let extended_imm = (imm as u32) as i32;
                        self.x[rd] = if (self.x[rs1] as i32) < extended_imm {
                            1
                        } else {
                            0
                        };
                    }

                    // sltiu (set less than imediate unsigned)
                    // x[rd] = x[rs1] <u sext(immediate)
                    0x3 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let imm = ((instruction >> 20) & 0xFFF) as usize;
                        self.x[rd] = if self.x[rs1] < (imm as u32) { 1 } else { 0 };
                    }

                    // xori
                    // x[rd] = x[rs1] ^ sext(immediate)
                    0x4 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let imm = ((instruction >> 20) & 0xFFF) as usize;
                        let extended_imm = (imm as u32) as i32;
                        self.x[rd] = ((self.x[rs1] as i32) ^ extended_imm) as u32;
                    }

                    // ori
                    // x[rd] = x[rs1] | sext(immediate)
                    0x6 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let imm = ((instruction >> 20) & 0xFFF) as usize;
                        let extended_imm = (imm as u32) as i32;
                        self.x[rd] = ((self.x[rs1] as i32) | extended_imm) as u32;
                    }

                    // andi
                    // x[rd] = x[rs1] & sext(immediate)
                    0x7 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let imm = ((instruction >> 20) & 0xFFF) as usize;
                        let extended_imm = (imm as u32) as i32;
                        self.x[rd] = ((self.x[rs1] as i32) & extended_imm) as u32;
                    }

                    // slli
                    // x[rd] = x[rs1] << shamt
                    0x1 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let shamt = ((instruction >> 20) & 0x1F) as usize;
                        self.x[rd] = self.x[rs1] << shamt;
                    }

                    // srli (logical right shift on the value in register rs1 by the shift
                    // amount held in the lower 5 bits of the immediate (shamt))
                    // x[rd] = x[rs1] >>u shamt
                    0x5 => {
                        let rd = ((instruction >> 7) & 0x1F) as usize;
                        let rs1 = ((instruction >> 15) & 0x1F) as usize;
                        let shamt = ((instruction >> 20) & 0x1F) as usize;
                        self.x[rd] = self.x[rs1] >> shamt;
                    }

                    _ => {
                        println!("ERROR: No opcode matched");
                    }
                }
            }

            _ => {
                println!("ERROR: No opcode matched");
            }
        }
    }

    fn execute(&mut self) -> [u32; NUM_REGISTERS] {
        return self.x;
    }
}
