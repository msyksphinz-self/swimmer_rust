pub const DRAM_BASE: u32 = 0x1000_0000;
pub const DRAM_SIZE: usize = 0x01_0000;

pub struct EnvBase
{
    pub m_regs:  [i32; 32],
    pub m_memory: [u32; DRAM_SIZE], // memory
}


impl EnvBase {
    pub fn new() -> EnvBase {
        EnvBase {
            m_memory: [0; DRAM_SIZE],
            m_regs: [0; 32],
        }
    }
}


pub trait Riscv64Core {
    const XLEN: u32;

    fn fetch_memory(&mut self, addr:usize) -> u32;
    fn read_memory (&mut self, addr:usize) -> u32;
    fn write_memory(&mut self, addr:usize, data:u32) -> u32;
}


impl Riscv64Core for EnvBase {
    const XLEN: u32 = 64;

    fn fetch_memory(&mut self, addr:usize) -> u32 {
        return self.m_memory[addr];
    }

    fn read_memory(&mut self, addr:usize) -> u32 {
        return self.m_memory[addr];
    }

    fn write_memory(&mut self, addr:usize, data:u32) -> u32 {
        self.m_memory[addr + 0] = (data >>  0) & 0x0ff;
        self.m_memory[addr + 1] = (data >>  8) & 0x0ff;
        self.m_memory[addr + 2] = (data >> 16) & 0x0ff;
        self.m_memory[addr + 3] = (data >> 24) & 0x0ff;

        return 0;
    }
}

