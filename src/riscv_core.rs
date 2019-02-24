use std::num::Wrapping;

use crate::riscv_csr::RiscvCsrBase;
use crate::riscv_csr::RiscvCsr;
use crate::riscv_csr::Riscv64Csr;
use crate::riscv_csr::CsrAddr;

pub type XlenType  = i32;
pub type UXlenType = u32;
pub type InstType  = u32;
pub type AddrType  = u32;
pub type RegAddrType = u8;

pub const DRAM_BASE: AddrType = 0x8000_0000;
pub const DRAM_SIZE: usize    = 0x01_0000;

pub enum RiscvInst {
    CSRRW , CSRRS , CSRRC ,
    CSRRWI, CSRRSI, CSRRCI,
    LUI, AUIPC,
    ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI,
    ADD, SUB, SLL ,SLT, SLTU, XOR, SRL, SRA, OR, AND,
    LB, LH, LW, LBU, LHU, SW,
    JAL, JALR,
    BEQ, BNE, BLT, BGE, BLTU, BGEU,
    FENCE, FENCEI,
    ECALL, EBREAK,
    MRET, SRET, URET,
    NOP,
    WFI
}


pub enum ExceptCode {
  InstAddrMisalign  =  0,
  InstAccessFault   =  1,
  IllegalInst       =  2,
  Breakpoint        =  3,
  LoadAddrMisalign  =  4,
  LoadAccessFault   =  5,
  StoreAddrMisalign =  6,
  StoreAccessFault  =  7,
  EcallFromUMode    =  8,
  EcallFromSMode    =  9,
  EcallFromHMode    = 10,
  EcallFromMMode    = 11,
  InstPageFault     = 12,
  LoadPageFault     = 13,
  StorePageFault    = 15,
}

pub enum MemType {
  LOAD = 0,
  STORE = 1
}


