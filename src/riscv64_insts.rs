use crate::riscv_csr::CsrAddr;

use crate::riscv_tracer::RiscvTracer;

use crate::riscv32_core::UXlenT;
use crate::riscv32_core::XlenT;
use crate::riscv64_core::UXlen64T;
use crate::riscv64_core::Addr64T;
use crate::riscv64_core::Xlen64T;

use crate::riscv32_core::PrivMode;
use crate::riscv32_core::MemResult;

use crate::riscv32_core::InstT;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv_exception::ExceptCode;
use crate::riscv_exception::RiscvException;

use crate::riscv32_insts::RiscvInstId;
use crate::riscv32_insts::RiscvInsts;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_MSB;

impl RiscvInsts for Riscv64Env {
    fn decode_inst(&mut self, inst: InstT) -> Option<RiscvInstId> {
        let opcode = inst & 0x7f;
        let funct3 = (inst >> 12) & 0x07;
        let funct7 = (inst >> 25) & 0x7f;
        let funct6 = (inst >> 26) & 0x3f; // RV64 shamt
        let rs2 = (inst >> 20) & 0x1f;

        return match opcode {
            0x0f => match funct3 {
                0b000 => Some(RiscvInstId::FENCE),
                0b001 => Some(RiscvInstId::FENCEI),
                _ => None,
            },
            0x1b => match funct3 {
                0b000 => Some(RiscvInstId::ADDIW),
                0b001 => Some(RiscvInstId::SLLIW),
                0b101 => match funct7 {
                    0b0000000 => Some(RiscvInstId::SRLIW),
                    0b0100000 => Some(RiscvInstId::SRAIW),
                    _ => None,
                },
                _ => None,
            },
            0x3b => match funct3 {
                0b000 => match funct7 {
                    0b0000000 => Some(RiscvInstId::ADDW),
                    0b0100000 => Some(RiscvInstId::SUBW),
                    0b0000001 => Some(RiscvInstId::MULW),
                    _ => None,
                },
                0b001 => match funct7 {
                    0b0000000 => Some(RiscvInstId::SLLW),
                    _ => None,
                },
                0b100 => match funct7 {
                    0b0000001 => Some(RiscvInstId::DIVW),
                    _ => None,
                },
                0b101 => match funct7 {
                    0b0000000 => Some(RiscvInstId::SRLW),
                    0b0100000 => Some(RiscvInstId::SRAW),
                    0b0000001 => Some(RiscvInstId::DIVUW),
                    _ => None,
                },
                0b110 => match funct7 {
                    0b0000001 => Some(RiscvInstId::REMW),
                    _ => None,
                },
                0b111 => match funct7 {
                    0b0000001 => Some(RiscvInstId::REMUW),
                    _ => None,
                },
                _ => None,
            },
            0x33 => match funct7 {
                0b0000000 => match funct3 {
                    0b000 => Some(RiscvInstId::ADD),
                    0b001 => Some(RiscvInstId::SLL),
                    0b010 => Some(RiscvInstId::SLT),
                    0b011 => Some(RiscvInstId::SLTU),
                    0b100 => Some(RiscvInstId::XOR),
                    0b101 => Some(RiscvInstId::SRL),
                    0b110 => Some(RiscvInstId::OR),
                    0b111 => Some(RiscvInstId::AND),
                    _ => None,
                },
                0b0100000 => match funct3 {
                    0b000 => Some(RiscvInstId::SUB),
                    0b101 => Some(RiscvInstId::SRA),
                    _ => None,
                },
                0b0000001 => match funct3 {
                    0b000 => Some(RiscvInstId::MUL),
                    0b001 => Some(RiscvInstId::MULH),
                    0b010 => Some(RiscvInstId::MULHSU),
                    0b011 => Some(RiscvInstId::MULHU),
                    0b100 => Some(RiscvInstId::DIV),
                    0b101 => Some(RiscvInstId::DIVU),
                    0b110 => Some(RiscvInstId::REM),
                    0b111 => Some(RiscvInstId::REMU),
                    _ => None,
                },
                _ => None,
            },
            0x03 => match funct3 {
                0b000 => Some(RiscvInstId::LB),
                0b001 => Some(RiscvInstId::LH),
                0b010 => Some(RiscvInstId::LW),
                0b100 => Some(RiscvInstId::LBU),
                0b101 => Some(RiscvInstId::LHU),
                0b110 => Some(RiscvInstId::LWU),
                0b011 => Some(RiscvInstId::LD),
                _ => None,
            },
            0x23 => match funct3 {
                0b000 => Some(RiscvInstId::SB),
                0b001 => Some(RiscvInstId::SH),
                0b010 => Some(RiscvInstId::SW),
                0b011 => Some(RiscvInstId::SD),
                _ => None,
            },
            0x37 => Some(RiscvInstId::LUI),
            0x17 => Some(RiscvInstId::AUIPC),
            0x63 => match funct3 {
                0b000 => Some(RiscvInstId::BEQ),
                0b001 => Some(RiscvInstId::BNE),
                0b100 => Some(RiscvInstId::BLT),
                0b101 => Some(RiscvInstId::BGE),
                0b110 => Some(RiscvInstId::BLTU),
                0b111 => Some(RiscvInstId::BGEU),
                _ => None,
            },
            0x13 => match funct3 {
                0b000 => Some(RiscvInstId::ADDI),
                0b010 => Some(RiscvInstId::SLTI),
                0b011 => Some(RiscvInstId::SLTIU),
                0b100 => Some(RiscvInstId::XORI),
                0b110 => Some(RiscvInstId::ORI),
                0b111 => Some(RiscvInstId::ANDI),
                0b001 => Some(RiscvInstId::SLLI),
                0b101 => match funct6 {
                    0b000000 => Some(RiscvInstId::SRLI),
                    0b010000 => Some(RiscvInstId::SRAI),
                    _ => None,
                },
                _ => None,
            },
            0x6f => Some(RiscvInstId::JAL),
            0x67 => Some(RiscvInstId::JALR),
            0x73 => match funct3 {
                0x000 => match funct7 {
                    0b0001001 => Some(RiscvInstId::SFENCEVMA),
                    0b0000000 => match rs2 {
                        0b00001 => Some(RiscvInstId::EBREAK),
                        0b00010 => Some(RiscvInstId::URET),
                        0b00000 => Some(RiscvInstId::ECALL),
                        _ => None,
                    }
                    0b0001000 => match rs2 {
                        0b00010 => Some(RiscvInstId::SRET),
                        _ => None,
                    }
                    0b0011000 => match rs2 {
                        0b00010 => Some(RiscvInstId::MRET),
                        _ => None,
                    }
                    _ => None,
                }
                0b001 => Some(RiscvInstId::CSRRW),
                0b010 => Some(RiscvInstId::CSRRS),
                0b011 => Some(RiscvInstId::CSRRC),
                0b101 => Some(RiscvInstId::CSRRWI),
                0b110 => Some(RiscvInstId::CSRRSI),
                0b111 => Some(RiscvInstId::CSRRCI),
                _ => None,
            },
            _ => Some(RiscvInstId::WFI),
        };
    }

