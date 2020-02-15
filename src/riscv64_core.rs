use std::collections::HashMap;

use crate::riscv_csr::CsrAddr;
use crate::riscv_csr::RiscvCsr;

use crate::riscv_mmu::Riscv64Mmu;

use crate::riscv_tracer::TraceInfo;
use crate::riscv_tracer::Tracer;

pub type XlenT = i32;
pub type UXlenT = u32;
pub type InstT = u32;
pub type AddrT = u32;
pub type RegAddrT = u8;

pub type Xlen64T = i64;
pub type UXlen64T = u64;
pub type Addr64T = u64;

pub const DRAM_BASE: Addr64T = 0x08000_0000;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PrivMode {
    User,
    Supervisor,
    Hypervisor,
    Machine,
}

impl PrivMode {
    pub fn from_u8(n: u8) -> PrivMode {
        match n {
            0 => PrivMode::User,
            1 => PrivMode::Supervisor,
            2 => PrivMode::Hypervisor,
            3 => PrivMode::Machine,
            _ => PrivMode::Machine,
        }
    }
}

#[derive(Copy, Clone)]
pub enum MemAccType {
    Fetch,
    Write,
    Read,
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum MemResult {
    NoExcept = 0,
    MisAlign = 1 << 0,
    NotDefined = 1 << 1,
    NewRegion = 1 << 2,
    TlbError = 1 << 3,
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum VMMode {
    Mbare = 0,
    Sv32 = 1,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}
impl VMMode {
    pub fn from(x: i64) -> VMMode {
        match x {
            0 => VMMode::Mbare,
            1 => VMMode::Sv32,
            8 => VMMode::Sv39,
            9 => VMMode::Sv48,
            10 => VMMode::Sv57,
            11 => VMMode::Sv64,
            _ => panic!("Intelnal Error: Unknown VMMode = {:}", x),
        }
    }
}

pub struct Riscv64Env {
    pub m_xlen: i32,

    pub m_pc: Addr64T,
    m_previous_pc: Addr64T,
    m_regs: [Xlen64T; 32],
    m_fregs: [Xlen64T; 32],
    pub m_memory: HashMap<Addr64T, u8>, // memory
    pub m_csr: RiscvCsr<Xlen64T>,

    pub m_priv: PrivMode,
    m_maxpriv: PrivMode,
    m_vmmode: VMMode,

    pub m_trace: Tracer,

    m_tohost_addr: Addr64T,
    m_fromhost_addr: Addr64T,
    m_tohost: Xlen64T,
    m_fromhost: Xlen64T,

    m_finish_cpu: bool,

    m_is_update_pc: bool,
}

impl Riscv64Env {
    pub fn new(xlen: i32) -> Riscv64Env {
        Riscv64Env {
            m_xlen: xlen,
            m_pc: DRAM_BASE as Addr64T,
            m_memory: HashMap::new(),
            m_regs: [0; 32],
            m_fregs: [0; 32],
            m_maxpriv: PrivMode::Machine,
            m_previous_pc: 0,
            m_vmmode: VMMode::Mbare,
            m_finish_cpu: false,
            m_fromhost_addr: (DRAM_BASE + 0x001000) as Addr64T,
            m_tohost_addr: (DRAM_BASE + 0x001000) as Addr64T,
            m_fromhost: 0,
            m_tohost: 0,
            m_is_update_pc: false,
            m_csr: RiscvCsr::<i64>::new(),
            m_priv: PrivMode::Machine,
            m_trace: Tracer::new(),
        }
    }

    pub fn extend_sign(data: Xlen64T, msb: Xlen64T) -> Xlen64T {
        let mask: Xlen64T = 1 << msb; // mask can be pre-computed if b is fixed
        let res_data = data & ((1 << (msb + 1)) - 1); // (Skip this if bits in x above position b are already zero.)
        return (res_data ^ mask) - mask;
    }

    pub fn extract_bit_field(hex: Xlen64T, left: u8, right: u8) -> Xlen64T {
        let mask: Xlen64T = (1 << (left - right + 1)) - 1;
        return (hex >> right) & mask;
    }

    pub fn set_bit_field(hex: Xlen64T, val: Xlen64T, left: u8, right: u8) -> Xlen64T {
        let mask: Xlen64T = (1 << (left - right + 1)) - 1;
        return (hex & !(mask << right)) | (val << right);
    }

    pub fn extract_uj_field(hex: InstT) -> Xlen64T {
        let i24_21 = Self::extract_bit_field(hex as Xlen64T, 24, 21) & 0x0f;
        let i30_25 = Self::extract_bit_field(hex as Xlen64T, 30, 25) & 0x03f;
        let i20_20 = Self::extract_bit_field(hex as Xlen64T, 20, 20) & 0x01;
        let i19_12 = Self::extract_bit_field(hex as Xlen64T, 19, 12) & 0x0ff;
        let i31_31 = Self::extract_bit_field(hex as Xlen64T, 31, 31) & 0x01;

        let u_res: Xlen64T =
            (i31_31 << 20) | (i19_12 << 12) | (i20_20 << 11) | (i30_25 << 5) | (i24_21 << 1);
        return Self::extend_sign(u_res, 20);
    }

    pub fn extract_ifield(hex: InstT) -> Xlen64T {
        let uimm32: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 31, 20);
        return Self::extend_sign(uimm32, 11);
    }

    pub fn extract_shamt_field(hex: InstT) -> Xlen64T {
        return Self::extract_bit_field(hex as Xlen64T, 25, 20);
    }

    pub fn extract_sb_field(hex: InstT) -> Xlen64T {
        let i07_07: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 7, 7) & 0x01;
        let i11_08: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 11, 8) & 0x0f;
        let i30_25: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 30, 25) & 0x03f;
        let i31_31: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 31, 31) & 0x01;

        let u_res: Xlen64T = (i31_31 << 12) | (i07_07 << 11) | (i30_25 << 5) | (i11_08 << 1);
        return Self::extend_sign(u_res, 12);
    }

    pub fn extract_sfield(hex: InstT) -> Xlen64T {
        let i11_07: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 11, 7) & 0x01f;
        let i31_25: Xlen64T = Self::extract_bit_field(hex as Xlen64T, 31, 25) & 0x07f;

        let u_res: Xlen64T = (i31_25 << 5) | (i11_07 << 0);

        return Self::extend_sign(u_res, 11);
    }

    pub fn sext_xlen(&mut self, hex: Xlen64T) -> Xlen64T {
        return (hex << (64-self.m_xlen)) >> (64-self.m_xlen)
    }

    pub fn uext_xlen(&mut self, hex: Xlen64T) -> UXlen64T {
        return ((hex as UXlen64T) << (64-self.m_xlen)) >> (64-self.m_xlen)
    }

    pub fn is_update_pc(&mut self) -> bool {
        return self.m_is_update_pc;
    }
    pub fn set_update_pc(&mut self, update_pc: bool) {
        self.m_is_update_pc = update_pc;
    }
}

