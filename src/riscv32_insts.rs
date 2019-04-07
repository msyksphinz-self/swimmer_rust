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
    fn decode_inst(&mut self, inst: InstT) -> RiscvInst;
    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstT, step: u32);
}

impl RiscvInsts for Riscv32Env {
    fn decode_inst(&mut self, inst: InstT) -> RiscvInst {
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

    fn execute_inst(&mut self, dec_inst: RiscvInst, inst: InstT, step: u32) {
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
                let (result, reg_data) = self.read_bus_byte(addr as AddrT);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 7);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInst::LH => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as AddrT);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 15);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInst::LW => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_word(addr as AddrT);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data);
                }
            }
            RiscvInst::LBU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as AddrT);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as XlenT);
                }
            }
            RiscvInst::LHU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as AddrT);
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
                let addr: AddrT = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrT;
                self.write_bus_byte(addr, rs2_data);
            }
            RiscvInst::SH => {
                let rs2_data = self.read_reg(rs2);
                let addr: AddrT = (self.read_reg(rs1) + Self::extract_sfield(inst)) as AddrT;
                self.write_bus_hword(addr, rs2_data);
            }
            RiscvInst::SW => {
                let rs2_data = self.read_reg(rs2);
                let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
                self.write_bus_word(addr as AddrT, rs2_data);
            }
            RiscvInst::JAL => {
                let addr: AddrT = Self::extract_uj_field(inst) as AddrT;
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
                let addr: AddrT = Self::extract_sb_field(inst) as AddrT;
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
                let mut addr: AddrT = Self::extract_ifield(inst) as AddrT;
                let rs1_data: AddrT = self.read_reg(rs1) as AddrT;
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

                self.set_pc(ret_pc as AddrT);
                self.set_update_pc(true);
            }
            RiscvInst::MRET => {
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
