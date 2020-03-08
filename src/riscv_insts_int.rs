use crate::riscv_core::UXlenT;
use crate::riscv_core::XlenT;
use crate::riscv_core::Xlen64T;
use crate::riscv_core::UXlen64T;
use crate::riscv_core::Addr64T;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::Riscv64Env;

use crate::riscv_core::InstT;

pub trait Riscv64InstsInt {
    fn execute_lui   (&mut self, inst: InstT);
    fn execute_auipc (&mut self, inst: InstT);
    fn execute_addi  (&mut self, inst: InstT);
    fn execute_slti  (&mut self, inst: InstT);
    fn execute_sltiu (&mut self, inst: InstT);
    fn execute_xori  (&mut self, inst: InstT);
    fn execute_ori   (&mut self, inst: InstT);
    fn execute_andi  (&mut self, inst: InstT);
    fn execute_slli  (&mut self, inst: InstT);
    fn execute_srli  (&mut self, inst: InstT);
    fn execute_srai  (&mut self, inst: InstT);
    fn execute_add   (&mut self, inst: InstT);
    fn execute_sub   (&mut self, inst: InstT);
    fn execute_sll   (&mut self, inst: InstT);
    fn execute_slt   (&mut self, inst: InstT);
    fn execute_sltu  (&mut self, inst: InstT);
    fn execute_xor   (&mut self, inst: InstT);
    fn execute_srl   (&mut self, inst: InstT);
    fn execute_sra   (&mut self, inst: InstT);
}

impl Riscv64InstsInt for Riscv64Env {
    fn execute_lui (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let mut imm: Xlen64T =
            Self::extend_sign(Self::extract_bit_field(inst as Xlen64T, 31, 12), 19);
        imm = imm << 12;
        self.write_reg(rd, imm);
    }
    fn execute_auipc (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let mut imm: Xlen64T =
            Self::extend_sign(Self::extract_bit_field(inst as Xlen64T, 31, 12), 19);
        imm = (imm << 12).wrapping_add(self.uext_xlen(self.m_pc as Xlen64T) as Xlen64T);
        self.write_reg(rd, imm);
    }
    fn execute_addi (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1);
        let imm_data = Self::extract_ifield(inst);
        let reg_data: Xlen64T = self.sext_xlen(rs1_data.wrapping_add(imm_data));
        self.write_reg(rd, reg_data);
    }
    fn execute_slti (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let reg_data: bool = self.read_reg(rs1) < Self::extract_ifield(inst);
        self.write_reg(rd, reg_data as Xlen64T);
    }
    fn execute_sltiu (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let reg_data: bool =
            (self.read_reg(rs1) as UXlen64T) < (Self::extract_ifield(inst) as UXlen64T);
        self.write_reg(rd, reg_data as Xlen64T);
    }
    fn execute_xori (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let data = self.read_reg(rs1) ^ Self::extract_ifield(inst);
        let reg_data: Xlen64T = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_ori (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let data = self.read_reg(rs1) | Self::extract_ifield(inst);
        let reg_data: Xlen64T = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_andi (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let data = self.read_reg(rs1) & Self::extract_ifield(inst);
        let reg_data: Xlen64T = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_slli (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let shamt_mask = if self.m_xlen == 32 { 0x1f } else { 0x3f };
        let shamt: u32 = (Self::extract_shamt_field(inst) & shamt_mask) as u32;
        let data = (self.read_reg(rs1) as UXlen64T).wrapping_shl(shamt) as Xlen64T;
        let reg_data = self.sext_xlen(data);
        self.write_reg(rd, reg_data as Xlen64T);
    }
    fn execute_srli (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let shamt_mask = if self.m_xlen == 32 { 0x1f } else { 0x3f };
        let shamt: u32 = (Self::extract_shamt_field(inst) & shamt_mask) as u32;
        let rs1_data_64 = self.read_reg(rs1) as UXlen64T;
        let rs1_data = self.uext_xlen(rs1_data_64 as Xlen64T);
        let data = rs1_data.wrapping_shr(shamt) as Xlen64T;
        let reg_data = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_srai (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let shamt_mask = if self.m_xlen == 32 { 0x1f } else { 0x3f };
        let shamt: u32 = (Self::extract_shamt_field(inst) & shamt_mask) as u32;
        let data = self.read_reg(rs1).wrapping_shr(shamt) as Xlen64T;
        let reg_data = self.sext_xlen(data);
        self.write_reg(rd, reg_data as Xlen64T);
    }

    fn execute_add (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1);
        let rs2_data = self.read_reg(rs2);
        let data = self.sext_xlen(rs1_data.wrapping_add(rs2_data)) as Xlen64T;
        let reg_data = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_sub (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1);
        let rs2_data = self.read_reg(rs2);
        let data = self.sext_xlen(rs1_data.wrapping_sub(rs2_data)) as Xlen64T;
        let reg_data: Xlen64T = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_sll (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1) as UXlen64T;
        let rs2_data = self.read_reg(rs2) as UXlenT;
        let data = rs1_data.wrapping_shl(rs2_data) as Xlen64T;
        let reg_data = self.sext_xlen(data);
        self.write_reg(rd, reg_data as Xlen64T);
    }
    fn execute_slt (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let reg_data: bool = self.read_reg(rs1) < self.read_reg(rs2);
        self.write_reg(rd, reg_data as Xlen64T);
    }
    fn execute_sltu (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let reg_data: bool =
            (self.read_reg(rs1) as UXlen64T) < (self.read_reg(rs2) as UXlen64T);
        self.write_reg(rd, reg_data as Xlen64T);
    }
    fn execute_xor (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let data = self.read_reg(rs1) ^ self.read_reg(rs2) as Xlen64T;
        let reg_data: Xlen64T = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_srl (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1) as UXlen64T;
        let rs1_data = self.uext_xlen(rs1_data_64 as Xlen64T);
        let rs2_data = self.read_reg(rs2);
        let data = rs1_data.wrapping_shr(rs2_data as u32) as Xlen64T;
        let reg_data = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
    fn execute_sra (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1);
        let rs2_data: UXlen64T = self.read_reg(rs2) as UXlen64T;
        let data = rs1_data.wrapping_shr(rs2_data as u32) as Xlen64T;
        let reg_data: Xlen64T = self.sext_xlen(data);
        self.write_reg(rd, reg_data);
    }
}