pub trait Riscv64Core {
    fn get_rs1_addr(inst: InstT) -> RegAddrT;
    fn get_rs2_addr(inst: InstT) -> RegAddrT;
    fn get_rs3_addr(inst: InstT) -> RegAddrT;
    fn get_rd_addr(inst: InstT) -> RegAddrT;

    fn set_pc(&mut self, addr: Addr64T);
    fn get_pc(&mut self) -> Addr64T;
    fn get_previous_pc(&mut self) -> Addr64T;

    fn read_memory_dword(&mut self, phy_addr: Addr64T) -> Result<u64, MemResult>;
    fn read_memory_word(&mut self, phy_addr: Addr64T) -> Result<u32, MemResult>;
    fn read_memory_hword(&mut self, phy_addr: Addr64T) -> Result<u16, MemResult>;
    fn read_memory_byte(&mut self, phy_addr: Addr64T) -> Result<u8, MemResult>;
    fn write_memory_dword(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T;
    fn write_memory_word(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T;
    fn write_memory_hword(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T;
    fn write_memory_byte(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T;

    fn fetch_bus(&mut self) -> Result<InstT, MemResult>;
    fn read_bus_dword(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult>;
    fn read_bus_word(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult>;
    fn read_bus_hword(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult>;
    fn read_bus_byte(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult>;
    fn write_bus_dword(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult;
    fn write_bus_word(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult;
    fn write_bus_hword(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult;
    fn write_bus_byte(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult;

    fn read_reg(&mut self, reg_addr: RegAddrT) -> Xlen64T;
    fn write_reg(&mut self, reg_addr: RegAddrT, data: Xlen64T);

    fn read_freg32(&mut self, reg_addr: RegAddrT) -> Xlen64T;
    fn write_freg32(&mut self, reg_addr: RegAddrT, data: Xlen64T);

    fn read_freg64(&mut self, reg_addr: RegAddrT) -> Xlen64T;
    fn write_freg64(&mut self, reg_addr: RegAddrT, data: Xlen64T);

    // fn decode_inst(&mut self, inst: InstT) -> RiscvInstId;
    // fn execute_inst(&mut self, dec_inst: RiscvInstId, inst: InstT, step: u32);

    // fn print_priv_mode(priv_mode: PrivMode) -> &str;

    // fn get_priv_mode(&mut self) -> PrivMode;
    fn set_priv_mode(&mut self, priv_mode: PrivMode);

    // fn get_max_priv(&mut self) -> PrivMode;
    fn set_max_priv(&mut self, maxpriv: PrivMode);

    fn get_vm_mode(&mut self) -> VMMode;
    fn set_vm_mode(&mut self, vmmode: VMMode);

    fn get_is_finish_cpu(&mut self) -> bool;

    fn get_tohost(&mut self) -> Xlen64T;
    fn get_fromhost(&mut self) -> Xlen64T;
}

impl Riscv64Core for Riscv64Env {
    fn get_rs1_addr(inst: InstT) -> RegAddrT {
        return ((inst >> 15) & 0x1f) as RegAddrT;
    }
    fn get_rs2_addr(inst: InstT) -> RegAddrT {
        return ((inst >> 20) & 0x1f) as RegAddrT;
    }
    fn get_rs3_addr(inst: InstT) -> RegAddrT {
        return ((inst >> 27) & 0x1f) as RegAddrT;
    }
    fn get_rd_addr(inst: InstT) -> RegAddrT {
        return ((inst >> 7) & 0x1f) as RegAddrT;
    }

    fn read_reg(&mut self, reg_addr: RegAddrT) -> Xlen64T {
        let ret_val: Xlen64T;

        if reg_addr == 0 {
            ret_val = 0;
        } else {
            ret_val = self.m_regs[reg_addr as usize];
        }

        let read_reg_trace = TraceInfo::XRegRead { addr: reg_addr,
                                                   value: ret_val };
        self.m_trace.m_trace_info.push(read_reg_trace);

        return ret_val;
    }

    fn write_reg(&mut self, reg_addr: RegAddrT, data: Xlen64T) {
        if reg_addr != 0 {
            let write_reg_trace = TraceInfo::XRegWrite { addr: reg_addr,
                                                         value: data };
            self.m_trace.m_trace_info.push(write_reg_trace);

            self.m_regs[reg_addr as usize] = data;
        }
    }

    fn read_freg32(&mut self, reg_addr: RegAddrT) -> Xlen64T {
        let ret_val: Xlen64T = self.m_fregs[reg_addr as usize];

        let read_reg_trace = TraceInfo::F32RegRead { addr: reg_addr,
                                                     value: ret_val as XlenT};

        self.m_trace.m_trace_info.push(read_reg_trace);

        return ret_val;
    }

    fn write_freg32(&mut self, reg_addr: RegAddrT, data: Xlen64T) {
        let write_reg_trace = TraceInfo::F32RegWrite { addr: reg_addr,
                                                       value: data as XlenT };

        self.m_trace.m_trace_info.push(write_reg_trace);

        self.m_fregs[reg_addr as usize] = data;
    }

    fn read_freg64(&mut self, reg_addr: RegAddrT) -> Xlen64T {
        let ret_val: Xlen64T = self.m_fregs[reg_addr as usize];

        let read_reg_trace = TraceInfo::F64RegRead { addr: reg_addr,
                                                     value: ret_val};

        self.m_trace.m_trace_info.push(read_reg_trace);

        return ret_val;
    }

    fn write_freg64(&mut self, reg_addr: RegAddrT, data: Xlen64T) {
        let write_reg_trace = TraceInfo::F64RegWrite { addr: reg_addr,
                                                       value: data };

        self.m_trace.m_trace_info.push(write_reg_trace);

        self.m_fregs[reg_addr as usize] = data;
    }

    fn set_pc(&mut self, addr: Addr64T) {
        self.m_previous_pc = self.m_pc;
        self.m_pc = addr;
    }

    fn get_pc(&mut self) -> Addr64T {
        return self.m_pc;
    }

    fn get_previous_pc(&mut self) -> Addr64T {
        return self.m_previous_pc;
    }

    // fn get_priv_mode(&mut self) -> PrivMode {
    //     return self.m_priv;
    // }
    fn set_priv_mode(&mut self, priv_mode: PrivMode) {
        self.m_priv = priv_mode;
        // FlushTlb();
    }

    // fn get_max_priv(&mut self) -> PrivMode {
    //     return self.m_maxpriv;
    // }

    fn set_max_priv(&mut self, maxpriv: PrivMode) {
        self.m_maxpriv = maxpriv;
    }

    fn set_vm_mode(&mut self, vmmode: VMMode) {
        self.m_vmmode = vmmode;
    }

    fn read_memory_dword(&mut self, phy_addr: Addr64T) -> Result<u64, MemResult> {
        let mut ret_val: u64 = 0;
        for idx in 0..8 {
            match self.read_memory_byte(phy_addr+idx) {
                Ok(val) => { ret_val = ret_val | (val as u64).wrapping_shl(idx as u32 * 8) },
                Err(result) => { println!("dword error {:016x}", phy_addr+idx); return Err(result) },
            }
        }
        Ok(ret_val)
    }

    fn read_memory_word(&mut self, phy_addr: Addr64T) -> Result<u32, MemResult> {
        if phy_addr == self.m_tohost_addr {
            return Ok(self.m_tohost as u32);
        } else if phy_addr == self.m_fromhost_addr {
            return Ok(self.m_fromhost as u32);
        } else {
            let mut ret_val: u32 = 0;
            for idx in 0..4 {
                match self.read_memory_byte(phy_addr+idx) {
                    Ok(val) => ret_val = ret_val | (val as u32).wrapping_shl(idx as u32 * 8),
                    Err(result) => return Err(result),
                }
            }
            Ok(ret_val)
        }
    }

    fn read_memory_hword(&mut self, phy_addr: Addr64T) -> Result<u16, MemResult> {
        let mut ret_val: u16 = 0;
        for idx in 0..2 {
            match self.read_memory_byte(phy_addr+idx) {
                Ok(val) => ret_val = ret_val | (val as u16).wrapping_shl(idx as u32 * 8),
                Err(result) => return Err(result),
            }
        }
        Ok(ret_val)
    }

    fn read_memory_byte(&mut self, phy_addr: Addr64T) -> Result<u8, MemResult> {
        if phy_addr < DRAM_BASE {
            println!("phys_addr = {:x}", phy_addr);
        }
        assert!(phy_addr >= (DRAM_BASE as Addr64T));
        let base_addr: Addr64T = phy_addr - (DRAM_BASE as Addr64T);

        match self.m_memory.get(&base_addr) {
            Some(value) => Ok(*value),
            None => Ok(0),
            // None => Err(MemResult::NotDefined),
        }
    }

    fn write_memory_dword(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T {
        if phy_addr == self.m_tohost_addr {
            self.m_finish_cpu = true;
            self.m_tohost = data;
        } else if phy_addr == self.m_fromhost_addr {
            self.m_finish_cpu = true;
            self.m_fromhost = data;
        } else {
            self.write_memory_byte(phy_addr + 0, (data >> 0) & 0xff);
            self.write_memory_byte(phy_addr + 1, (data >> 8) & 0xff);
            self.write_memory_byte(phy_addr + 2, (data >> 16) & 0xff);
            self.write_memory_byte(phy_addr + 3, (data >> 24) & 0xff);
            self.write_memory_byte(phy_addr + 4, (data >> 32) & 0xff);
            self.write_memory_byte(phy_addr + 5, (data >> 40) & 0xff);
            self.write_memory_byte(phy_addr + 6, (data >> 48) & 0xff);
            self.write_memory_byte(phy_addr + 7, (data >> 56) & 0xff);
        }
        return 0;
    }

    fn write_memory_word(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T {
        if phy_addr == self.m_tohost_addr {
            self.m_finish_cpu = true;
            self.m_tohost = data;
        } else if phy_addr == self.m_fromhost_addr {
            self.m_finish_cpu = true;
            self.m_fromhost = data;
        } else {
            self.write_memory_byte(phy_addr + 0, (data >> 0) & 0xff);
            self.write_memory_byte(phy_addr + 1, (data >> 8) & 0xff);
            self.write_memory_byte(phy_addr + 2, (data >> 16) & 0xff);
            self.write_memory_byte(phy_addr + 3, (data >> 24) & 0xff);
        }
        return 0;
    }

    fn write_memory_hword(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T {
        self.write_memory_byte(phy_addr + 0, (data >> 0) & 0xff);
        self.write_memory_byte(phy_addr + 1, (data >> 8) & 0xff);

        return 0;
    }

    fn write_memory_byte(&mut self, phy_addr: Addr64T, data: Xlen64T) -> Xlen64T {
        assert!(phy_addr >= (DRAM_BASE as Addr64T));
        let base_addr: Addr64T = phy_addr - (DRAM_BASE as Addr64T);

        self.m_memory.insert(base_addr, data as u8);
        return 0;
    }

    fn fetch_bus(&mut self) -> Result<InstT, MemResult> {
        return match self.convert_virtual_address(self.m_pc, MemAccType::Fetch) {
            Ok(phy_addr) => {
                let uext_phy_addr = self.uext_xlen(phy_addr as Xlen64T);
                match self.read_memory_word(uext_phy_addr) {
                    Ok(val) => Ok(val as InstT),
                    Err(result) => Err(result),
                }
            },
            Err(result) => Err(result),
        }
    }

    fn read_bus_dword(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult> {
        return match self.convert_virtual_address(addr, MemAccType::Read) {
            Ok(phy_addr) => match self.read_memory_dword(phy_addr) {
                Ok(ret_value) => {
                    let ret_val: Xlen64T = ret_value as Xlen64T;
                    let read_mem_trace = TraceInfo::MemRead { addr: addr,
                                                              value: ret_val,
                                                              memresult: MemResult::NoExcept };
                    self.m_trace.m_trace_info.push(read_mem_trace);

                    Ok(ret_val as Xlen64T)
                },
                Err(result) => { return Err(result) },
            },
            Err(result) => Err(result),
        }
    }

    fn read_bus_word(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult> {
        return match self.convert_virtual_address(addr, MemAccType::Read) {
            Ok(phy_addr) => {
                let uext_phy_addr = self.uext_xlen(phy_addr as Xlen64T);
                match self.read_memory_word(uext_phy_addr) {
                    Ok(value) => {
                        let ret_val: u32 = value;

                        let read_mem_trace = TraceInfo::MemRead { addr: addr,
                                                                  value: ret_val as Xlen64T,
                                                                  memresult: MemResult::NoExcept };
                        self.m_trace.m_trace_info.push(read_mem_trace);

                        Ok(ret_val as Xlen64T)
                    },
                    Err(result) => Err(result),
                }
            },
            Err(result) => Err(result),
        }
    }

    fn read_bus_hword(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult> {
        return match self.convert_virtual_address(addr, MemAccType::Read) {
            Ok(phy_addr) => match self.read_memory_hword(phy_addr) {
                Ok(val) => Ok(val as i64),
                Err(result) => Err(result),
            },
            Err(result) => Err(result),
        }
    }

    fn read_bus_byte(&mut self, addr: Addr64T) -> Result<Xlen64T, MemResult> {
        return match self.convert_virtual_address(addr, MemAccType::Read) {
            Ok(phy_addr) => match self.read_memory_byte(phy_addr) {
                Ok(val) => Ok(val as i64),
                Err(result) => Err(result),
            },
            Err(result) => Err(result),
        }
    }

    fn write_bus_dword(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult {
        return match self.convert_virtual_address(addr, MemAccType::Write) {
            Ok(phy_addr) => {
                let write_mem_trace = TraceInfo::MemWrite { addr: addr,
                                                            value: data,
                                                            memresult: MemResult::NoExcept };
                self.m_trace.m_trace_info.push(write_mem_trace);
                self.write_memory_dword(phy_addr, data);

                MemResult::NoExcept
            },
            Err(result) => result,
        }
    }

    fn write_bus_word(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult {
        return match self.convert_virtual_address(addr, MemAccType::Write) {
            Ok(phy_addr) => {
                let uext_phy_addr = self.uext_xlen(phy_addr as Xlen64T);
                let write_mem_trace = TraceInfo::MemWrite { addr: addr,
                                                            value: data,
                                                            memresult: MemResult::NoExcept };
                self.m_trace.m_trace_info.push(write_mem_trace);
                self.write_memory_word(uext_phy_addr, data);

                MemResult::NoExcept
            },
            Err(result) => result,
        }
    }

    fn write_bus_hword(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult {
        return match self.convert_virtual_address(addr, MemAccType::Write) {
            Ok(phy_addr) => {

                self.write_memory_hword(phy_addr, data);
                MemResult::NoExcept
            },
            Err(result) => result,
        }
    }

    fn write_bus_byte(&mut self, addr: Addr64T, data: Xlen64T) -> MemResult {
        return match self.convert_virtual_address(addr, MemAccType::Write) {
            Ok(phy_addr) => {

                self.write_memory_byte(phy_addr, data);
                MemResult::NoExcept
            },
            Err(result) => result,
        }
    }

    fn get_vm_mode(&mut self) -> VMMode {
        let satp_val = self.m_csr.csrrs(CsrAddr::Satp, 0) as Xlen64T; // SATP
        let mode = match self.m_xlen {
            32 => Self::extract_bit_field(satp_val, 31, 31),
            64 => Self::extract_bit_field(satp_val, 63, 60),
            _ => panic!("Internal Error: XLEN should either 32 or 64"),
        };
        return if self.m_priv == PrivMode::Machine {
            VMMode::Mbare
        } else {
            let v_mode = VMMode::from(mode);
            if v_mode == VMMode::Mbare || v_mode == VMMode::Sv32 ||
                v_mode == VMMode::Sv39 || v_mode == VMMode::Sv48 || v_mode == VMMode::Sv57 || v_mode == VMMode::Sv64 {
                return v_mode
            } else {
                panic!("Error: illegal VM Mode in SATP {:}", mode)
            }
        };
    }

    // fn print_priv_mode(priv_mode: PrivMode) -> &str {
    //     return match priv_mode {
    //         PrivMode::User => "UserMode",
    //         PrivMode::Supervisor => "SuprevisorMode",
    //         PrivMode::Hypervisor => "HypervisorMode",
    //         PrivMode::Machine => "MachineMode",
    //         _ => "<Internal Error: PrivMode is illegal>",
    //     };
    // }

    fn get_is_finish_cpu(&mut self) -> bool {
        return self.m_finish_cpu;
    }

    fn get_tohost(&mut self) -> Xlen64T {
        return self.m_tohost;
    }
    fn get_fromhost(&mut self) -> Xlen64T {
        return self.m_fromhost;
    }
}
