use num::iter::range;
use std::num::Wrapping;

use crate::riscv_csr::CsrAddr;
use crate::riscv_csr::Riscv64Csr;
use crate::riscv_csr::RiscvCsr;
use crate::riscv_csr::RiscvCsrBase;

pub type XlenType = i32;
pub type UXlenType = u32;
pub type InstType = u32;
pub type AddrType = u32;
pub type RegAddrType = u8;

pub const DRAM_BASE: AddrType = 0x8000_0000;
pub const DRAM_SIZE: usize = 0x01_0000;

pub enum RiscvBitMode {
    Bit32 = 0,
    Bit64 = 1,
}

pub enum RiscvInst {
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    LUI,
    AUIPC,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SW,
    SH,
    SB,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
    FENCE,
    FENCEI,
    ECALL,
    EBREAK,
    MRET,
    SRET,
    URET,
    NOP,
    WFI,
}

pub enum PrivMode {
    PrivUser = 0,
    PrivSupervisor = 1,
    PrivHypervisor = 2,
    PrivMachine = 3,
}

pub enum MemAccType {
    Fetch = 0,
    Write = 1,
    Read = 2,
}

pub enum MemResult {
    NoExcept = 0,
    MisAlign = 1 << 0,
    NotDefined = 1 << 1,
    NewRegion = 1 << 2,
    TlbError = 1 << 3,
}

pub enum ExceptCode {
    InstAddrMisalign = 0,
    InstAccessFault = 1,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisalign = 4,
    LoadAccessFault = 5,
    StoreAddrMisalign = 6,
    StoreAccessFault = 7,
    EcallFromUMode = 8,
    EcallFromSMode = 9,
    EcallFromHMode = 10,
    EcallFromMMode = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

pub enum MemType {
    LOAD = 0,
    STORE = 1,
}

pub enum MemSize {
    BYTE = 0,
    HWORD = 1,
    WORD = 2,
    DWORD = 3,
}

pub struct EnvBase {
    m_bitmode: RiscvBitMode,

    pub m_pc: AddrType,
    pub m_regs: [XlenType; 32],
    pub m_memory: [u8; DRAM_SIZE], // memory
    pub m_csr: RiscvCsr,

    m_tohost_addr: AddrType,
    m_fromhost_addr: AddrType,
    m_tohost: XlenType,
    m_fromhost: XlenType,

    m_finish_cpu: bool,
}

impl EnvBase {
    pub fn new() -> EnvBase {
        EnvBase {
            m_bitmode: RiscvBitMode::Bit32,
            m_pc: DRAM_BASE as AddrType,
            m_memory: [0; DRAM_SIZE],
            m_regs: [0; 32],
            m_finish_cpu: false,
            m_fromhost_addr: (DRAM_BASE + 0x001000) as AddrType,
            m_tohost_addr: (DRAM_BASE + 0x001000) as AddrType,
            m_fromhost: 0,
            m_tohost: 0,
            m_csr: RiscvCsr {
                m_mcycle: RiscvCsrBase { m_csr: 0 },
                m_minstret: RiscvCsrBase { m_csr: 0 },
                m_mimpid: RiscvCsrBase { m_csr: 0 },
                m_marchid: RiscvCsrBase { m_csr: 0 },
                m_mvendorid: RiscvCsrBase { m_csr: 0 },
                m_misa: RiscvCsrBase { m_csr: 0 },
                m_mstatus: RiscvCsrBase { m_csr: 0 },
                m_mtvec: RiscvCsrBase { m_csr: 0 },
                m_mip: RiscvCsrBase { m_csr: 0 },
                m_mie: RiscvCsrBase { m_csr: 0 },
                m_mscratch: RiscvCsrBase { m_csr: 0 },
                m_mepc: RiscvCsrBase { m_csr: 0 },
                m_mtval: RiscvCsrBase { m_csr: 0 },
                m_mcause: RiscvCsrBase { m_csr: 0 },
                m_mhartid: RiscvCsrBase { m_csr: 0 },
                m_dcsr: RiscvCsrBase { m_csr: 0 },
                m_dpc: RiscvCsrBase { m_csr: 0 },
                m_dscratch: RiscvCsrBase { m_csr: 0 },
                m_medeleg: RiscvCsrBase { m_csr: 0 },
            },
        }
    }

    fn extend_sign(data: XlenType, msb: XlenType) -> XlenType {
        let mask: XlenType = 1 << msb; // mask can be pre-computed if b is fixed
        let res_data = data & ((1 << (msb + 1)) - 1); // (Skip this if bits in x above position b are already zero.)
        return (res_data ^ mask) - mask;
    }

    fn extract_bit_field(hex: InstType, left: XlenType, right: XlenType) -> XlenType {
        let mask: XlenType = (1 << (left - right + 1)) - 1;
        return ((hex >> right) & (mask as UXlenType)) as XlenType;
    }