    fn execute_inst(&mut self, dec_inst: RiscvInstId, inst: InstT, step: u32) {
        self.m_trace.m_executed_pc = self.m_pc;
        self.m_trace.m_inst_hex = inst;
        self.m_trace.m_dec_inst = Some(dec_inst);

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
                let reg_data: Xlen64T = self.m_csr.csrrw(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRS => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: Xlen64T = self.m_csr.csrrs(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRC => {
                let rs1_data = self.read_reg(rs1);
                let reg_data: Xlen64T = self.m_csr.csrrc(csr_addr, rs1_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRWI => {
                let zimm: Xlen64T = ((inst >> 15) & 0x1f) as Xlen64T;
                let reg_data: Xlen64T = self.m_csr.csrrw(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRSI => {
                let zimm: Xlen64T = ((inst >> 15) & 0x1f) as Xlen64T;
                let reg_data: Xlen64T = self.m_csr.csrrs(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::CSRRCI => {
                let zimm: Xlen64T = ((inst >> 15) & 0x1f) as Xlen64T;
                let reg_data: Xlen64T = self.m_csr.csrrc(csr_addr, zimm);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::LUI => {
                let mut imm: Xlen64T =
                    Self::extend_sign(Self::extract_bit_field(inst as Xlen64T, 31, 12), 19);
                imm = imm << 12;
                self.write_reg(rd, imm);
            }
            RiscvInstId::AUIPC => {
                let mut imm: Xlen64T =
                    Self::extend_sign(Self::extract_bit_field(inst as Xlen64T, 31, 12), 19);
                imm = (imm << 12).wrapping_add(self.m_pc as Xlen64T);
                self.write_reg(rd, imm);
            }
            RiscvInstId::LB => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as Addr64T);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 7);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInstId::LH => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as Addr64T);
                if result == MemResult::NoExcept {
                    let extended_reg_data = Self::extend_sign(reg_data, 15);
                    self.write_reg(rd, extended_reg_data);
                }
            }
            RiscvInstId::LW => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_word(addr as Addr64T);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data);
                }
            }
            RiscvInstId::LD => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_dword(addr as Addr64T);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data);
                }
            }
            RiscvInstId::LBU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_byte(addr as Addr64T);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as Xlen64T);
                }
            }
            RiscvInstId::LHU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_hword(addr as Addr64T);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data as Xlen64T);
                }
            }
            RiscvInstId::LWU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                let (result, reg_data) = self.read_bus_word(addr as Addr64T);
                if result == MemResult::NoExcept {
                    self.write_reg(rd, reg_data & 0xffffffff);
                }
            }
            RiscvInstId::ADDI => {
                let rs1_data = self.read_reg(rs1);
                let imm_data = Self::extract_ifield(inst);
                let reg_data: Xlen64T = rs1_data.wrapping_add(imm_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLTI => {
                let reg_data: bool = self.read_reg(rs1) < Self::extract_ifield(inst);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::SLTIU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlen64T) < (Self::extract_ifield(inst) as UXlen64T);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::XORI => {
                let reg_data: Xlen64T = self.read_reg(rs1) ^ Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::ORI => {
                let reg_data: Xlen64T = self.read_reg(rs1) | Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::ANDI => {
                let reg_data: Xlen64T = self.read_reg(rs1) & Self::extract_ifield(inst);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLLI => {
                let shamt: u32 = (Self::extract_shamt_field(inst) & 0x3f) as u32;
                let reg_data: UXlen64T = (self.read_reg(rs1) as UXlen64T).wrapping_shl(shamt);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::SRLI => {
                let shamt: u32 = (Self::extract_shamt_field(inst) & 0x3f) as u32;
                let reg_data: UXlen64T = (self.read_reg(rs1) as UXlen64T).wrapping_shr(shamt);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::SRAI => {
                let shamt: u32 = (Self::extract_shamt_field(inst) & 0x3f) as u32;
                let reg_data: Xlen64T = self.read_reg(rs1).wrapping_shr(shamt);
                self.write_reg(rd, reg_data as Xlen64T);
            }

            RiscvInstId::ADD => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T = rs1_data.wrapping_add(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SUB => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T = rs1_data.wrapping_sub(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLL => {
                let rs1_data = self.read_reg(rs1) as UXlen64T;
                let rs2_data = self.read_reg(rs2) as UXlenT;
                let reg_data = rs1_data.wrapping_shl(rs2_data);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::SLT => {
                let reg_data: bool = self.read_reg(rs1) < self.read_reg(rs2);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::SLTU => {
                let reg_data: bool =
                    (self.read_reg(rs1) as UXlen64T) < (self.read_reg(rs2) as UXlen64T);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::XOR => {
                let reg_data: Xlen64T = self.read_reg(rs1) ^ self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SRL => {
                let rs1_data = self.read_reg(rs1) as UXlen64T;
                let rs2_data = self.read_reg(rs2);
                let reg_data = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::SRA => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data: UXlen64T = self.read_reg(rs2) as UXlen64T;
                let reg_data: Xlen64T = rs1_data.wrapping_shr(rs2_data as u32);
                self.write_reg(rd, reg_data);
            }

            RiscvInstId::MUL => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T = rs1_data.wrapping_mul(rs2_data);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::MULH => {
                let rs1_data: i64 = self.read_reg(rs1) as i64;
                let rs2_data: i64 = self.read_reg(rs2) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0x0ffffffff;
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::MULHSU => {
                let rs1_data: i64 = (self.read_reg(rs1) as i32) as i64;
                let rs2_data: i64 = (self.read_reg(rs2) as u32) as i64;
                let mut reg_data: i64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::MULHU => {
                let rs1_data: u64 = (self.read_reg(rs1) as u32) as u64;
                let rs2_data: u64 = (self.read_reg(rs2) as u32) as u64;
                let mut reg_data: u64 = rs1_data.wrapping_mul(rs2_data);
                reg_data = (reg_data >> 32) & 0xffffffff;
                self.write_reg(rd, reg_data as Xlen64T);
            }

            RiscvInstId::REM => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T;
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
                let rs1_data: UXlen64T = self.read_reg(rs1) as UXlen64T;
                let rs2_data: UXlen64T = self.read_reg(rs2) as UXlen64T;
                let reg_data: UXlen64T;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, reg_data as Xlen64T);
            }

            RiscvInstId::DIV => {
                let rs1_data = self.read_reg(rs1);
                let rs2_data = self.read_reg(rs2);
                let reg_data: Xlen64T;
                if rs2_data == 0 {
                    reg_data = -1;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::DIVU => {
                let rs1_data: UXlen64T = self.read_reg(rs1) as UXlen64T;
                let rs2_data: UXlen64T = self.read_reg(rs2) as UXlen64T;
                let reg_data: UXlen64T;
                if rs2_data == 0 {
                    reg_data = 0xffffffff;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, reg_data as Xlen64T);
            }

            RiscvInstId::OR => {
                let reg_data: Xlen64T = self.read_reg(rs1) | self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::AND => {
                let reg_data: Xlen64T = self.read_reg(rs1) & self.read_reg(rs2);
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SB => {
                let rs2_data = self.read_reg(rs2);
                let addr: Addr64T = (self.read_reg(rs1) + Self::extract_sfield(inst)) as Addr64T;
                self.write_bus_byte(addr as Addr64T, rs2_data);
            }
            RiscvInstId::SH => {
                let rs2_data = self.read_reg(rs2);
                let addr: Addr64T = (self.read_reg(rs1) + Self::extract_sfield(inst)) as Addr64T;
                self.write_bus_hword(addr as Addr64T, rs2_data);
            }
            RiscvInstId::SW => {
                let rs2_data = self.read_reg(rs2);
                let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
                self.write_bus_word(addr as Addr64T, rs2_data);
            }
            RiscvInstId::SD => {
                let rs2_data = self.read_reg(rs2);
                let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
                self.write_bus_dword(addr as Addr64T, rs2_data);
            }
            RiscvInstId::JAL => {
                let addr: Addr64T = Self::extract_uj_field(inst) as Addr64T;
                self.write_reg(rd, (self.m_pc + 4) as Xlen64T);
                self.m_pc = self.m_pc.wrapping_add(addr);
                self.set_update_pc(true);
            }
            RiscvInstId::BEQ
            | RiscvInstId::BNE
            | RiscvInstId::BLT
            | RiscvInstId::BGE
            | RiscvInstId::BLTU
            | RiscvInstId::BGEU => {
                let rs1_data: Xlen64T = self.read_reg(rs1);
                let rs2_data: Xlen64T = self.read_reg(rs2);
                let addr: Addr64T = Self::extract_sb_field(inst) as Addr64T;
                let jump_en: bool;
                match dec_inst {
                    RiscvInstId::BEQ => jump_en = rs1_data == rs2_data,
                    RiscvInstId::BNE => jump_en = rs1_data != rs2_data,
                    RiscvInstId::BLT => jump_en = rs1_data < rs2_data,
                    RiscvInstId::BGE => jump_en = rs1_data >= rs2_data,
                    RiscvInstId::BLTU => jump_en = (rs1_data as UXlen64T) < (rs2_data as UXlen64T),
                    RiscvInstId::BGEU => jump_en = (rs1_data as UXlen64T) >= (rs2_data as UXlen64T),
                    _ => panic!("Unknown value Branch"),
                }
                if jump_en {
                    self.m_pc = self.m_pc.wrapping_add(addr);
                    self.set_update_pc(true);
                }
            }
            RiscvInstId::JALR => {
                let mut addr: Addr64T = Self::extract_ifield(inst) as Addr64T;
                let rs1_data: Addr64T = self.read_reg(rs1) as Addr64T;
                addr = rs1_data.wrapping_add(addr);
                addr = addr & (!0x01);

                self.write_reg(rd, (self.m_pc + 4) as Xlen64T);
                self.m_pc = addr;
                self.set_update_pc(true);
            }
            RiscvInstId::FENCE => {}
            RiscvInstId::FENCEI => {}
            RiscvInstId::SFENCEVMA => {}
            RiscvInstId::ECALL => {
                self.m_csr.csrrw(CsrAddr::Mepc, self.m_pc as Xlen64T); // MEPC

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
                let mstatus: Xlen64T = self
                    .m_csr
                    .csrrs(CsrAddr::Mstatus, PrivMode::Machine as Xlen64T);
                let next_priv_uint: Xlen64T = Self::extract_bit_field(
                    mstatus,
                    SYSREG_MSTATUS_SPP_MSB,
                    SYSREG_MSTATUS_SPP_LSB,
                );
                let next_priv: PrivMode = PrivMode::from_u8(next_priv_uint as u8);
                let mut next_mstatus: Xlen64T = mstatus;
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
                    PrivMode::User as Xlen64T,
                    SYSREG_MSTATUS_SPP_MSB,
                    SYSREG_MSTATUS_SPP_LSB,
                );

                self.m_csr.csrrw(CsrAddr::Mstatus, next_mstatus);
                let ret_pc = self.m_csr.csrrs(CsrAddr::Sepc, 0);
                self.set_priv_mode(next_priv);

                self.set_pc(ret_pc as Addr64T);
                self.set_update_pc(true);
            }
            RiscvInstId::MRET => {
                let mepc: Xlen64T = self.m_csr.csrrs(CsrAddr::Mepc, 0); // MEPC
                self.m_pc = mepc as Addr64T;
                self.set_update_pc(true);
            }
            RiscvInstId::ADDIW => {
                let rs1_data = self.read_reg(rs1) as i32;
                let imm_data = Self::extract_ifield(inst) as i32;
                let reg_data = rs1_data.wrapping_add(imm_data) as Xlen64T;
                self.write_reg(rd, reg_data);
            }
            RiscvInstId::SLLIW => {
                let rs1_data = self.read_reg(rs1) as i32;
                let imm_data = Self::extract_shamt_field(inst) & 0x01f;
                let reg_data = rs1_data << imm_data;
                self.write_reg(rd, reg_data as i64);
            }
            RiscvInstId::SRLIW => {
                let rs1_data = self.read_reg(rs1) as u32;
                let imm_data = Self::extract_shamt_field(inst) & 0x01f;
                let reg_data = rs1_data >> imm_data;
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::SRAIW => {
                let rs1_data = self.read_reg(rs1) as i32;
                let imm_data = Self::extract_shamt_field(inst) & 0x01f;
                let reg_data = rs1_data >> imm_data;
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::ADDW => {
                let rs1_data = self.read_reg(rs1) as i32;
                let rs2_data = self.read_reg(rs2) as i32;
                let reg_data = rs1_data.wrapping_add(rs2_data);
                self.write_reg(rd, reg_data.into());
            }
            RiscvInstId::SUBW => {
                let rs1_data = self.read_reg(rs1) as i32;
                let rs2_data = self.read_reg(rs2) as i32;
                let reg_data = rs1_data.wrapping_sub(rs2_data);
                self.write_reg(rd, reg_data.into());
            }
            RiscvInstId::SLLW => {
                let rs1_data = self.read_reg(rs1) as UXlenT;
                let rs2_data = self.read_reg(rs2) as UXlenT;
                let reg_data = rs1_data.wrapping_shl(rs2_data);
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::SRLW => {
                let rs1_data = self.read_reg(rs1) as UXlenT;
                let rs2_data = self.read_reg(rs2) as UXlenT;
                let shamt: UXlenT = rs2_data & 0x1f;
                let reg_data = rs1_data.wrapping_shr(shamt);
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::SRAW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let shamt: UXlenT = (rs2_data & 0x1f) as UXlenT;
                let reg_data = rs1_data.wrapping_shr(shamt);
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }

            RiscvInstId::MULW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let reg_data: XlenT = rs1_data.wrapping_mul(rs2_data);
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }

            RiscvInstId::DIVW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let reg_data: XlenT;
                if rs2_data == 0 {
                    reg_data = -1;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::DIVUW => {
                let rs1_data: UXlenT = self.read_reg(rs1) as UXlenT;
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: UXlenT;
                if rs2_data == 0 {
                    reg_data = 0xffffffff;
                } else {
                    reg_data = rs1_data.wrapping_div(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }

            RiscvInstId::REMW => {
                let rs1_data = self.read_reg(rs1) as XlenT;
                let rs2_data = self.read_reg(rs2) as XlenT;
                let reg_data: XlenT;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else if rs2_data == -1 {
                    reg_data = 0;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
            }
            RiscvInstId::REMUW => {
                let rs1_data: UXlenT = self.read_reg(rs1) as UXlenT;
                let rs2_data: UXlenT = self.read_reg(rs2) as UXlenT;
                let reg_data: UXlenT;
                if rs2_data == 0 {
                    reg_data = rs1_data;
                } else {
                    reg_data = rs1_data.wrapping_rem(rs2_data);
                }
                self.write_reg(rd, Self::extend_sign(reg_data as Xlen64T, 31));
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
