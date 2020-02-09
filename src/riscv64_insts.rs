use crate::riscv_csr::CsrAddr;

use crate::riscv_tracer::RiscvTracer;

use crate::riscv32_core::UXlenT;
use crate::riscv32_core::XlenT;
use crate::riscv64_core::UXlen64T;
use crate::riscv64_core::Addr64T;
use crate::riscv64_core::Xlen64T;

use crate::riscv32_core::PrivMode;

use crate::riscv32_core::InstT;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv_exception::ExceptCode;
use crate::riscv_exception::RiscvException;

use crate::riscv_inst_id::RiscvInstId;
use crate::riscv32_insts::RiscvInsts;

use crate::riscv64_insts_fpu::Riscv64InstsFpu;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_SPP_MSB;

impl RiscvInsts for Riscv64Env {
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
                match self.read_bus_byte(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, (reg_data as i8) as Xlen64T);
                    },
                    Err(_result) => {},
                }
            }
            RiscvInstId::LH => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                match self.read_bus_hword(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, (reg_data as i16) as Xlen64T);
                    },
                    Err(_result) => {},
                }
            }
            RiscvInstId::LW => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                match self.read_bus_word(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, (reg_data as XlenT) as Xlen64T);
                    },
                    Err(_result) => {},
                }
            }
            RiscvInstId::LD => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                match self.read_bus_dword(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, reg_data);
                    },
                    Err(_result) => {},
                }
            }
            RiscvInstId::LBU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                match self.read_bus_byte(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, reg_data as Xlen64T);
                    },
                    Err(_result) => {},
                }
            }
            RiscvInstId::LHU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                match self.read_bus_hword(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, reg_data as Xlen64T);
                    },
                    Err(_result) => {},
                }
            }
            RiscvInstId::LWU => {
                let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
                match self.read_bus_word(addr as Addr64T) {
                    Ok(reg_data) => {
                        self.write_reg(rd, reg_data & 0xffffffff);
                    },
                    Err(_result) => {},
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
                let rs1_data: i128 = (self.read_reg(rs1) as i64) as i128;
                let rs2_data: i128 = (self.read_reg(rs2) as i64) as i128;
                let mut reg_data: i128 = rs1_data.wrapping_mul(rs2_data);
                reg_data = reg_data >> 64;
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::MULHSU => {
                let rs1_data: i128 = (self.read_reg(rs1) as i64) as i128;
                let rs2_data: i128 = (self.read_reg(rs2) as u64) as i128;
                let mut reg_data: i128 = rs1_data.wrapping_mul(rs2_data);
                reg_data = reg_data >> 64;
                self.write_reg(rd, reg_data as Xlen64T);
            }
            RiscvInstId::MULHU => {
                let rs1_data: u128 = (self.read_reg(rs1) as u64) as u128;
                let rs2_data: u128 = (self.read_reg(rs2) as u64) as u128;
                let mut reg_data: u128 = rs1_data.wrapping_mul(rs2_data);
                reg_data = reg_data >> 64;
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
                    reg_data = 0xffffffffffffffff as UXlen64T;
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
            RiscvInstId::FENCE_I => {}
            RiscvInstId::SFENCE_VMA => {}
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

            RiscvInstId::FLW => self.execute_flw(inst),
            RiscvInstId::FSW => self.execute_fsw(inst),
            RiscvInstId::FADD_S  => self.execute_fadd_s(inst),
            RiscvInstId::FSUB_S  => self.execute_fsub_s(inst),
            RiscvInstId::FMUL_S  => self.execute_fmul_s(inst),
            RiscvInstId::FMV_X_W => self.execute_fmv_x_w(inst),

            RiscvInstId::FLD => self.execute_fld(inst),
            RiscvInstId::FSD => self.execute_fsd(inst),
            RiscvInstId::FADD_D => self.execute_fadd_d(inst),
            RiscvInstId::FSUB_D => self.execute_fsub_d(inst),
            RiscvInstId::FMUL_D => self.execute_fmul_d(inst),
            RiscvInstId::FMV_X_D => self.execute_fmv_x_d(inst),

            _ => { panic!("Unimplemneted instruction. Stop."); }
        }

        if self.is_update_pc() == false {
            self.m_pc += 4;
        }

        self.m_trace.print_trace();
        self.m_trace.clear();
    }
}