    fn extract_uj_field(hex: InstType) -> XlenType {
        let i24_21 = Self::extract_bit_field(hex, 24, 21) & 0x0f;
        let i30_25 = Self::extract_bit_field(hex, 30, 25) & 0x03f;
        let i20_20 = Self::extract_bit_field(hex, 20, 20) & 0x01;
        let i19_12 = Self::extract_bit_field(hex, 19, 12) & 0x0ff;
        let i31_31 = Self::extract_bit_field(hex, 31, 31) & 0x01;

        let u_res: XlenType =
            (i31_31 << 20) | (i19_12 << 12) | (i20_20 << 11) | (i30_25 << 5) | (i24_21 << 1);
        return Self::extend_sign(u_res, 20);
    }

    fn extract_ifield(hex: InstType) -> XlenType {
        let uimm32: XlenType = Self::extract_bit_field(hex, 31, 20);
        return Self::extend_sign(uimm32, 11);
    }

    fn extract_shamt_field(hex: InstType) -> XlenType {
        return Self::extract_bit_field(hex, 24, 20);
    }

    fn extract_sb_field(hex: InstType) -> XlenType {
        let i07_07: XlenType = Self::extract_bit_field(hex, 7, 7) & 0x01;
        let i11_08: XlenType = Self::extract_bit_field(hex, 11, 8) & 0x0f;
        let i30_25: XlenType = Self::extract_bit_field(hex, 30, 25) & 0x03f;
        let i31_31: XlenType = Self::extract_bit_field(hex, 31, 31) & 0x01;

        let u_res: XlenType = (i31_31 << 12) | (i07_07 << 11) | (i30_25 << 5) | (i11_08 << 1);
        return Self::extend_sign(u_res, 12);
    }

    fn extract_sfield(hex: InstType) -> XlenType {
        let i11_07: XlenType = Self::extract_bit_field(hex, 11, 7) & 0x01f;
        let i31_25: XlenType = Self::extract_bit_field(hex, 31, 25) & 0x07f;

        let u_res: XlenType = (i31_25 << 5) | (i11_07 << 0);

        return Self::extend_sign(u_res, 11);
    }

    fn GetPrivMode() -> PrivMode {
        return m_priv;
    }
    fn SetPrivMode(priv_mode: PrivMode) {
        m_priv = priv_mode;
        FlushTlb();
    }

    fn sext_xlen(hex: InstType) -> XlenType {
        return hex as XlenType;
    }
    fn uext_xlen(hex: InstType) -> UXlenType {
        return hex as UXlenType;
    }
}

pub trait Riscv64Core {
    fn get_rs1_addr(inst: InstType) -> RegAddrType;
    fn get_rs2_addr(inst: InstType) -> RegAddrType;
    fn get_rd_addr(inst: InstType) -> RegAddrType;

    fn fetch_memory(&mut self) -> XlenType;
    fn read_memory_word(&mut self, addr: AddrType) -> XlenType;
    fn read_memory_hword(&mut self, addr: AddrType) -> XlenType;
    fn read_memory_byte(&mut self, addr: AddrType) -> XlenType;
    fn write_memory_word(&mut self, addr: AddrType, data: XlenType) -> XlenType;
    fn write_memory_hword(&mut self, addr: AddrType, data: XlenType) -> XlenType;
    fn write_memory_byte(&mut self, addr: AddrType, data: XlenType) -> XlenType;

    fn read_reg(&mut self, reg_addr: RegAddrType) -> i32;
    fn write_reg(&mut self, reg_addr: RegAddrType, data: XlenType);

    fn decode_inst(&mut self, inst: XlenType) -> RiscvInst;
    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstType);

    fn mem_access(
        &mut self,
        op: MemType,
        size: MemSize,
        data: XlenType,
        addr: AddrType,
    ) -> XlenType;

    fn WalkPageTable(
        &mut self,
        vaddr: AddrType,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u32>,
        pte_len: Vec<AddrType>,
        pte_idx: Vec<AddrType>,
        vpn_len: Vec<AddrType>,
        vpn_idx: Vec<u32>,
        PAGESIZE: i32,
        PTESIZE: i32,
    ) -> MemResult;

    fn GenerateException(&mut self, code: ExceptCode, tval: XlenType);
    fn PrintPrivMode(priv_mode: PrivMode) -> String;

    fn get_is_finish_cpu(&mut self) -> bool;

    fn get_tohost(&mut self) -> XlenType;
    fn get_fromhost(&mut self) -> XlenType;
}

impl Riscv64Core for EnvBase {
    fn get_rs1_addr(inst: InstType) -> RegAddrType {
        return ((inst >> 15) & 0x1f) as RegAddrType;
    }
    fn get_rs2_addr(inst: InstType) -> RegAddrType {
        return ((inst >> 20) & 0x1f) as RegAddrType;
    }
    fn get_rd_addr(inst: InstType) -> RegAddrType {
        return ((inst >> 7) & 0x1f) as RegAddrType;
    }

    fn read_reg(&mut self, reg_addr: RegAddrType) -> XlenType {
        if reg_addr == 0 {
            return 0;
        } else {
            return self.m_regs[reg_addr as usize];
        }
    }

