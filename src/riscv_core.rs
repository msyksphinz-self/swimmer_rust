pub struct EnvBase
{
    pub m_regs: Vec<u64>,   // general register
    pub m_memory: Vec<u32>,  // memory
}


impl EnvBase {
    const DRAM_BASE: u32 = 0x1000_0000;
    const DRAM_SIZE: usize = 0x10_0000;

    pub fn new() -> Self {
        Self { m_memory: vec![0; Self::DRAM_SIZE],
               m_regs: vec![0; 32],
        }
    }
}


pub trait Riscv64Core {
    const XLEN: u32;

    fn FetchMemory(&mut self, addr:usize, data:&mut u32) -> u32;
    fn ReadMemory (&self, addr:usize) -> u32;
    fn WriteMemory(&mut self, addr:usize, data:u32) -> u32;
}


impl Riscv64Core for EnvBase {
    const XLEN: u32 = 64;

    fn FetchMemory(&mut self, addr:usize, data:&mut u32) -> u32 {
        data = &mut self.m_memory[addr];
        return self.m_memory[addr];
    }

    fn ReadMemory(&self, addr:usize) -> u32 {
        return self.m_memory[addr];
    }

    fn WriteMemory(&mut self, addr:usize, data:u32) -> u32 {
        self.m_memory[addr + 0] = (data >>  0) & 0x0ff;
        self.m_memory[addr + 1] = (data >>  8) & 0x0ff;
        self.m_memory[addr + 2] = (data >> 16) & 0x0ff;
        self.m_memory[addr + 3] = (data >> 24) & 0x0ff;

        return 0;
    }
}

