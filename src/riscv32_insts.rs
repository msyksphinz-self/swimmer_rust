use crate::riscv32_core::Riscv32Core;
use crate::riscv32_core::Riscv32Env;

use crate::riscv64_core::Riscv64Core;

use crate::riscv_csr::CsrAddr;

use crate::riscv32_core::PrivMode;

use crate::riscv32_core::MemResult;

use crate::riscv_exception::ExceptCode;
use crate::riscv_exception::RiscvException;

use crate::riscv_tracer::RiscvTracer;

use crate::riscv32_core::AddrT;
use crate::riscv32_core::InstT;
use crate::riscv32_core::UXlenT;
use crate::riscv32_core::XlenT;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_MSB;

pub enum RiscvInstId {
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
    LWU,   // for RV64
    LD,    // for RV64
    SD,    // for RV64
    ADDIW, // for RV64
    SLLIW, // for RV64
    SRLIW, // for RV64
    SRAIW, // for RV64
    ADDW,  // for RV64
    SUBW,  // for RV64
    SLLW,  // for RV64
    SRLW,  // for RV64
    SRAW,  // for RV64
}

pub trait RiscvInsts {
    fn decode_inst(&mut self, inst: InstT) -> RiscvInstId;
    fn execute_inst(&mut self, dec_inst: RiscvInstId, inst: InstT, step: u32);
}

impl RiscvInsts for Riscv32Env {
    fn decode_inst(&mut self, inst: InstT) -> RiscvInstId {
        let opcode = inst & 0x7f;
        let funct3 = (inst >> 12) & 0x07;
        let funct5 = (inst >> 25) & 0x7f;
        let imm12 = (inst >> 20) & 0xfff;

        let dec_inst: RiscvInstId;

        match opcode {
            0x0f => match funct3 {
                0b000 => dec_inst = RiscvInstId::FENCE,
                0b001 => dec_inst = RiscvInstId::FENCEI,
                _ => dec_inst = RiscvInstId::NOP,
            },
            0x33 => match funct5 {
                0b0000000 => match funct3 {
                    0b000 => dec_inst = RiscvInstId::ADD,
                    0b001 => dec_inst = RiscvInstId::SLL,
                    0b010 => dec_inst = RiscvInstId::SLT,
                    0b011 => dec_inst = RiscvInstId::SLTU,
                    0b100 => dec_inst = RiscvInstId::XOR,
                    0b101 => dec_inst = RiscvInstId::SRL,
                    0b110 => dec_inst = RiscvInstId::OR,
                    0b111 => dec_inst = RiscvInstId::AND,
                    _ => dec_inst = RiscvInstId::NOP,
                },
                0b0100000 => match funct3 {
                    0b000 => dec_inst = RiscvInstId::SUB,
                    0b101 => dec_inst = RiscvInstId::SRA,
                    _ => dec_inst = RiscvInstId::NOP,
                },
                0b0000001 => match funct3 {
                    0b000 => dec_inst = RiscvInstId::MUL,
                    0b001 => dec_inst = RiscvInstId::MULH,
                    0b010 => dec_inst = RiscvInstId::MULHSU,
                    0b011 => dec_inst = RiscvInstId::MULHU,
                    0b100 => dec_inst = RiscvInstId::DIV,
                    0b101 => dec_inst = RiscvInstId::DIVU,
                    0b110 => dec_inst = RiscvInstId::REM,
                    0b111 => dec_inst = RiscvInstId::REMU,
                    _ => dec_inst = RiscvInstId::NOP,
                },
                _ => dec_inst = RiscvInstId::NOP,
            },
            0x03 => match funct3 {
                0b000 => dec_inst = RiscvInstId::LB,
                0b001 => dec_inst = RiscvInstId::LH,
                0b010 => dec_inst = RiscvInstId::LW,
                0b100 => dec_inst = RiscvInstId::LBU,
                0b101 => dec_inst = RiscvInstId::LHU,
                _ => dec_inst = RiscvInstId::NOP,
            },
            0x23 => match funct3 {
                0b000 => dec_inst = RiscvInstId::SB,
                0b001 => dec_inst = RiscvInstId::SH,
                0b010 => dec_inst = RiscvInstId::SW,
                _ => dec_inst = RiscvInstId::NOP,
            },
            0x37 => dec_inst = RiscvInstId::LUI,
            0x17 => dec_inst = RiscvInstId::AUIPC,
            0x63 => match funct3 {
                0b000 => dec_inst = RiscvInstId::BEQ,
                0b001 => dec_inst = RiscvInstId::BNE,
                0b100 => dec_inst = RiscvInstId::BLT,
                0b101 => dec_inst = RiscvInstId::BGE,
                0b110 => dec_inst = RiscvInstId::BLTU,
                0b111 => dec_inst = RiscvInstId::BGEU,
                _ => dec_inst = RiscvInstId::NOP,
            },
            0x13 => match funct3 {
                0b000 => dec_inst = RiscvInstId::ADDI,
                0b010 => dec_inst = RiscvInstId::SLTI,
                0b011 => dec_inst = RiscvInstId::SLTIU,
                0b100 => dec_inst = RiscvInstId::XORI,
                0b110 => dec_inst = RiscvInstId::ORI,
                0b111 => dec_inst = RiscvInstId::ANDI,
                0b001 => dec_inst = RiscvInstId::SLLI,
                0b101 => match funct5 {
                    0b0000000 => dec_inst = RiscvInstId::SRLI,
                    0b0100000 => dec_inst = RiscvInstId::SRAI,
                    _ => dec_inst = RiscvInstId::NOP,
                },
                _ => dec_inst = RiscvInstId::NOP,
            },
            0x6f => dec_inst = RiscvInstId::JAL,
            0x67 => dec_inst = RiscvInstId::JALR,
            0x73 => match funct3 {
                0x000 => match imm12 {
                    0x000 => dec_inst = RiscvInstId::ECALL,
                    0x001 => dec_inst = RiscvInstId::EBREAK,
                    0x002 => dec_inst = RiscvInstId::URET,
                    0x102 => dec_inst = RiscvInstId::SRET,
                    0x302 => dec_inst = RiscvInstId::MRET,
                    _ => dec_inst = RiscvInstId::NOP,
                },
                0b001 => dec_inst = RiscvInstId::CSRRW,
                0b010 => dec_inst = RiscvInstId::CSRRS,
                0b011 => dec_inst = RiscvInstId::CSRRC,
                0b101 => dec_inst = RiscvInstId::CSRRWI,
                0b110 => dec_inst = RiscvInstId::CSRRSI,
                0b111 => dec_inst = RiscvInstId::CSRRCI,
                _ => dec_inst = RiscvInstId::NOP,
            },
            _ => dec_inst = RiscvInstId::WFI,
        }

        return dec_inst;
    }

