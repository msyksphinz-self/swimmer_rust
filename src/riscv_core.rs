extern crate num;

use crate::riscv_csr::CsrAddr;
use crate::riscv_csr::RiscvCsr;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_MSB;

use crate::riscv_csr_bitdef::SYSREG_SATP_MODE_LSB;
use crate::riscv_csr_bitdef::SYSREG_SATP_MODE_MSB;

use crate::riscv_tracer::RiscvTracer;
use crate::riscv_tracer::TraceInfo;
use crate::riscv_tracer::TraceType;
use crate::riscv_tracer::Tracer;

use crate::riscv_exception::ExceptCode;
use crate::riscv_exception::RiscvException;

use crate::riscv_mmu::RiscvMmu;

pub type InstType = u32;
pub type AddrType = u32;
pub type RegAddrType = u8;

pub const DRAM_BASE: AddrType = 0x8000_0000;
pub const DRAM_SIZE: usize = 0x10_0000;

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
pub enum MemResult {
    NoExcept = 0,
    MisAlign = 1 << 0,
    NotDefined = 1 << 1,
    NewRegion = 1 << 2,
    TlbError = 1 << 3,
}

#[derive(PartialEq, Eq)]
pub enum VMMode {
    Mbare = 0,
    Sv32 = 1,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

pub enum MemType {
    LOAD,
    STORE,
}

pub enum MemSize {
    BYTE,
    HWORD,
    WORD,
    DWORD,
}

pub struct EnvBase<XlenT> {
    // m_bitmode: RiscvBitMode,
    pub m_pc: AddrType,
    m_previous_pc: AddrType,
    m_regs: [XlenT; 32],
    pub m_memory: [u8; DRAM_SIZE], // memory
    pub m_csr: RiscvCsr<XlenT>,

    pub m_priv: PrivMode,
    m_maxpriv: PrivMode,
    m_vmmode: VMMode,

    m_trace: Tracer<XlenT>,

    m_tohost_addr: AddrType,
    m_fromhost_addr: AddrType,
    m_tohost: XlenT,
    m_fromhost: XlenT,

    m_finish_cpu: bool,