    fn write_reg(&mut self, reg_addr: RegAddrType, data: XlenType) {
        if reg_addr != 0 {
            self.m_regs[reg_addr as usize] = data;
            println!("     x{:02} <= {:08x}", reg_addr, data);
        }
    }

    fn fetch_memory(&mut self) -> XlenType {
        let base_addr: AddrType = self.m_pc - DRAM_BASE;
        let fetch_data = ((self.m_memory[base_addr as usize + 3] as XlenType) << 24)
            | ((self.m_memory[base_addr as usize + 2] as XlenType) << 16)
            | ((self.m_memory[base_addr as usize + 1] as XlenType) << 8)
            | ((self.m_memory[base_addr as usize + 0] as XlenType) << 0);
        return fetch_data;
    }

    fn read_memory_word(&mut self, addr: AddrType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        return ((self.m_memory[base_addr as usize + 3] as XlenType) << 24)
            | ((self.m_memory[base_addr as usize + 2] as XlenType) << 16)
            | ((self.m_memory[base_addr as usize + 1] as XlenType) << 8)
            | ((self.m_memory[base_addr as usize + 0] as XlenType) << 0);
    }

    fn read_memory_hword(&mut self, addr: AddrType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        return ((self.m_memory[base_addr as usize + 1] as XlenType) << 8)
            | ((self.m_memory[base_addr as usize + 0] as XlenType) << 0);
    }

    fn read_memory_byte(&mut self, addr: AddrType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        return self.m_memory[base_addr as usize + 0] as XlenType;
    }

    fn write_memory_word(&mut self, addr: AddrType, data: XlenType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        self.m_memory[base_addr as usize + 0] = ((data >> 0) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 1] = ((data >> 8) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 2] = ((data >> 16) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 3] = ((data >> 24) & 0x0ff) as u8;

        return 0;
    }

    fn write_memory_hword(&mut self, addr: AddrType, data: XlenType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        self.m_memory[base_addr as usize + 0] = ((data >> 0) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 1] = ((data >> 8) & 0x0ff) as u8;

        return 0;
    }

    fn write_memory_byte(&mut self, addr: AddrType, data: XlenType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        self.m_memory[base_addr as usize] = (data & 0xff) as u8;
        return 0;
    }