    fn execute_inst(&mut self, dec_inst: RiscvInstId, inst: InstT, step: u32) {
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
            RiscvInstId::CSRRW => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenT = self.m_csr.csrrw(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRS => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenT = self.m_csr.csrrs(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRC => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: XlenT = self.m_csr.csrrc(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRWI => {
                let zimm: XlenT = ((inst >> 15) & 0x1f) as XlenT;
                let reg_data: XlenT = self.m_csr.csrrw(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRSI => {
                let zimm: XlenT = ((inst >> 15) & 0x1f) as XlenT;
                let reg_data: XlenT = self.m_csr.csrrs(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRCI => {
                let zimm: XlenT = ((inst >> 15) & 0x1f) as XlenT;
                let reg_data: XlenT = self.m_csr.csrrc(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::LUI => {
                let mut imm: XlenT =
                    Self::extend_sign(Self::extract_bit_field(inst as XlenT, 31, 12), 19);
                imm = imm << 12;
                self.write_reg(rd, imm);
            }
            RiscvInstId::AUIPC => {
                let mut imm: XlenT =
                    Self::extend_sign(Self::extract_bit_field(inst as XlenT, 31, 12), 19);
                imm = (imm << 12).wrapping_add(self.m_pc as XlenT);
                self.write_reg(rd, imm);
            }
            RiscvInstId::LB => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as AddrT);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 7);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInstId::LH => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as AddrT);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 15);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInstId::LW => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_word(addr as AddrT);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data);
                }
            }
            RiscvInstId::LBU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as AddrT);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as XlenT);
                }
            }
            RiscvInstId::LHU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as AddrT);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as XlenT);
                }
            }
            RiscvInstId::ADDI => {
                let rs1_data = self.read_reg(rs1);
                let imm_data = Self::extract_ifield(inst);
                let reg_data: XlenT = rs1_data.wrapping_add(imm_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLTI => {
                let reg_data: bool = self.read_reg(rs1) < Self::extract_ifield(inst);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::SLTIU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlenT) < (Self::extract_ifield(inst) as UXlenT);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::XORI => {
                let reg_data: XlenT = self.read_reg(rs1) ^ Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::ORI => {
                let reg_data: XlenT = self.read_reg(rs1) | Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::ANDI => {
                let reg_data: XlenT = self.read_reg(rs1) & Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLLI => {
                let reg_data: XlenT = self.read_reg(rs1) << Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SRLI => {
                let reg_data: UXlenT =
                    (self.read_reg(rs1) as UXlenT) >> Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::SRAI => {
                let reg_data: XlenT = self.read_reg(rs1) >> Self::extract_shamt_field(inst);
                self.write_reg(rd, reg_data);
            }

            RiscvInstId::ADD => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT = rs1_data.wrapping_add(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SUB => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT = rs1_data.wrapping_sub(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: XlenT = rs1_data.wrapping_shl(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLT => {
                let reg_data: bool = self.read_reg(rs1) < self.read_reg(rs2);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::SLTU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlenT) < (self.read_reg(rs2) as UXlenT);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::XOR => {
                let reg_data: XlenT = self.read_reg(rs1) ^ self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SRL => {
                let rs1_data = self.read_reg(rs1) as UXlenT;
                let rs2_data = self.read_reg(rs2);
                let reg_data = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::SRA => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: XlenT = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data);
            }

            RiscvInstId::MUL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: XlenT = rs1_data.wrapping_mul(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::MULH => {
                let rs1_data: i64 = self.read_reg(rs1) as i64;
                let rs2_data: i64 = self.read_reg(rs2) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0x0ffffffff;
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::MULHSU => {
                let rs1_data: i64 = (self.read_reg(rs1) as i32) as i64;
                let rs2_data: i64 = (self.read_reg(rs2) as u32) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as XlenT);
            }
            RiscvInstId::MULHU => {
                let rs1_data: u64 = (self.read_reg(rs1) as u32) as u64;
                let rs2_data: u64 = (self.read_reg(rs2) as u32) as u64;
                let mut reg_data: u64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as XlenT);
            }

            RiscvInstId::REM => {
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
            RiscvInstId::REMU => {
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

            RiscvInstId::DIV => {
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
            RiscvInstId::DIVU => {
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

            RiscvInstId::OR => {
                let reg_data: XlenT = self.read_reg(rs1) | self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::AND => {
                let reg_data: XlenT = self.read_reg(rs1) & self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SB => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrT = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrT;
                self.write_bus_byte(addr, rs2_data);
            }
            RiscvInstId::SH => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrT = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrT;
                self.write_bus_hword(addr, rs2_data);
            }
            RiscvInstId::SW => {
                let rs2_data = self.read_reg(rs2);
                let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
                self.write_bus_word(addr as AddrT, rs2_data);
            }
            RiscvInstId::JAL => {
                let addr: AddrT = Self::extract_uj_field(inst) as AddrT;
                self.write_reg(rd, (self.m_pc + 4) as XlenT);
                self.m_pc = self.m_pc.wrapping_add(addr);
                self.set_update_pc(true);
            }
            RiscvInstId::BEQ
            | RiscvInstId::BNE
            | RiscvInstId::BLT
            | RiscvInstId::BGE
            | RiscvInstId::BLTU
            | RiscvInstId::BGEU => {
                let rs1_data: XlenT = self.read_reg(rs1);
                let rs2_data: XlenT = self.read_reg(rs2);
                let addr: AddrT = Self::extract_sb_field(inst) as AddrT;
                let jump_en: bool;
                match dec_inst {
                    RiscvInstId::BEQ => jump_en = rs1_data == rs2_data,
                    RiscvInstId::BNE => jump_en = rs1_data != rs2_data,
                    RiscvInstId::BLT => jump_en = rs1_data < rs2_data,
                    RiscvInstId::BGE => jump_en = rs1_data >= rs2_data,
                    RiscvInstId::BLTU => jump_en = (rs1_data as UXlenT) < (rs2_data as UXlenT),
                    RiscvInstId::BGEU => jump_en = (rs1_data as UXlenT) >= (rs2_data as UXlenT),
                    _ => panic!("Unknown value Branch"),
                }
                if jump_en {
                    self.m_pc = self.m_pc.wrapping_add(addr);
                    self.set_update_pc(true);
                }
            }
            RiscvInstId::JALR => {
                let mut addr: AddrT = Self::extract_ifield(inst) as AddrT;
                let rs1_data: AddrT = self.read_reg(rs1) as AddrT;
                addr = rs1_data.wrapping_add(addr);
                addr = addr & (!0x01);

                self.write_reg(rd, (self.m_pc + 4) as XlenT);
                self.m_pc = addr;
                self.set_update_pc(true);
            }
            RiscvInstId::FENCE => {}
            RiscvInstId::FENCEI => {}
            RiscvInstId::ECALL => {
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
            RiscvInstId::EBREAK => {}
            RiscvInstId::URET => {}
            RiscvInstId::SRET => {
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

                self.set_pc(ret_pc as AddrT);
                self.set_update_pc(true);
            }
            RiscvInstId::MRET => {
                let mepc: XlenT = self.m_csr.csrrs(CsrAddr::Mepc, 0); // MEPC
                self.m_pc = mepc as AddrT;
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
}