    m_is_update_pc: bool,
}

impl<XlenT> EnvBase<XlenT>
where
    XlenT: Copy + Clone,
{
    // pub fn new() -> EnvBase<XlenT> {
    //     EnvBase {
    //         // m_bitmode: RiscvBitMode::Bit32,
    //         m_pc: DRAM_BASE as AddrType,
    //         m_memory: [0; DRAM_SIZE],
    //         m_regs: [0 as XlenT; 32],
    //         m_maxpriv: PrivMode::Machine,
    //         m_previous_pc: 0 as AddrType,
    //         m_vmmode: VMMode::Mbare,
    //         m_finish_cpu: false,
    //         m_fromhost_addr: (DRAM_BASE + 0x001000) as AddrType,
    //         m_tohost_addr: (DRAM_BASE + 0x001000) as AddrType,
    //         m_fromhost: 0 as XlenT,
    //         m_tohost: 0 as XlenT,
    //         m_is_update_pc: false,
    //         m_csr: RiscvCsr::<XlenT>::new(),
    //         m_priv: PrivMode::Machine,
    //         m_trace: Tracer::new(),
    //     }
    // }

    fn extend_sign(data: XlenT, msb: XlenT) -> XlenT {
        let mask: XlenT = XlenT::one << msb; // mask can be pre-computed if b is fixed
        let res_data = data & ((XlenT::one << (msb + 1)) - 1); // (Skip this if bits in x above position b are already zero.)
        return (res_data ^ mask) - mask;
    }

    pub fn extract_bit_field(hex: XlenT, left: u8, right: u8) -> XlenT {
        let mask: XlenT = (1 << (left - right + 1)) - 1;
        return (hex >> right) & mask;
    }

    pub fn set_bit_field(hex: XlenT, val: XlenT, left: u8, right: u8) -> XlenT {
        let mask: XlenT = (1 << (left - right + 1)) - 1;
        return (hex & !(mask << right)) | (val << right);
    }

    fn extract_uj_field(hex: InstType) -> XlenT {
        let i24_21 = Self::extract_bit_field(hex as XlenT, 24, 21) & 0x0f;
        let i30_25 = Self::extract_bit_field(hex as XlenT, 30, 25) & 0x03f;
        let i20_20 = Self::extract_bit_field(hex as XlenT, 20, 20) & 0x01;
        let i19_12 = Self::extract_bit_field(hex as XlenT, 19, 12) & 0x0ff;
        let i31_31 = Self::extract_bit_field(hex as XlenT, 31, 31) & 0x01;

        let u_res: XlenT =
            (i31_31 << 20) | (i19_12 << 12) | (i20_20 << 11) | (i30_25 << 5) | (i24_21 << 1);
        return Self::extend_sign(u_res, 20);
    }

    fn extract_ifield(hex: InstType) -> XlenT {
        let uimm32: XlenT = Self::extract_bit_field(hex as XlenT, 31, 20);
        return Self::extend_sign(uimm32, 11);
    }

    fn extract_shamt_field(hex: InstType) -> XlenT {
        return Self::extract_bit_field(hex as XlenT, 24, 20);
    }

    fn extract_sb_field(hex: InstType) -> XlenT {
        let i07_07: XlenT = Self::extract_bit_field(hex as XlenT, 7, 7) & 0x01;
        let i11_08: XlenT = Self::extract_bit_field(hex as XlenT, 11, 8) & 0x0f;
        let i30_25: XlenT = Self::extract_bit_field(hex as XlenT, 30, 25) & 0x03f;
        let i31_31: XlenT = Self::extract_bit_field(hex as XlenT, 31, 31) & 0x01;

        let u_res: XlenT = (i31_31 << 12) | (i07_07 << 11) | (i30_25 << 5) | (i11_08 << 1);
        return Self::extend_sign(u_res, 12);
    }

    fn extract_sfield(hex: InstType) -> XlenT {
        let i11_07: XlenT = Self::extract_bit_field(hex as XlenT, 11, 7) & 0x01f;
        let i31_25: XlenT = Self::extract_bit_field(hex as XlenT, 31, 25) & 0x07f;

        let u_res: XlenT = (i31_25 << 5) | (i11_07 << 0);

        return Self::extend_sign(u_res, 11);
    }

    fn sext_xlen(hex: InstType) -> XlenT {
        return hex as XlenT;
    }

    fn is_update_pc(&mut self) -> bool {
        return self.m_is_update_pc;
    }
    pub fn set_update_pc(&mut self, update_pc: bool) {
        self.m_is_update_pc = update_pc;
    }
}

pub trait Riscv64Core<XlenT, UXlenT> {
    fn get_rs1_addr(inst: InstType) -> RegAddrType;
    fn get_rs2_addr(inst: InstType) -> RegAddrType;
    fn get_rd_addr(inst: InstType) -> RegAddrType;

    fn set_pc(&mut self, addr: AddrType);
    fn get_pc(&mut self) -> AddrType;
    fn get_previous_pc(&mut self) -> AddrType;

    fn read_memory_word(&mut self, phy_addr: AddrType) -> XlenT;
    fn read_memory_hword(&mut self, phy_addr: AddrType) -> XlenT;
    fn read_memory_byte(&mut self, phy_addr: AddrType) -> XlenT;
    fn write_memory_word(&mut self, phy_addr: AddrType, data: XlenT) -> XlenT;
    fn write_memory_hword(&mut self, phy_addr: AddrType, data: XlenT) -> XlenT;
    fn write_memory_byte(&mut self, phy_addr: AddrType, data: XlenT) -> XlenT;

