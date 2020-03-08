use crate::riscv_core::UXlenT;
use crate::riscv_core::XlenT;
use crate::riscv_core::Xlen64T;
use crate::riscv_core::UXlen64T;
use crate::riscv_core::Addr64T;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::Riscv64Env;

use crate::riscv_core::InstT;

pub trait Riscv64InstsAmo {
    fn execute_amoswap_w (&mut self, inst: InstT);
    fn execute_amoadd_w (&mut self, inst: InstT);
    fn execute_amoxor_w (&mut self, inst: InstT);
    fn execute_amoand_w (&mut self, inst: InstT);
    fn execute_amoor_w (&mut self, inst: InstT);
    fn execute_amomin_w (&mut self, inst: InstT);
    fn execute_amomax_w (&mut self, inst: InstT);
    fn execute_amominu_w (&mut self, inst: InstT);
    fn execute_amomaxu_w (&mut self, inst: InstT);

    fn execute_amoswap_d (&mut self, inst: InstT);
    fn execute_amoadd_d (&mut self, inst: InstT);
    fn execute_amoxor_d (&mut self, inst: InstT);
    fn execute_amoand_d (&mut self, inst: InstT);
    fn execute_amoor_d (&mut self, inst: InstT);
    fn execute_amomin_d (&mut self, inst: InstT);
    fn execute_amomax_d (&mut self, inst: InstT);
    fn execute_amominu_d (&mut self, inst: InstT);
    fn execute_amomaxu_d (&mut self, inst: InstT);
}


impl Riscv64InstsAmo for Riscv64Env {
    fn execute_amoswap_w (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1);
        let rs2_data = self.read_reg(rs2);

        match self.read_bus_word(rs1_data as Addr64T) {
            Ok(reg_data) => {
                self.write_bus_word(rs1_data as Addr64T, rs2_data);
                self.write_reg(rd, (reg_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoadd_w  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = self.uext_xlen(rs2_data_64);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = self.uext_xlen(mem_data_64);
                let ret = mem_data.wrapping_add(rs2_data);
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoxor_w  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = self.uext_xlen(rs2_data_64);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = self.uext_xlen(mem_data_64);
                let ret = mem_data ^ rs2_data;
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoand_w  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = self.uext_xlen(rs2_data_64);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = self.uext_xlen(mem_data_64);
                let ret = mem_data & rs2_data;
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoor_w   (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = self.uext_xlen(rs2_data_64);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = self.uext_xlen(mem_data_64);
                let ret = mem_data | rs2_data;
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amomin_w  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = Self::extend_sign(rs2_data_64, 31);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = Self::extend_sign(mem_data_64, 31);
                let ret = std::cmp::min(mem_data, rs2_data);
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amomax_w  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = Self::extend_sign(rs2_data_64, 31);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = Self::extend_sign(mem_data_64, 31);
                let ret = std::cmp::max(mem_data, rs2_data);
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amominu_w (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = self.uext_xlen(rs2_data_64);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = self.uext_xlen(mem_data_64);
                let ret = std::cmp::min(mem_data as UXlenT, rs2_data as UXlenT);
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amomaxu_w (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);
        let rs2_data = self.uext_xlen(rs2_data_64);

        match self.read_bus_word(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let mem_data = self.uext_xlen(mem_data_64);
                let ret = std::cmp::max(mem_data as UXlenT, rs2_data as UXlenT);
                self.write_bus_word(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, (mem_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }

    fn execute_amoswap_d (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_reg(rs1);
        let rs2_data = self.read_reg(rs2);

        match self.read_bus_dword(rs1_data as Addr64T) {
            Ok(reg_data) => {
                self.write_bus_dword(rs1_data as Addr64T, rs2_data);
                self.write_reg(rd, (reg_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoadd_d  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = mem_data_64.wrapping_add(rs2_data_64);
                self.write_bus_dword(mem_addr as Addr64T, ret);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoxor_d  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = mem_data_64 ^ rs2_data_64;
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoand_d  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = mem_data_64 & rs2_data_64;
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amoor_d   (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = mem_data_64 | rs2_data_64;
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amomin_d  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = std::cmp::min(mem_data_64, rs2_data_64);
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amomax_d  (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = std::cmp::max(mem_data_64, rs2_data_64);
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amominu_d (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = std::cmp::min(mem_data_64 as UXlen64T, rs2_data_64 as UXlen64T);
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
    fn execute_amomaxu_d (&mut self, inst: InstT)
    {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data_64 = self.read_reg(rs1);
        let rs2_data_64 = self.read_reg(rs2);

        let mem_addr = self.uext_xlen(rs1_data_64);

        match self.read_bus_dword(mem_addr as Addr64T) {
            Ok(mem_data_64) => {
                let ret = std::cmp::max(mem_data_64 as UXlen64T, rs2_data_64 as UXlen64T);
                self.write_bus_dword(mem_addr as Addr64T, ret as Xlen64T);
                self.write_reg(rd, mem_data_64);
            },
            Err(_result) => {},
        }
    }
}