pub struct EnvBase
{
    pub m_pc: AddrType,
    pub m_regs:  [XlenType; 32],
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
            m_pc: DRAM_BASE as AddrType,
            m_memory: [0; DRAM_SIZE],
            m_regs: [0; 32],
            m_finish_cpu: false,
            m_fromhost_addr: (DRAM_BASE + 0x001000) as AddrType,
            m_tohost_addr:   (DRAM_BASE + 0x001000) as AddrType,
            m_fromhost: 0,
            m_tohost: 0,
            m_csr: RiscvCsr {
                m_mcycle    : RiscvCsrBase { m_csr: 0 },
                m_minstret  : RiscvCsrBase { m_csr: 0 },
                m_mimpid    : RiscvCsrBase { m_csr: 0 },
                m_marchid   : RiscvCsrBase { m_csr: 0 },
                m_mvendorid : RiscvCsrBase { m_csr: 0 },
                m_misa      : RiscvCsrBase { m_csr: 0 },
                m_mstatus   : RiscvCsrBase { m_csr: 0 },
                m_mtvec     : RiscvCsrBase { m_csr: 0 },
                m_mip       : RiscvCsrBase { m_csr: 0 },
                m_mie       : RiscvCsrBase { m_csr: 0 },
                m_mscratch  : RiscvCsrBase { m_csr: 0 },
                m_mepc      : RiscvCsrBase { m_csr: 0 },
                m_mtval     : RiscvCsrBase { m_csr: 0 },
                m_mcause    : RiscvCsrBase { m_csr: 0 },
                m_mhartid   : RiscvCsrBase { m_csr: 0 },
                m_dcsr      : RiscvCsrBase { m_csr: 0 },
                m_dpc       : RiscvCsrBase { m_csr: 0 },
                m_dscratch  : RiscvCsrBase { m_csr: 0 },
                m_medeleg   : RiscvCsrBase { m_csr: 0 },
            },
        }
    }

    fn extend_sign (data: XlenType, msb: XlenType) -> XlenType
    {
        let mask:XlenType = 1 << msb; // mask can be pre-computed if b is fixed
        let res_data = data & ((1 << (msb + 1)) - 1);  // (Skip this if bits in x above position b are already zero.)
        return (res_data ^ mask) - mask;
    }


    fn extract_bit_field (hex: InstType, left: XlenType, right: XlenType) -> XlenType
    {
        let mask: XlenType = (1 << (left - right + 1)) - 1;
        return ((hex >> right) & (mask as UXlenType)) as XlenType;
    }


    fn extract_uj_field (hex: InstType) -> XlenType
    {
        let i24_21 = Self::extract_bit_field (hex, 24, 21) & 0x0f;
        let i30_25 = Self::extract_bit_field (hex, 30, 25) & 0x03f;
        let i20_20 = Self::extract_bit_field (hex, 20, 20) & 0x01;
        let i19_12 = Self::extract_bit_field (hex, 19, 12) & 0x0ff;
        let i31_31 = Self::extract_bit_field (hex, 31, 31) & 0x01;

        let u_res: XlenType = (i31_31 << 20) |
        (i19_12 << 12) |
        (i20_20 << 11) |
        (i30_25 <<  5) |
        (i24_21 <<  1);
        return Self::extend_sign (u_res, 20);
    }


    fn extract_ifield (hex: InstType) -> XlenType
    {
        let uimm32:XlenType = Self::extract_bit_field (hex, 31, 20);
        return Self::extend_sign (uimm32, 11);
    }


    fn extract_shamt_field (hex: InstType) -> XlenType
    {
        return Self::extract_bit_field (hex, 24, 20);
    }


    fn extract_sb_field (hex: InstType) -> XlenType
    {
        let i07_07:XlenType = Self::extract_bit_field (hex,  7,  7) & 0x01;
        let i11_08:XlenType = Self::extract_bit_field (hex, 11,  8) & 0x0f;
        let i30_25:XlenType = Self::extract_bit_field (hex, 30, 25) & 0x03f;
        let i31_31:XlenType = Self::extract_bit_field (hex, 31, 31) & 0x01;

        let u_res:XlenType = (i31_31 << 12) |
        (i07_07 << 11) |
        (i30_25 <<  5) |
        (i11_08 <<  1);
        return Self::extend_sign (u_res, 12);
    }


    fn extract_sfield (hex: InstType) -> XlenType
    {
        let i11_07:XlenType = Self::extract_bit_field (hex, 11,  7) & 0x01f;
        let i31_25:XlenType = Self::extract_bit_field (hex, 31, 25) & 0x07f;

        let u_res:XlenType = (i31_25 << 5) |
        (i11_07 << 0);

        return Self::extend_sign (u_res, 11);
    }

    fn sext_xlen (hex: InstType) -> XlenType  { return hex as XlenType; }
    fn uext_xlen (hex: InstType) -> UXlenType { return hex as UXlenType; }
}


pub trait Riscv64Core {
    fn get_rs1_addr (inst:InstType) -> RegAddrType;
    fn get_rs2_addr (inst:InstType) -> RegAddrType;
    fn get_rd_addr  (inst:InstType) -> RegAddrType;

    fn fetch_memory(&mut self) -> XlenType;
    fn read_memory (&mut self, addr:AddrType) -> XlenType;
    fn write_memory(&mut self, addr:AddrType, data:XlenType) -> XlenType;
    fn write_memory_byte(&mut self, addr:AddrType, data:u8) -> XlenType;

    fn read_reg (&mut self, reg_addr: RegAddrType) -> i32;
    fn write_reg (&mut self, reg_addr: RegAddrType, data:XlenType);

    fn decode_inst(&mut self, inst:XlenType) -> RiscvInst;
    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstType);

    fn mem_access (&mut self, op: MemType, data: XlenType, addr: AddrType) -> XlenType;
}

impl Riscv64Core for EnvBase {
    fn get_rs1_addr (inst:InstType) -> RegAddrType {
        return ((inst >> 15) & 0x1f) as RegAddrType;
    }
    fn get_rs2_addr (inst:InstType) -> RegAddrType {
        return ((inst >> 20) & 0x1f) as RegAddrType;
    }
    fn get_rd_addr (inst:InstType) -> RegAddrType {
        return ((inst >> 7) & 0x1f) as RegAddrType;
    }

    fn read_reg (&mut self, reg_addr: RegAddrType) -> XlenType {
        if reg_addr == 0 {
            return 0;
        } else {
            return self.m_regs[reg_addr as usize];
        }
    }