    fn fetch_bus(&mut self) -> (MemResult, InstType);
    fn read_bus_word(&mut self, addr: AddrType) -> (MemResult, XlenT);
    fn read_bus_hword(&mut self, addr: AddrType) -> (MemResult, XlenT);
    fn read_bus_byte(&mut self, addr: AddrType) -> (MemResult, XlenT);
    fn write_bus_word(&mut self, addr: AddrType, data: XlenT) -> MemResult;
    fn write_bus_hword(&mut self, addr: AddrType, data: XlenT) -> MemResult;
    fn write_bus_byte(&mut self, addr: AddrType, data: XlenT) -> MemResult;

    fn read_reg(&mut self, reg_addr: RegAddrType) -> XlenT;
    fn write_reg(&mut self, reg_addr: RegAddrType, data: XlenT);

    fn decode_inst(&mut self, inst: InstType) -> RiscvInst;
    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstType, step: u32);

    // fn print_priv_mode(priv_mode: PrivMode) -> &str;

    // fn get_priv_mode(&mut self) -> PrivMode;
    fn set_priv_mode(&mut self, priv_mode: PrivMode);

    // fn get_max_priv(&mut self) -> PrivMode;
    fn set_max_priv(&mut self, maxpriv: PrivMode);

    fn get_vm_mode(&mut self) -> VMMode;
    fn set_vm_mode(&mut self, vmmode: VMMode);

    fn get_is_finish_cpu(&mut self) -> bool;

    fn get_tohost(&mut self) -> XlenT;
    fn get_fromhost(&mut self) -> XlenT;
}

impl<XlenT, UXlenT> Riscv64Core<XlenT, UXlenT> for EnvBase<XlenT> {
    fn get_rs1_addr(inst: InstType) -> RegAddrType {
        return ((inst >> 15) & 0x1f) as RegAddrType;
    }
    fn get_rs2_addr(inst: InstType) -> RegAddrType {
        return ((inst >> 20) & 0x1f) as RegAddrType;
    }
    fn get_rd_addr(inst: InstType) -> RegAddrType {
        return ((inst >> 7) & 0x1f) as RegAddrType;
    }

    fn read_reg(&mut self, reg_addr: RegAddrType) -> XlenT {
        let ret_val: XlenT;

        if reg_addr == 0 {
            ret_val = 0;
        } else {
            ret_val = self.m_regs[reg_addr as usize];
        }

        let mut read_reg_trace = TraceInfo::new();
        read_reg_trace.m_trace_type = TraceType::XRegRead;
        read_reg_trace.m_trace_addr = reg_addr as AddrType;
        read_reg_trace.m_trace_value = ret_val;
        read_reg_trace.m_trace_memresult = MemResult::NoExcept;

        self.m_trace.m_trace_info.push(read_reg_trace);

        return ret_val;
    }

    fn write_reg(&mut self, reg_addr: RegAddrType, data: XlenT) {
        if reg_addr != 0 {
            let mut write_reg_trace = TraceInfo::new();

            write_reg_trace.m_trace_type = TraceType::XRegWrite;
            write_reg_trace.m_trace_addr = reg_addr as AddrType;
            write_reg_trace.m_trace_value = data;
            write_reg_trace.m_trace_memresult = MemResult::NoExcept;

            self.m_trace.m_trace_info.push(write_reg_trace);

            self.m_regs[reg_addr as usize] = data;
            // println!("     x{:02} <= {:08x}", reg_addr, data);
        }
    }

    fn set_pc(&mut self, addr: AddrType) {
        self.m_previous_pc = self.m_pc;
        self.m_pc = addr;
    }

    fn get_pc(&mut self) -> AddrType {
        return self.m_pc;
    }