    fn decode_inst(&mut self, inst: XlenType) -> RiscvInst {
        let opcode = inst & 0x7f;
        let funct3 = (inst >> 12) & 0x07;
        let funct5 = (inst >> 25) & 0x7f;
        let imm12 = (inst >> 20) & 0xfff;

        let dec_inst: RiscvInst;

        match opcode {
            0x0f => match funct3 {
                0b000 => dec_inst = RiscvInst::FENCE,
                0b001 => dec_inst = RiscvInst::FENCEI,
                _ => dec_inst = RiscvInst::NOP,
            },
            0x33 => match funct5 {
                0b0000000 => match funct3 {
                    0b000 => dec_inst = RiscvInst::ADD,
                    0b001 => dec_inst = RiscvInst::SLL,
                    0b010 => dec_inst = RiscvInst::SLT,
                    0b011 => dec_inst = RiscvInst::SLTU,
                    0b100 => dec_inst = RiscvInst::XOR,
                    0b101 => dec_inst = RiscvInst::SRL,
                    0b110 => dec_inst = RiscvInst::OR,
                    0b111 => dec_inst = RiscvInst::AND,
                    _ => dec_inst = RiscvInst::NOP,
                },
                0b0100000 => match funct3 {
                    0b000 => dec_inst = RiscvInst::SUB,
                    0b101 => dec_inst = RiscvInst::SRA,
                    _ => dec_inst = RiscvInst::NOP,
                },
                0b0000001 => match funct3 {
                    0b000 => dec_inst = RiscvInst::MUL,
                    0b001 => dec_inst = RiscvInst::MULH,
                    0b010 => dec_inst = RiscvInst::MULHSU,
                    0b011 => dec_inst = RiscvInst::MULHU,
                    0b100 => dec_inst = RiscvInst::DIV,
                    0b101 => dec_inst = RiscvInst::DIVU,
                    0b110 => dec_inst = RiscvInst::REM,
                    0b111 => dec_inst = RiscvInst::REMU,
                    _ => dec_inst = RiscvInst::NOP,
                },
                _ => dec_inst = RiscvInst::NOP,
            },
            0x03 => match funct3 {
                0b000 => dec_inst = RiscvInst::LB,
                0b001 => dec_inst = RiscvInst::LH,
                0b010 => dec_inst = RiscvInst::LW,
                0b100 => dec_inst = RiscvInst::LBU,
                0b101 => dec_inst = RiscvInst::LHU,
                _ => dec_inst = RiscvInst::NOP,
            },
            0x23 => match funct3 {
                0b000 => dec_inst = RiscvInst::SB,
                0b001 => dec_inst = RiscvInst::SH,
                0b010 => dec_inst = RiscvInst::SW,
                _ => dec_inst = RiscvInst::NOP,
            },
            0x37 => dec_inst = RiscvInst::LUI,
            0x17 => dec_inst = RiscvInst::AUIPC,
            0x63 => match funct3 {
                0b000 => dec_inst = RiscvInst::BEQ,
                0b001 => dec_inst = RiscvInst::BNE,
                0b100 => dec_inst = RiscvInst::BLT,
                0b101 => dec_inst = RiscvInst::BGE,
                0b110 => dec_inst = RiscvInst::BLTU,
                0b111 => dec_inst = RiscvInst::BGEU,
                _ => dec_inst = RiscvInst::NOP,
            },
            0x13 => match funct3 {
                0b000 => dec_inst = RiscvInst::ADDI,
                0b010 => dec_inst = RiscvInst::SLTI,
                0b011 => dec_inst = RiscvInst::SLTIU,
                0b100 => dec_inst = RiscvInst::XORI,
                0b110 => dec_inst = RiscvInst::ORI,
                0b111 => dec_inst = RiscvInst::ANDI,
                0b001 => dec_inst = RiscvInst::SLLI,
                0b101 => match funct5 {
                    0b0000000 => dec_inst = RiscvInst::SRLI,
                    0b0100000 => dec_inst = RiscvInst::SRAI,
                    _ => dec_inst = RiscvInst::NOP,
                },
                _ => dec_inst = RiscvInst::NOP,
            },
            0x6f => dec_inst = RiscvInst::JAL,
            0x67 => dec_inst = RiscvInst::JALR,
            0x73 => match funct3 {
                0x000 => match imm12 {
                    0x000 => dec_inst = RiscvInst::ECALL,
                    0x001 => dec_inst = RiscvInst::EBREAK,
                    0x002 => dec_inst = RiscvInst::URET,
                    0x102 => dec_inst = RiscvInst::SRET,
                    0x302 => dec_inst = RiscvInst::MRET,
                    _ => dec_inst = RiscvInst::NOP,
                },
                0b001 => dec_inst = RiscvInst::CSRRW,
                0b010 => dec_inst = RiscvInst::CSRRS,
                0b011 => dec_inst = RiscvInst::CSRRC,
                0b101 => dec_inst = RiscvInst::CSRRWI,
                0b110 => dec_inst = RiscvInst::CSRRSI,
                0b111 => dec_inst = RiscvInst::CSRRCI,
                _ => dec_inst = RiscvInst::NOP,
            },
            _ => dec_inst = RiscvInst::WFI,
        }

        return dec_inst;
    }

    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstType) {
        println!(
            "{:08x} : {:08x} // DASM({:08x})",
            self.m_pc as u32, inst as u32, inst as u32
        );

        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let csr_addr = CsrAddr::from_u64(((inst >> 20) & 0x0fff) as u64);

        let mut update_pc = false;

        match dec_inst {
            RiscvInst::CSRRW => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenType = self.m_csr.csrrw(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRS => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenType = self.m_csr.csrrs(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRC => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenType = self.m_csr.csrrc(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRWI => {
                let zimm: XlenType = ((inst >> 15) & 0x1f) as XlenType;
                let reg_data: XlenType = self.m_csr.csrrw(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRSI => {
                let zimm: XlenType = ((inst >> 15) & 0x1f) as XlenType;
                let reg_data: XlenType = self.m_csr.csrrs(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRCI => {
                let zimm: XlenType = ((inst >> 15) & 0x1f) as XlenType;
                let reg_data: XlenType = self.m_csr.csrrc(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LUI => {
                let mut imm: XlenType =
                    Self::extend_sign(Self::extract_bit_field(inst, 31, 12), 19);
                imm = imm << 12;
                self.write_reg(rd, imm);
            }
            RiscvInst::AUIPC => {
                let mut imm: XlenType =
                    Self::extend_sign(Self::extract_bit_field(inst, 31, 12), 19);
                imm = (imm << 12).wrapping_add(self.m_pc as XlenType);
                self.write_reg(rd, imm);
            }
            RiscvInst::LB => {
                let rs1_data = self.read_reg(rs1);
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let mut reg_data: XlenType =
                    self.mem_access(MemType::LOAD, MemSize::BYTE, rs1_data, addr as AddrType);
                reg_data = Self::extend_sign(reg_data, 7);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LH => {
                let rs1_data = self.read_reg(rs1);
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let mut reg_data: XlenType =
                    self.mem_access(MemType::LOAD, MemSize::HWORD, rs1_data, addr as AddrType);
                reg_data = Self::extend_sign(reg_data, 15);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LW => {
                let rs1_data = self.read_reg(rs1);
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let reg_data: XlenType =
                    self.mem_access(MemType::LOAD, MemSize::WORD, rs1_data, addr as AddrType);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LBU => {
                let rs1_data = self.read_reg(rs1);
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let reg_data: UXlenType =
                    self.mem_access(MemType::LOAD, MemSize::BYTE, rs1_data, addr as AddrType)
                        as UXlenType;
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::LHU => {
                let rs1_data = self.read_reg(rs1);
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let reg_data: UXlenType =
                    self.mem_access(MemType::LOAD, MemSize::HWORD, rs1_data, addr as AddrType)
                        as UXlenType;
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::ADDI => {
                let rs1_data = self.read_reg(rs1);
                let imm_data = Self::extract_ifield(inst);
                let reg_data: XlenType = rs1_data.wrapping_add(imm_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLTI => {
                let reg_data: bool = self.read_reg(rs1) < Self::extract_ifield(inst);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SLTIU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlenType) < (Self::extract_ifield(inst) as UXlenType);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::XORI => {
                let reg_data: XlenType = self.read_reg(rs1) ^ Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::ORI => {
                let reg_data: XlenType = self.read_reg(rs1) | Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::ANDI => {
                let reg_data: XlenType = self.read_reg(rs1) & Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLLI => {
                let reg_data: XlenType = self.read_reg(rs1) << Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SRLI => {
                let reg_data: UXlenType =
                    (self.read_reg(rs1) as UXlenType) >> Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SRAI => {
                let reg_data: XlenType = self.read_reg(rs1) >> Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data);
            }

            RiscvInst::ADD => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenType = rs1_data.wrapping_add(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SUB => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenType = rs1_data.wrapping_sub(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlenType = self.read_reg(rs2) as UXlenType;
                let reg_data: XlenType = rs1_data.wrapping_shl(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLT => {
                let reg_data: bool = self.read_reg(rs1) < self.read_reg(rs2);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SLTU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlenType) < (self.read_reg(rs2) as UXlenType);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::XOR => {
                let reg_data: XlenType = self.read_reg(rs1) ^ self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SRL => {
                let rs1_data = self.read_reg(rs1) as UXlenType;
                let rs2_data = self.read_reg(rs2);
                let reg_data = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SRA => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlenType = self.read_reg(rs2) as UXlenType;
                let reg_data: XlenType = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data);
            }

            RiscvInst::MUL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenType = rs1_data.wrapping_mul(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::MULH => {
                let rs1_data: i64 = self.read_reg(rs1) as i64;
                let rs2_data: i64 = self.read_reg(rs2) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0x0ffffffff;
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::MULHSU => {
                let rs1_data: i64 = (self.read_reg(rs1) as i32) as i64;
                let rs2_data: i64 = (self.read_reg(rs2) as u32) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::MULHU => {
                let rs1_data: u64 = (self.read_reg(rs1) as u32) as u64;
                let rs2_data: u64 = (self.read_reg(rs2) as u32) as u64;
                let mut reg_data: u64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as XlenType);
            }

            RiscvInst::REM => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenType;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else if rs2_data == -1 {
                    reg_data = 0;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, reg_data);
            }
            RiscvInst::REMU => {
                let rs1_data: UXlenType = self.read_reg(rs1) as UXlenType;
                let rs2_data: UXlenType = self.read_reg(rs2) as UXlenType;
                let reg_data: UXlenType;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, reg_data as XlenType);
            }

            RiscvInst::DIV => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenType;
                if rs2_data == 0 {
                    reg_data = -1;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data);
            }
            RiscvInst::DIVU => {
                let rs1_data: UXlenType = self.read_reg(rs1) as UXlenType;
                let rs2_data: UXlenType = self.read_reg(rs2) as UXlenType;
                let reg_data: UXlenType;
                if rs2_data == 0 {
                    reg_data = 0xffffffff;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data as XlenType);
            }

            RiscvInst::OR => {
                let reg_data: XlenType = self.read_reg(rs1) | self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::AND => {
                let reg_data: XlenType = self.read_reg(rs1) & self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SB => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrType = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrType;
                self.mem_access(MemType::STORE, MemSize::BYTE, rs2_data, addr);
            }
            RiscvInst::SH => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrType = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrType;
                self.mem_access(MemType::STORE, MemSize::HWORD, rs2_data, addr);
            }
            RiscvInst::SW => {
                let rs2_data = self.read_reg(rs2);
                let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
                self.mem_access(MemType::STORE, MemSize::WORD, rs2_data, addr as AddrType);
            }
            RiscvInst::JAL => {
                let addr: AddrType = Self::extract_uj_field(inst) as AddrType;
                self.write_reg(rd, (self.m_pc + 4) as XlenType);
                self.m_pc = self.m_pc.wrapping_add(addr);
                update_pc = true;
            }
            RiscvInst::BEQ
            | RiscvInst::BNE
            | RiscvInst::BLT
            | RiscvInst::BGE
            | RiscvInst::BLTU
            | RiscvInst::BGEU => {
                let rs1_data: XlenType = self.read_reg(rs1);
                let rs2_data: XlenType = self.read_reg(rs2);
                let addr: AddrType = Self::extract_sb_field(inst) as AddrType;
                let jump_en: bool;
                match dec_inst {
                    RiscvInst::BEQ => jump_en = rs1_data == rs2_data,
                    RiscvInst::BNE => jump_en = rs1_data != rs2_data,
                    RiscvInst::BLT => jump_en = rs1_data < rs2_data,
                    RiscvInst::BGE => jump_en = rs1_data >= rs2_data,
                    RiscvInst::BLTU => jump_en = (rs1_data as UXlenType) < (rs2_data as UXlenType),
                    RiscvInst::BGEU => jump_en = (rs1_data as UXlenType) >= (rs2_data as UXlenType),
                    _ => panic!("Unknown value Branch"),
                }
                if jump_en {
                    self.m_pc = self.m_pc.wrapping_add(addr);
                    update_pc = true;
                }
            }
            RiscvInst::JALR => {
                let mut addr: AddrType = Self::extract_ifield(inst) as AddrType;
                let rs1_data: AddrType = self.read_reg(rs1) as AddrType;
                addr = rs1_data.wrapping_add(addr);
                addr = addr & (!0x01);

                self.write_reg(rd, (self.m_pc + 4) as XlenType);
                self.m_pc = addr;
                update_pc = true;
            }
            RiscvInst::FENCE => {}
            RiscvInst::FENCEI => {}
            RiscvInst::ECALL => {
                let mtvec: XlenType = self.m_csr.csrrs(CsrAddr::Mtvec, 0); // MTVEC

                self.m_csr.csrrw(CsrAddr::Mepc, self.m_pc as XlenType); // MEPC
                self.m_csr
                    .csrrw(CsrAddr::Mcause, ExceptCode::EcallFromMMode as XlenType); // MCAUSE
                self.m_csr.csrrw(CsrAddr::Mtval, 0); // MTVAL

                self.m_pc = mtvec as AddrType;
                update_pc = true;
            }
            RiscvInst::EBREAK => {}
            RiscvInst::URET => {}
            RiscvInst::SRET => {}
            RiscvInst::MRET => {
                let mepc: XlenType = self.m_csr.csrrs(CsrAddr::Mepc, 0); // MEPC
                self.m_pc = mepc as AddrType;
                update_pc = true;
            }
            _ => {}
        }

        if update_pc == false {
            self.m_pc += 4;
        }
    }

    fn mem_access(
        &mut self,
        op: MemType,
        size: MemSize,
        data: XlenType,
        addr: AddrType,
    ) -> XlenType {
        match op {
            MemType::STORE => {
                if addr == self.m_tohost_addr {
                    self.m_finish_cpu = true;
                    self.m_tohost = data;
                } else if addr == self.m_fromhost_addr {
                    self.m_finish_cpu = true;
                    self.m_fromhost = data;
                } else {
                    match size {
                        MemSize::BYTE => self.write_memory_byte(addr, data),
                        MemSize::HWORD => self.write_memory_hword(addr, data),
                        MemSize::WORD => self.write_memory_word(addr, data),
                        _ => 1,
                    };
                }
            }
            MemType::LOAD => {
                if addr == self.m_tohost_addr {
                    return self.m_tohost;
                } else if addr == self.m_fromhost_addr {
                    return self.m_fromhost;
                } else {
                    match size {
                        MemSize::BYTE => return self.read_memory_byte(addr),
                        MemSize::HWORD => return self.read_memory_hword(addr),
                        MemSize::WORD => return self.read_memory_word(addr),
                        _ => 1,
                    };
                }
            }
        }
        return 0;
    }

    fn WalkPageTable(
        &mut self,
        vaddr: AddrType,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u32>,
        pte_len: Vec<AddrType>,
        pte_idx: Vec<AddrType>,
        vpn_len: Vec<AddrType>,
        vpn_idx: Vec<u32>,
        PAGESIZE: i32,
        PTESIZE: i32,
    ) -> (MemResult, AddrType) {
        //===================
        // Simple TLB Search
        //===================
        // let vaddr_vpn: AddrType = (vaddr >> 12);
        // let vaddr_tag: u8 = vaddr_vpn & (tlb_width-1);
        // if (m_tlb_en[vaddr_tag] && m_tlb_tag[vaddr_tag] == vaddr_vpn) {
        //     let paddr:AddrType = (m_tlb_addr[vaddr_tag] & !0x0fff) + (vaddr & 0x0fff);
        //     let pte_val:u64 = m_tlb_addr[vaddr_tag] & 0x0ff;
        //
        //     if (!IsAllowedAccess ((pte_val >> 1) & 0x0f, acc_type, GetPrivMode())) {
        //         println! ("<Page Access Failed. Allowed Access Failed PTE_VAL=%016lx>\n", pte_val);
        //         return (MemResult::TlbError, paddr);
        //     }
        //     if (((pte_val & 0x40) == 0) || // PTE.A
        //         ((acc_type == MemAccType::WriteMemType) && (pte_val & 0x80) == 0)) { // PTE.D
        //         println!("<Access Fault : Page Permission Fault {:01x}>\n", (pte_val >> 1) & 0x0f);
        //         if (acc_type == MemAccType::FetchMemType) {
        //             GenerateException (self, ExceptCode::InstPageFault, vaddr);
        //         }
        //         return (MemResult::TlbError, paddr);
        //     }
        //     return (MemResult::NoExcept, paddr);
        // }

        let phy_addr: AddrType = vaddr & 0x0fff;

        let sptbr: u32 = self.m_csr.csrrs(CsrAddr::Satp, 0);
        let pte_base: u64 = ExtractSptbrPpnField(sptbr);

        let pte_val: u32;
        let pte_addr: AddrType = pte_base * PAGESIZE;
        let level: i32;

        // println! ("<Info: SATP=%016lx>\n", sptbr);

        for level in range(0, init_level).rev() {
            // for (level = init_level-1; level >= 0; level--) {
            // bool is_leaf_achieve = false;
            let va_vpn_i: AddrType = (vaddr >> vpn_idx[level]) & ((1 << vpn_len[level]) - 1);
            pte_addr += (va_vpn_i * PTESIZE);

            // if (m_bit_mode == RiscvBitMode_t::Bit64) {
            //     LoadMemoryDebug<u32> (pte_addr, &pte_val);
            // } else {
            // pte_addr &= 0x0FFFFFFFFULL;
            let pte_val = self.read_memory_word(pte_addr);
            // }

            println!(
                "<Info: VAddr = 0x{:016x} PTEAddr = 0x{:016x} : PPTE = 0x{:016x}>\n",
                vaddr, pte_addr, pte_val
            );

            // 3. If pte:v = 0, or if pte:r = 0 and pte:w = 1, stop and raise a page-fault exception.
            if ((pte_val & 0x01) == 0 || (((pte_val & 0x02) == 0) && ((pte_val & 0x04) == 0x04))) {
                {
                    // let bit_length: u32 = m_bit_mode == RiscvBitMode_t::Bit32 ? 8 : 16;
                    println!("<Page Table Error : 0x{:016x} = 0x{:08x} is not valid Page Table. Generate Exception>\n",
                             pte_addr, pte_val);
                }

                if (acc_type == MemAccType::FetchMemType) {
                    self.GenerateException(ExceptCode::Except_InstPageFault, vaddr);
                }
                return (MemResult::TlbError, 0);
            }

            // If pte:r = 1 or pte:x = 1, go to step 5. Otherwise, this PTE is a
            // pointer to the next level of the page table. Let i = i − 1. If i < 0, stop and raise a page-fault
            // exception. Otherwise, let a = pte:ppn × PAGESIZE and go to step 2.
            if (((pte_val & 0x08) == 0x08) || ((pte_val & 0x02) == 0x02)) {
                break;
            } else {
                if (level == 0) {
                    println!(
                        "<Access Fault : Tried to Access to Page {:01x}>\n",
                        ((pte_val >> 1) & 0x0f)
                    );

                    if (acc_type == MemAccType::FetchMemType) {
                        self.GenerateException(ExceptCode::Except_InstPageFault, vaddr);
                    }
                    return (MemResult::TlbError, 0);
                }
            }
            let pte_ppn: AddrType = Self::extract_bit_field(
                pte_val,
                pte_len[init_level - 1] + pte_idx[init_level - 1] - 1,
                pte_idx[0],
            );
            pte_addr = pte_ppn * PAGESIZE;
        }

        if (!IsAllowedAccess((pte_val >> 1) & 0x0f, acc_type, GetPrivMode())) {
            println!(
                "<Page Access Failed. Allowed Access Failed PTE_VAL={:016x}>\n",
                pte_val,
            );
            return (MemResult::TlbError, 0);
        }

        if (level != 0
            && Self::extract_bit_field(
                pte_val,
                pte_len[level - 1] + pte_idx[level - 1] - 1,
                pte_idx[0],
            ) != 0)
        {
            // 6. If i > 0 and pa:ppn[i−1:0] != 0, this is a misaligned superpage
            // stop and raise a page-fault exception.
            // println! ("<Page Access Failed. Last PTE != 0>\n");
            return (MemResult::TlbError, 0);
        }

        if (((pte_val & 0x40) == 0) || // PTE.A
            ((acc_type == MemAccType::WriteMemType) && (pte_val & 0x80) == 0))
        {
            // PTE.D
            println!(
                "<Access Fault : Page Permission Fault {:01x}\n",
                ((pte_val >> 1) & 0x0f)
            );
            if (acc_type == MemAccType::FetchMemType) {
                self.GenerateException(ExceptCode::Except_InstPageFault, vaddr);
            }
            return (MemResult::TlbError, 0);
        }

        phy_addr = Self::extract_bit_field(
            pte_val,
            pte_len[init_level - 1] + pte_idx[init_level - 1] - 1,
            pte_idx[level],
        ) << ppn_idx[level];
        for level in range(0, init_level).rev() {
            let vaddr_vpn: AddrType =
                Self::extract_bit_field(vaddr, vpn_len[level] + vpn_idx[level] - 1, vpn_idx[level]);
            phy_addr |= (vaddr_vpn << ppn_idx[level]);
        }

        // Finally Add Page Offset
        phy_addr |= Self::extract_bit_field(vaddr, vpn_idx[0] - 1, 0);

        *paddr = phy_addr;

        //==========================
        // Update Simple TLB Search
        //==========================
        // println!("<Info: TLB[{:d}] <= 0x{:016x}(0x{:016x})>\n", vaddr_tag, vaddr_vpn, *paddr & !0x0fff);
        // m_tlb_en  [vaddr_tag] = true;
        // m_tlb_tag [vaddr_tag] = vaddr_vpn;
        // m_tlb_addr[vaddr_tag] = (*paddr & !0x0fff) | (pte_val & 0x0ff);

        println!("Converted Virtual Address is = 0x{:016x}\n", paddr);

        return (MemResult::NoExcept, paddr);
    }

    fn GenerateException(&mut self, code: ExceptCode, tval: XlenType) {
        FlushTlb();

        if (code != ExceptCode::Except_IllegalInst && code != ExceptCode::Except_EcallFromSMode) {
            let paddr: AddrType = ConvertVirtualAddress(GetPC(), MemAccType::ReadMemType);
            // println!(
            //     "<Info: GenerateException Code={:d}, TVAL={:016x} PC={:016x},{:016x}>\n",
            //     code,
            //     tval,
            //     GetPC(),
            //     paddr
            // );
        }

        let epc: Addr_t;
        if (code == ExceptCode::Except_InstAddrMisalign) {
            epc = GetPreviousPC();
        } else {
            epc = GetPC();
        }

        curr_priv: PrivMode = GetPrivMode();

        let medeleg: XlenType;
        let mstatus: XlenType;
        let sstatus: XlenType;
        let tvec: AddrType;
        CSRRead(CsrAddr::MEDELEG, &medeleg);
        let next_priv: PrivMode = PrivMode::PrivMachine;
        SetPrivMode(next_priv);
        if (medeleg & (1 << code)) {
            // Delegation
            CSRWrite(CsrAddr::Sepc, epc);
            CSRWrite(CsrAddr::Scause, code);
            CSRWrite(CsrAddr::Stval, tval);

            CSRRead(CsrAddr::Stvec, &tvec);
            next_priv = PrivMode::PrivSupervisor;
        } else {
            CSRWrite(CsrAddr::Mepc, epc);
            CSRWrite(CsrAddr::Mcause, code);
            CSRWrite(CsrAddr::Mtval, tval);

            CSRRead(CsrAddr::Mtvec, &tvec);
        }

        // Update status CSR
        if (medeleg & (1 << code)) {
            // Delegation
            CSRRead(CsrAddr::SSTATUS, &sstatus);
            sstatus = SetBitField(
                sstatus,
                ExtractBitField(sstatus, SYSREG_SSTATUS_SIE_MSB, SYSREG_SSTATUS_SIE_LSB),
                SYSREG_SSTATUS_SPIE_MSB,
                SYSREG_SSTATUS_SPIE_LSB,
            );
            sstatus = SetBitField(
                sstatus,
                curr_priv,
                SYSREG_SSTATUS_SPP_MSB,
                SYSREG_SSTATUS_SPP_LSB,
            );
            sstatus = SetBitField(sstatus, 0, SYSREG_SSTATUS_SIE_MSB, SYSREG_SSTATUS_SIE_LSB);
            CSRWrite(CsrAddr::SSTATUS, sstatus);
        } else {
            CSRRead(CsrAddr::MSTATUS, &mstatus);

            mstatus = SetBitField(
                mstatus,
                ExtractBitField(mstatus, SYSREG_MSTATUS_MIE_MSB, SYSREG_MSTATUS_MIE_LSB),
                SYSREG_MSTATUS_MPIE_MSB,
                SYSREG_MSTATUS_MPIE_LSB,
            );
            mstatus = SetBitField(
                mstatus,
                curr_priv,
                SYSREG_MSTATUS_MPP_MSB,
                SYSREG_MSTATUS_MPP_LSB,
            );
            mstatus = SetBitField(mstatus, 0, SYSREG_MSTATUS_MIE_MSB, SYSREG_MSTATUS_MIE_LSB);

            CSRWrite(CsrAddr::MSTATUS, mstatus);
        }

        if (m_bit_mode == RiscvBitMode::Bit32) {
            tvec = tvec & 0xffffffff;
        }

        SetPrivMode(next_priv);

        SetPC(tvec);
        SetJumped(true);

        println!(
            "<Info: Exception. ChangeMode from {} to {}>\n",
            PrintPrivMode(curr_priv),
            PrintPrivMode(next_priv)
        );
        println!("<Info: Set Program Counter = 0x{:16x}>\n", tvec);

        // Relesae Load Reservation
        ClearLoadReservation();

        // // CountUp Timer
        // m_riscv_clint->Increment(INTERLEAVE / INSNS_PER_RTC_TICK);
        // m_count_timer = 0;

        return;
    }

    fn PrintPrivMode(priv_mode: PrivMode) -> String {
        match (priv_mode) {
            PrivMode::PrivUser => return "UserMode",
            PrivMode::PrivSupervisor => return "SuprevisorMode",
            PrivMode::PrivHypervisor => return "HypervisorMode",
            PrivMode::PrivMachine => return "MachineMode",
            _ => return "<Internal Error: PrivMode is illegal>\n",
        }
    }

    fn get_is_finish_cpu(&mut self) -> bool {
        return self.m_finish_cpu;
    }

    fn get_tohost(&mut self) -> XlenType {
        return self.m_tohost;
    }
    fn get_fromhost(&mut self) -> XlenType {
        return self.m_fromhost;
    }
}
