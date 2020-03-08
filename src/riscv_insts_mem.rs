use crate::riscv_core::UXlenT;
use crate::riscv_core::XlenT;
use crate::riscv_core::Xlen64T;
use crate::riscv_core::UXlen64T;
use crate::riscv_core::Addr64T;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::Riscv64Env;

use crate::riscv_core::InstT;

pub trait Riscv64InstsMem {
    fn execute_lb  (&mut self, inst: InstT);
    fn execute_lh  (&mut self, inst: InstT);
    fn execute_lw  (&mut self, inst: InstT);
    fn execute_ld  (&mut self, inst: InstT);
    fn execute_lbu (&mut self, inst: InstT);
    fn execute_lhu (&mut self, inst: InstT);
    fn execute_lwu (&mut self, inst: InstT);

    fn execute_sb  (&mut self, inst: InstT);
    fn execute_sh  (&mut self, inst: InstT);
    fn execute_sw  (&mut self, inst: InstT);
    fn execute_sd  (&mut self, inst: InstT);
}

impl Riscv64InstsMem for Riscv64Env {
    fn execute_lb (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_byte(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_reg(rd, (reg_data as i8) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_lh (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_hword(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_reg(rd, (reg_data as i16) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_lw (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_word(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_reg(rd, (reg_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_ld (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_dword(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_reg(rd, reg_data);
            },
            Err(_result) => {},
        }
    }
    fn execute_lbu (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_byte(addr as Addr64T) {
            Ok(reg_data) => {
                let reg_data_xlen = self.sext_xlen(reg_data);
                self.write_reg(rd, reg_data_xlen);
            },
            Err(_result) => {},
        }
    }
    fn execute_lhu (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_hword(addr as Addr64T) {
            Ok(reg_data) => {
                let reg_data_xlen = self.sext_xlen(reg_data);
                self.write_reg(rd, reg_data_xlen);
            },
            Err(_result) => {},
        }
    }
    fn execute_lwu (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_word(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_reg(rd, reg_data & 0xffffffff);
            },
            Err(_result) => {},
        }
    }
    fn execute_sb (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs2_data = self.read_reg(rs2);
        let addr: Addr64T = (self.read_reg(rs1) + Self::extract_sfield(inst)) as Addr64T;
        self.write_bus_byte(addr as Addr64T, rs2_data);
    }
    fn execute_sh (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs2_data = self.read_reg(rs2);
        let addr: Addr64T = (self.read_reg(rs1) + Self::extract_sfield(inst)) as Addr64T;
        self.write_bus_hword(addr as Addr64T, rs2_data);
    }
    fn execute_sw (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs2_data = self.read_reg(rs2);
        let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
        self.write_bus_word(addr as Addr64T, rs2_data);
    }
    fn execute_sd (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs2_data = self.read_reg(rs2);
        let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
        self.write_bus_dword(addr as Addr64T, rs2_data);
    }
}