    fn get_previous_pc(&mut self) -> AddrType {
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

    fn read_memory_word(&mut self, phy_addr: AddrType) -> XlenT {
        if phy_addr == self.m_tohost_addr {
            return self.m_tohost;
        } else if phy_addr == self.m_fromhost_addr {
            return self.m_fromhost;
        } else {
            return (self.read_memory_byte(phy_addr + 3) << 24)
                | (self.read_memory_byte(phy_addr + 2) << 16)
                | (self.read_memory_byte(phy_addr + 1) << 8)
                | (self.read_memory_byte(phy_addr + 0) << 0);
        }
    }

    fn read_memory_hword(&mut self, phy_addr: AddrType) -> XlenT {
        return (self.read_memory_byte(phy_addr + 1) << 8)
            | (self.read_memory_byte(phy_addr + 0) << 0);
    }

    fn read_memory_byte(&mut self, phy_addr: AddrType) -> XlenT {
        assert!(phy_addr >= DRAM_BASE);
        let base_addr: AddrType = phy_addr - DRAM_BASE;

        return self.m_memory[base_addr as usize + 0] as XlenT;
    }

    fn write_memory_word(&mut self, phy_addr: AddrType, data: XlenT) -> XlenT {
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

    fn write_memory_hword(&mut self, phy_addr: AddrType, data: XlenT) -> XlenT {
        self.write_memory_byte(phy_addr + 0, (data >> 0) & 0xff);
        self.write_memory_byte(phy_addr + 1, (data >> 8) & 0xff);

        return 0;
    }

    fn write_memory_byte(&mut self, phy_addr: AddrType, data: XlenT) -> XlenT {
        assert!(phy_addr >= DRAM_BASE);
        let base_addr: AddrType = phy_addr - DRAM_BASE;

        self.m_memory[base_addr as usize] = (data & 0xff) as u8;
        return 0;
    }

    fn fetch_bus(&mut self) -> (MemResult, InstType) {
        // let result: MemResult;
        // let phy_addr: AddrType;
        let (result, phy_addr) = self.convert_virtual_address(self.m_pc, MemAccType::Fetch);

        if result != MemResult::NoExcept {
            return (result, 0);
        }
        return (result, self.read_memory_word(phy_addr) as InstType);
    }

    fn read_bus_word(&mut self, addr: AddrType) -> (MemResult, XlenT) {
        let (result, phy_addr) = self.convert_virtual_address(addr, MemAccType::Read);

        if result != MemResult::NoExcept {
            return (result, 0);
        }

        let ret_val = self.read_memory_word(phy_addr);

        let mut read_mem_trace = TraceInfo::new();

        read_mem_trace.m_trace_type = TraceType::MemRead;
        read_mem_trace.m_trace_addr = addr;
        read_mem_trace.m_trace_value = ret_val;
        read_mem_trace.m_trace_memresult = MemResult::NoExcept;

        self.m_trace.m_trace_info.push(read_mem_trace);

        return (result, ret_val);
    }

    fn read_bus_hword(&mut self, addr: AddrType) -> (MemResult, XlenT) {
        let (result, phy_addr) = self.convert_virtual_address(addr, MemAccType::Read);

        if result != MemResult::NoExcept {
            return (result, 0);
        }

        return (result, self.read_memory_hword(phy_addr));
    }

    fn read_bus_byte(&mut self, addr: AddrType) -> (MemResult, XlenT) {
        let (result, phy_addr) = self.convert_virtual_address(addr, MemAccType::Read);

        if result != MemResult::NoExcept {
            return (result, 0);
        }
        return (result, self.read_memory_byte(phy_addr));
    }

    fn write_bus_word(&mut self, addr: AddrType, data: XlenT) -> MemResult {
        // let result: MemResult;
        // let phy_addr: AddrType;
        let (result, phy_addr) = self.convert_virtual_address(addr, MemAccType::Write);

        if result != MemResult::NoExcept {
            return result;
        }

        let mut write_mem_trace = TraceInfo::new();

        write_mem_trace.m_trace_type = TraceType::MemWrite;
        write_mem_trace.m_trace_addr = addr;
        write_mem_trace.m_trace_value = data;
        write_mem_trace.m_trace_memresult = MemResult::NoExcept;

        self.m_trace.m_trace_info.push(write_mem_trace);

        self.write_memory_word(phy_addr, data);

        return result;
    }

    fn write_bus_hword(&mut self, addr: AddrType, data: XlenT) -> MemResult {
        let (result, phy_addr) = self.convert_virtual_address(addr, MemAccType::Write);

        if result != MemResult::NoExcept {
            return result;
        }

        self.write_memory_hword(phy_addr, data);

        return result;
    }

    fn write_bus_byte(&mut self, addr: AddrType, data: XlenT) -> MemResult {
        let (result, phy_addr) = self.convert_virtual_address(addr, MemAccType::Write);

        if result != MemResult::NoExcept {
            return result;
        }

        self.write_memory_byte(phy_addr, data);

        return result;
    }

    fn decode_inst(&mut self, inst: InstType) -> RiscvInst {
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

    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstType, step: u32) {
        self.m_trace.m_executed_pc = self.m_pc;
        self.m_trace.m_inst_hex = inst;
        self.m_trace.m_step = step;

        self.m_trace.m_priv = self.m_priv;
        self.m_trace.m_vmmode = self.get_vm_mode();

        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let csr_addr = CsrAddr::from_u64(((inst >> 20) & 0x0fff) as u64);

        self.set_update_pc(false);

        match dec_inst {
            RiscvInst::CSRRW => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenT = self.m_csr.csrrw(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRS => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenT = self.m_csr.csrrs(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRC => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenT = self.m_csr.csrrc(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRWI => {
                let zimm: XlenT = ((inst >> 15) & 0x1f) as XlenT;
                let reg_data: XlenT = self.m_csr.csrrw(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRSI => {
                let zimm: XlenT = ((inst >> 15) & 0x1f) as XlenT;
                let reg_data: XlenT = self.m_csr.csrrs(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::CSRRCI => {
                let zimm: XlenT = ((inst >> 15) & 0x1f) as XlenT;
                let reg_data: XlenT = self.m_csr.csrrc(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::LUI => {
                let mut imm: XlenT =
                    Self::extend_sign(Self::extract_bit_field(inst as XlenT, 31, 12), 19);
                imm = imm << 12;
                self.write_reg(rd, imm);
            }
            RiscvInst::AUIPC => {
                let mut imm: XlenT =
                    Self::extend_sign(Self::extract_bit_field(inst as XlenT, 31, 12), 19);
                imm = (imm << 12).wrapping_add(self.m_pc as XlenT);
                self.write_reg(rd, imm);
            }
            RiscvInst::LB => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as AddrType);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 7);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInst::LH => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as AddrType);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 15);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInst::LW => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_word(addr as AddrType);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data);
                }
            }
            RiscvInst::LBU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as AddrType);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as XlenT);
                }
            }
            RiscvInst::LHU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as AddrType);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as XlenT);
                }
            }
            RiscvInst::ADDI => {
                let rs1_data = self.read_reg(rs1);
                let imm_data = Self::extract_ifield(inst);
                let reg_data: XlenT = rs1_data.wrapping_add(imm_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLTI => {
                let reg_data: bool = self.read_reg(rs1) < Self::extract_ifield(inst);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::SLTIU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlenT) < (Self::extract_ifield(inst) as UXlenT);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::XORI => {
                let reg_data: XlenT = self.read_reg(rs1) ^ Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::ORI => {
                let reg_data: XlenT = self.read_reg(rs1) | Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::ANDI => {
                let reg_data: XlenT = self.read_reg(rs1) & Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLLI => {
                let reg_data: XlenT = self.read_reg(rs1) << Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SRLI => {
                let reg_data: UXlenT =
                    (self.read_reg(rs1) as UXlenT) >> Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::SRAI => {
                let reg_data: XlenT = self.read_reg(rs1) >> Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data);
            }

            RiscvInst::ADD => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT = rs1_data.wrapping_add(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SUB => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT = rs1_data.wrapping_sub(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: XlenT = rs1_data.wrapping_shl(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SLT => {
                let reg_data: bool = self.read_reg(rs1) < self.read_reg(rs2);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::SLTU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlenT) < (self.read_reg(rs2) as UXlenT);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::XOR => {
                let reg_data: XlenT = self.read_reg(rs1) ^ self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SRL => {
                let rs1_data = self.read_reg(rs1) as UXlenT;
                let rs2_data = self.read_reg(rs2);
                let reg_data = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::SRA => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: XlenT = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data);
            }

            RiscvInst::MUL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT = rs1_data.wrapping_mul(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::MULH => {
                let rs1_data: i64 = self.read_reg(rs1) as i64;
                let rs2_data: i64 = self.read_reg(rs2) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0x0ffffffff;
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::MULHSU => {
                let rs1_data: i64 = (self.read_reg(rs1) as i32) as i64;
                let rs2_data: i64 = (self.read_reg(rs2) as u32) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInst::MULHU => {
                let rs1_data: u64 = (self.read_reg(rs1) as u32) as u64;
                let rs2_data: u64 = (self.read_reg(rs2) as u32) as u64;
                let mut reg_data: u64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as XlenT);
            }

            RiscvInst::REM => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT;
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
                let rs1_data: UXlenT = self.read_reg(rs1) as UXlenT;
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: UXlenT;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, reg_data as XlenT);
            }

            RiscvInst::DIV => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT;
                if rs2_data == 0 {
                    reg_data = -1;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data);
            }
            RiscvInst::DIVU => {
                let rs1_data: UXlenT = self.read_reg(rs1) as UXlenT;
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: UXlenT;
                if rs2_data == 0 {
                    reg_data = 0xffffffff;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data as XlenT);
            }

            RiscvInst::OR => {
                let reg_data: XlenT = self.read_reg(rs1) | self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::AND => {
                let reg_data: XlenT = self.read_reg(rs1) & self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInst::SB => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrType = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrType;
                self.write_bus_byte(addr, rs2_data);
            }
            RiscvInst::SH => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrType = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrType;
                self.write_bus_hword(addr, rs2_data);
            }
            RiscvInst::SW => {
                let rs2_data = self.read_reg(rs2);
                let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
                self.write_bus_word(addr as AddrType, rs2_data);
            }
            RiscvInst::JAL => {
                let addr: AddrType = Self::extract_uj_field(inst) as AddrType;
                self.write_reg(rd, (self.m_pc + 4) as XlenT);
                self.m_pc = self.m_pc.wrapping_add(addr);
                self.set_update_pc(true);
            }
            RiscvInst::BEQ
            | RiscvInst::BNE
            | RiscvInst::BLT
            | RiscvInst::BGE
            | RiscvInst::BLTU
            | RiscvInst::BGEU => {
                let rs1_data: XlenT = self.read_reg(rs1);
                let rs2_data: XlenT = self.read_reg(rs2);
                let addr: AddrType = Self::extract_sb_field(inst) as AddrType;
                let jump_en: bool;
                match dec_inst {
                    RiscvInst::BEQ => jump_en = rs1_data == rs2_data,
                    RiscvInst::BNE => jump_en = rs1_data != rs2_data,
                    RiscvInst::BLT => jump_en = rs1_data < rs2_data,
                    RiscvInst::BGE => jump_en = rs1_data >= rs2_data,
                    RiscvInst::BLTU => jump_en = (rs1_data as UXlenT) < (rs2_data as UXlenT),
                    RiscvInst::BGEU => jump_en = (rs1_data as UXlenT) >= (rs2_data as UXlenT),
                    _ => panic!("Unknown value Branch"),
                }
                if jump_en {
                    self.m_pc = self.m_pc.wrapping_add(addr);
                    self.set_update_pc(true);
                }
            }
            RiscvInst::JALR => {
                let mut addr: AddrType = Self::extract_ifield(inst) as AddrType;
                let rs1_data: AddrType = self.read_reg(rs1) as AddrType;
                addr = rs1_data.wrapping_add(addr);
                addr = addr & (!0x01);

                self.write_reg(rd, (self.m_pc + 4) as XlenT);
                self.m_pc = addr;
                self.set_update_pc(true);
            }
            RiscvInst::FENCE => {}
            RiscvInst::FENCEI => {}
            RiscvInst::ECALL => {
                self.m_csr.csrrw(CsrAddr::Mepc, self.m_pc as XlenT); // MEPC

                let current_priv: PrivMode = self.m_priv;

                match current_priv {
                    PrivMode::User => self.generate_exception(ExceptCode::EcallFromUMode, 0),
                    PrivMode::Supervisor => self.generate_exception(ExceptCode::EcallFromSMode, 0),
                    PrivMode::Hypervisor => self.generate_exception(ExceptCode::EcallFromHMode, 0),
                    PrivMode::Machine => self.generate_exception(ExceptCode::EcallFromMMode, 0),
                }
                self.set_update_pc(true);
            }
            RiscvInst::EBREAK => {}
            RiscvInst::URET => {}
            RiscvInst::SRET => {
                let mstatus: XlenT = self
                    .m_csr
                    .csrrs(CsrAddr::Mstatus, PrivMode::Machine as XlenT);
                let next_priv_uint: XlenT = Self::extract_bit_field(
                    mstatus,
                    SYSREG_MSTATUS_SPP_MSB,
                    SYSREG_MSTATUS_SPP_LSB,
                );
                let next_priv: PrivMode = PrivMode::from_u8(next_priv_uint as u8);
                let mut next_mstatus: XlenT = mstatus;
                next_mstatus = Self::set_bit_field(
                    next_mstatus,
                    Self::extract_bit_field(
                        mstatus,
                        SYSREG_MSTATUS_SPIE_MSB,
                        SYSREG_MSTATUS_SPIE_LSB,
                    ),
                    SYSREG_MSTATUS_SIE_MSB,
                    SYSREG_MSTATUS_SIE_LSB,
                );
                next_mstatus = Self::set_bit_field(
                    next_mstatus,
                    1,
                    SYSREG_MSTATUS_SPIE_MSB,
                    SYSREG_MSTATUS_SPIE_LSB,
                );
                next_mstatus = Self::set_bit_field(
                    next_mstatus,
                    PrivMode::User as XlenT,
                    SYSREG_MSTATUS_SPP_MSB,
                    SYSREG_MSTATUS_SPP_LSB,
                );

                self.m_csr.csrrw(CsrAddr::Mstatus, next_mstatus);
                let ret_pc = self.m_csr.csrrs(CsrAddr::Sepc, 0);
                self.set_priv_mode(next_priv);

                self.set_pc(ret_pc as AddrType);
                self.set_update_pc(true);
            }
            RiscvInst::MRET => {
                let mepc: XlenT = self.m_csr.csrrs(CsrAddr::Mepc, 0); // MEPC
                self.m_pc = mepc as AddrType;
                self.set_update_pc(true);
            }
            _ => {}
        }

        if self.is_update_pc() == false {
            self.m_pc += 4;
        }

        self.m_trace.print_trace();
        self.m_trace.clear();
    }

    fn get_vm_mode(&mut self) -> VMMode {
        let satp_val = self.m_csr.csrrs(CsrAddr::Satp, 0); // SATP;
        let mode = Self::extract_bit_field(satp_val, SYSREG_SATP_MODE_MSB, SYSREG_SATP_MODE_LSB);
        return if mode == 1 {
            VMMode::Sv32
        } else {
            VMMode::Mbare
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

    fn get_tohost(&mut self) -> XlenT {
        return self.m_tohost;
    }
    fn get_fromhost(&mut self) -> XlenT {
        return self.m_fromhost;
    }
}