    fn write_reg (&mut self, reg_addr: RegAddrType, data: XlenType) {
        if reg_addr != 0 {
            self.m_regs[reg_addr as usize] = data;
            println!("     x{:02} <= {:08x}", reg_addr, data);
        }
    }

    fn fetch_memory(&mut self) -> XlenType {
        let base_addr: AddrType = self.m_pc - DRAM_BASE;
        println!("BeseAddr = {:08x} {:08x} {:08x}", self.m_pc, DRAM_BASE, base_addr);
        let fetch_data = ((self.m_memory[base_addr as usize + 3] as XlenType) << 24) |
                         ((self.m_memory[base_addr as usize + 2] as XlenType) << 16) |
                         ((self.m_memory[base_addr as usize + 1] as XlenType) <<  8) |
                         ((self.m_memory[base_addr as usize + 0] as XlenType) <<  0);
        return fetch_data;
    }

    fn read_memory(&mut self, addr:AddrType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        return ((self.m_memory[base_addr as usize + 3] as XlenType) << 24) |
               ((self.m_memory[base_addr as usize + 2] as XlenType) << 16) |
               ((self.m_memory[base_addr as usize + 1] as XlenType) <<  8) |
               ((self.m_memory[base_addr as usize + 0] as XlenType) <<  0);
    }

    fn write_memory(&mut self, addr:AddrType, data:XlenType) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        self.m_memory[base_addr as usize + 0] = ((data >>  0) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 1] = ((data >>  8) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 2] = ((data >> 16) & 0x0ff) as u8;
        self.m_memory[base_addr as usize + 3] = ((data >> 24) & 0x0ff) as u8;

