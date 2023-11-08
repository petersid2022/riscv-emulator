pub const DRAM_BASE: usize = 0x80000000;
pub const DRAM_SIZE: usize = 1024*1024*1;

pub struct DRAM {
    pub mem: [u8; DRAM_SIZE]
}

pub struct BUS {
    pub dram: DRAM,
}

impl BUS {
    pub fn bus_load(&mut self, address: u32, size: u32) -> u32{
        let mut out: u32 = 0;


        return out;
    }

    pub fn bus_store(){
    }
}