        return 0;
    }

    fn write_memory_byte (&mut self, addr:AddrType, data:u8) -> XlenType {
        let base_addr: AddrType = addr - DRAM_BASE;
        self.m_memory[base_addr as usize] = data;
        return 0;
    }

    fn decode_inst(&mut self, inst:XlenType) -> RiscvInst {
        let opcode = inst & 0x7f;
        let funct3 = (inst >> 12) & 0x07;
        let funct5 = (inst >> 25) & 0x7f;
        let imm12  = (inst >> 20) & 0xfff;

        let dec_inst: RiscvInst;

        match opcode {
            0x0f => {
                match funct3 {
                    0b000 => dec_inst = RiscvInst::FENCE,
                    0b001 => dec_inst = RiscvInst::FENCEI,
                    _     => dec_inst = RiscvInst::NOP,
                }
            }
            0x33 => {
                match funct3 {
                    0b000 => {
                        match funct5 {
                            0b0000000 => dec_inst = RiscvInst::ADD,
                            0b0100000 => dec_inst = RiscvInst::SUB,
                            _         => dec_inst = RiscvInst::NOP
                        }
                    }
                    0b001 => dec_inst = RiscvInst::SLL,
                    0b010 => dec_inst = RiscvInst::SLT,
                    0b011 => dec_inst = RiscvInst::SLTU,
                    0b100 => dec_inst = RiscvInst::XOR,
                    0b101 => {
                        match funct5 {
                            0b0000000 => dec_inst = RiscvInst::SRL,
                            0b0100000 => dec_inst = RiscvInst::SRA,
                            _         => dec_inst = RiscvInst::NOP
                        }
                    }
                    0b110 => dec_inst = RiscvInst::OR,
                    0b111 => dec_inst = RiscvInst::AND,
                    _     => dec_inst = RiscvInst::NOP,
                }
            }
            0x03 =>
                match funct3 {
                    0b000 => dec_inst = RiscvInst::LB,
                    0b001 => dec_inst = RiscvInst::LH,
                    0b010 => dec_inst = RiscvInst::LW,
                    0b100 => dec_inst = RiscvInst::LBU,
                    0b101 => dec_inst = RiscvInst::LHU,
                    _     => dec_inst = RiscvInst::NOP,
                }
            0x23 => dec_inst = RiscvInst::SW,
            0x37 => dec_inst = RiscvInst::LUI,
            0x17 => dec_inst = RiscvInst::AUIPC,
            0x63 => {
                match funct3 {
                    0b000 => dec_inst = RiscvInst::BEQ,
                    0b001 => dec_inst = RiscvInst::BNE,
                    0b100 => dec_inst = RiscvInst::BLT,
                    0b101 => dec_inst = RiscvInst::BGE,
                    0b110 => dec_inst = RiscvInst::BLTU,
                    0b111 => dec_inst = RiscvInst::BGEU,
                    _     => dec_inst = RiscvInst::NOP,
                }
            }
            0x13 => {
                match funct3 {
                    0b000 => dec_inst = RiscvInst::ADDI,
                    0b010 => dec_inst = RiscvInst::SLTI,
                    0b011 => dec_inst = RiscvInst::SLTIU,
                    0b100 => dec_inst = RiscvInst::XORI,
                    0b110 => dec_inst = RiscvInst::ORI,
                    0b111 => dec_inst = RiscvInst::ANDI,
                    0b001 => dec_inst = RiscvInst::SLLI,
                    0b101 => {
                        match funct5 {
                            0b0000000 => dec_inst = RiscvInst::SRLI,
                            0b0100000 => dec_inst = RiscvInst::SRAI,
                            _         => dec_inst = RiscvInst::NOP
                        }
                    }
                    _     => dec_inst = RiscvInst::NOP,
                }
            }
            0x6f => dec_inst = RiscvInst::JAL,
            0x67 => dec_inst = RiscvInst::JALR,
            0x73 => {
                match funct3 {
                    0x000 => {
                        match imm12 {
                            0x000 => dec_inst = RiscvInst::ECALL,
                            0x001 => dec_inst = RiscvInst::EBREAK,
                            0x002 => dec_inst = RiscvInst::URET,
                            0x102 => dec_inst = RiscvInst::SRET,
                            0x302 => dec_inst = RiscvInst::MRET,
                            _     => dec_inst = RiscvInst::NOP,
                        }
                    }
                    0b001 => dec_inst = RiscvInst::CSRRW  ,
                    0b010 => dec_inst = RiscvInst::CSRRS  ,
                    0b011 => dec_inst = RiscvInst::CSRRC  ,
                    0b101 => dec_inst = RiscvInst::CSRRWI ,
                    0b110 => dec_inst = RiscvInst::CSRRSI ,
                    0b111 => dec_inst = RiscvInst::CSRRCI ,
                    _     => dec_inst = RiscvInst::NOP    ,
                }
            }
            _    => dec_inst = RiscvInst::WFI,
        }

        return dec_inst;
    }

    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstType)
    {
        println!("{:08x} : {:08x} // DASM({:08x})", self.m_pc as u32, inst as u32, inst as u32);

        let rs1 = Self::get_rs1_addr (inst);
        let rs2 = Self::get_rs2_addr (inst);
        let rd  = Self::get_rd_addr  (inst);

        let csr_addr = CsrAddr::from_u64(((inst >> 20) & 0x0fff) as u64);

        let mut update_pc = false;

        match dec_inst {
            RiscvInst::CSRRW  => {
                let rs1_data = self.read_reg(rs1);
                let reg_data:XlenType = self.m_csr.csrrw (csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRS  => {
                let rs1_data = self.read_reg(rs1);
                let reg_data:XlenType = self.m_csr.csrrs (csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRC  => {
                let rs1_data = self.read_reg(rs1);
                let reg_data:XlenType = self.m_csr.csrrc (csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRWI => {
                let zimm: XlenType = ((inst >> 15) & 0x1f) as XlenType;
                let reg_data:XlenType = self.m_csr.csrrw (csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRSI => {
                let zimm: XlenType = ((inst >> 15) & 0x1f) as XlenType;
                let reg_data:XlenType = self.m_csr.csrrs (csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRCI => {
                let zimm: XlenType = ((inst >> 15) & 0x1f) as XlenType;
                let reg_data:XlenType = self.m_csr.csrrc (csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LUI => {
                let mut imm: XlenType = Self::extend_sign (Self::extract_bit_field (inst, 31, 12), 19);
                imm = imm << 12;
                self.write_reg(rd, imm);
            }
            RiscvInst::AUIPC => {
                let mut imm: XlenType = Self::extend_sign (Self::extract_bit_field (inst, 31, 12), 19);
                imm = (Wrapping(imm << 12) + Wrapping(self.m_pc as XlenType)).0;
                self.write_reg(rd, imm);
            }
            RiscvInst::LB  => {
                let rs1_data = self.read_reg(rs1);
                let imm: AddrType = ((inst >> 20) & 0xfff) as AddrType;
                let addr: AddrType = (self.read_reg(rs1) as AddrType) + imm;
                let mut reg_data:XlenType = self.mem_access(MemType::LOAD, rs1_data, addr);
                reg_data = reg_data >> (8 * (addr & 0x03));
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LH  => {
                let rs1_data = self.read_reg(rs1);
                let imm: AddrType = ((inst >> 20) & 0xfff) as AddrType;
                let addr: AddrType = (self.read_reg(rs1) as AddrType) + imm;
                let mut reg_data:XlenType = self.mem_access(MemType::LOAD, rs1_data, addr);
                reg_data = reg_data >> (8 * (addr & 0x02)) ;
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LW  => {
                let rs1_data = self.read_reg(rs1);
                let imm: AddrType = ((inst >> 20) & 0xfff) as AddrType;
                let addr: AddrType = (self.read_reg(rs1) as AddrType) + imm;
                let reg_data:XlenType = self.mem_access(MemType::LOAD, rs1_data, addr);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LBU  => {
                let rs1_data = self.read_reg(rs1);
                let imm: AddrType = ((inst >> 20) & 0xfff) as AddrType;
                let addr: AddrType = (self.read_reg(rs1) as AddrType) + imm;
                let mut reg_data:UXlenType = self.mem_access(MemType::LOAD, rs1_data, addr) as UXlenType;
                reg_data = reg_data >> (8 * (addr & 0x03));
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::LHU  => {
                let rs1_data = self.read_reg(rs1);
                let imm: AddrType = ((inst >> 20) & 0xfff) as AddrType;
                let addr: AddrType = (self.read_reg(rs1) as AddrType) + imm;
                let mut reg_data:UXlenType = self.mem_access(MemType::LOAD, rs1_data, addr) as UXlenType;
                reg_data = reg_data >> (8 * (addr & 0x02)) ;
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::ADDI => {
                let rs1_data = self.read_reg(rs1);
                let imm_data = Self::extract_ifield (inst);
                let reg_data:XlenType = (Wrapping(rs1_data) + Wrapping(imm_data)).0;
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLTI => {
                let reg_data:bool = self.read_reg(rs1) < Self::extract_ifield (inst);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SLTIU => {
                let reg_data:bool = (self.read_reg(rs1) as UXlenType) < (Self::extract_ifield (inst) as UXlenType);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::XORI => {
                let reg_data:XlenType = self.read_reg(rs1) ^ Self::extract_ifield (inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::ORI => {
                let reg_data:XlenType = self.read_reg(rs1) | Self::extract_ifield (inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::ANDI => {
                let reg_data:XlenType = self.read_reg(rs1) & Self::extract_ifield (inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLLI => {
                let reg_data:XlenType = self.read_reg(rs1) << Self::extract_shamt_field (inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SRLI => {
                let reg_data:UXlenType = (self.read_reg(rs1) as UXlenType) >> Self::extract_shamt_field (inst);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SRAI => {
                let reg_data:XlenType = self.read_reg(rs1) >> Self::extract_shamt_field (inst);
                self.write_reg(rd, reg_data);
            }

            RiscvInst::ADD => {
                let reg_data:XlenType = self.read_reg(rs1) + self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SUB => {
                let reg_data:XlenType = self.read_reg(rs1) - self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLL => {
                let reg_data:XlenType = self.read_reg(rs1) << (self.read_reg(rs2) as UXlenType);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLT => {
                let reg_data:bool = self.read_reg(rs1) < self.read_reg(rs2);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SLTU => {
                let reg_data:bool = (self.read_reg(rs1) as UXlenType) < (self.read_reg(rs2) as UXlenType);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::XOR => {
                let reg_data:XlenType = self.read_reg(rs1) ^ self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SRL => {
                let reg_data:UXlenType = (self.read_reg(rs1) as UXlenType) >> (self.read_reg(rs2) as UXlenType);
                self.write_reg(rd, reg_data as XlenType);
            }
            RiscvInst::SRA => {
                let reg_data:XlenType = self.read_reg(rs1) >> (self.read_reg(rs2) as UXlenType);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::OR => {
                let reg_data:XlenType = self.read_reg(rs1) | self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::AND => {
                let reg_data:XlenType = self.read_reg(rs1) & self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SW  => {
                let rs2_data = self.read_reg(rs2);
                let addr:AddrType = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrType;
                self.mem_access(MemType::STORE, rs2_data, addr);
            }
            RiscvInst::JAL => {
                let addr:AddrType = Self::extract_uj_field(inst) as AddrType;
                self.m_pc = (Wrapping(self.m_pc) + Wrapping(addr)).0;
                self.write_reg(rd, (addr + 4) as XlenType);
                update_pc = true;
            }
            RiscvInst::BEQ | RiscvInst::BNE | RiscvInst::BLT | RiscvInst::BGE | RiscvInst::BLTU | RiscvInst::BGEU => {
                let rs1_data:XlenType = self.read_reg(rs1);
                let rs2_data:XlenType = self.read_reg(rs2);
                let addr:AddrType = Self::extract_sb_field(inst) as AddrType;
                let jump_en: bool;
                match dec_inst {
                    RiscvInst::BEQ  => jump_en = rs1_data == rs2_data,
                    RiscvInst::BNE  => jump_en = rs1_data != rs2_data,
                    RiscvInst::BLT  => jump_en = rs1_data <  rs2_data,
                    RiscvInst::BGE  => jump_en = rs1_data >= rs2_data,
                    RiscvInst::BLTU => jump_en = (rs1_data as UXlenType) >= (rs2_data as UXlenType),
                    RiscvInst::BGEU => jump_en = (rs1_data as UXlenType) >= (rs2_data as UXlenType),
                    _               => panic!("Unknown value Branch"),
                }
                if jump_en {
                    self.m_pc = self.m_pc + addr;
                    update_pc = true;
                }
            }
            RiscvInst::JALR => {
                let addr: AddrType = Self::extract_ifield (inst) as AddrType;
                self.m_pc = addr;
                self.write_reg(rd, (addr + 4) as XlenType);
                update_pc = true;
            }
            RiscvInst::FENCE   => {  }
            RiscvInst::FENCEI => {  }
            RiscvInst::ECALL => {
                let mtvec:XlenType  = self.m_csr.csrrs (CsrAddr::Mtvec, 0); // MTVEC

                self.m_csr.csrrw (CsrAddr::Mepc, self.m_pc as XlenType);                           // MEPC
                self.m_csr.csrrw (CsrAddr::Mcause, ExceptCode::EcallFromMMode as XlenType); // MCAUSE
                self.m_csr.csrrw (CsrAddr::Mtval, 0);                             // MTVAL

                self.m_pc = mtvec as AddrType;
                update_pc = true;
            }
            RiscvInst::EBREAK => {  }
            RiscvInst::URET => {
            }
            RiscvInst::SRET => {
            }
            RiscvInst::MRET => {
                let mepc: XlenType = self.m_csr.csrrs (CsrAddr::Mepc, 0); // MEPC
                self.m_pc = mepc as AddrType;
                update_pc = true;
            }
            _  => { }
        }

        if update_pc == false {
            self.m_pc += 4;
        }
    }

    fn mem_access (&mut self, op: MemType, data: XlenType, addr: AddrType) -> XlenType
    {
        match op {
            MemType::STORE => {
                if addr == self.m_tohost_addr {
                    self.m_finish_cpu = true;
                    self.m_tohost = data;
                } else if addr == self.m_fromhost_addr {
                    self.m_finish_cpu = true;
                    self.m_fromhost = data;
                } else {
                    self.write_memory(addr, data);
                }

            }
            MemType::LOAD  => {
                if addr == self.m_tohost_addr {
                    return self.m_tohost;
                } else if addr == self.m_fromhost_addr {
                    return self.m_fromhost;
                } else {
                    return self.read_memory(addr);
                }
            }
        }
        return 0;
    }


}
