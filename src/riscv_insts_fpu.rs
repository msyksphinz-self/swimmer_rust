use crate::riscv_csr::CsrAddr;

use crate::riscv_core::XlenT;
use crate::riscv_core::Xlen64T;
use crate::riscv_core::Addr64T;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::Riscv64Env;

use crate::riscv_core::InstT;

#[link(name = "softfloat", kind="static")]
extern {
    static mut softfloat_exceptionFlags: u8;
    // static mut softfloat_roundingMode: u8;
    fn f32_add(a: u32, b: u32) -> u32;
    fn f32_sub(a: u32, b: u32) -> u32;
    fn f32_mul(a: u32, b: u32) -> u32;

    fn f64_add(a: u64, b: u64) -> u64;
    fn f64_sub(a: u64, b: u64) -> u64;
    fn f64_mul(a: u64, b: u64) -> u64;
}

pub trait Riscv64InstsFpu {
    fn neg_op_32 (&mut self, inst: XlenT) -> XlenT;
    fn neg_op_64 (&mut self, inst: Xlen64T) -> Xlen64T;

    fn execute_flw (&mut self, inst: InstT);
    fn execute_fsw (&mut self, inst: InstT);
    fn execute_fadd_s (&mut self, inst: InstT);
    fn execute_fsub_s (&mut self, inst: InstT);
    fn execute_fmul_s (&mut self, inst: InstT);
    fn execute_fmv_x_w (&mut self, inst: InstT);

    fn execute_fmadd_s(&mut self, inst: InstT);
    fn execute_fmsub_s(&mut self, inst: InstT);
    fn execute_fnmsub_s(&mut self, inst: InstT);
    fn execute_fnmadd_s(&mut self, inst: InstT);

    fn execute_fld (&mut self, inst: InstT);
    fn execute_fsd (&mut self, inst: InstT);
    fn execute_fadd_d (&mut self, inst: InstT);
    fn execute_fsub_d (&mut self, inst: InstT);
    fn execute_fmul_d (&mut self, inst: InstT);
    fn execute_fmv_x_d (&mut self, inst: InstT);

    fn execute_fmadd_d(&mut self, inst: InstT);
    fn execute_fmsub_d(&mut self, inst: InstT);
    fn execute_fnmsub_d(&mut self, inst: InstT);
    fn execute_fnmadd_d(&mut self, inst: InstT);

}


impl Riscv64InstsFpu for Riscv64Env {
    fn neg_op_32 (&mut self, inst: XlenT) -> XlenT {
        ((inst as i64) ^ 0x080000000) as XlenT
    }

    fn neg_op_64 (&mut self, inst: Xlen64T) -> Xlen64T {
        ((inst as u64) ^ (0x8000000000000000 as u64)) as Xlen64T
    }

    fn execute_flw (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_word(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_freg32(rd, (reg_data as XlenT) as Xlen64T);
            },
            Err(_result) => {},
        }
    }

    fn execute_fsw (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);

        let rs2_data = self.read_freg32(rs2);
        let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
        self.write_bus_word(addr as Addr64T, rs2_data);
    }

    fn execute_fadd_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = (unsafe { f32_add(rs1_data as u32, rs2_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }

    fn execute_fsub_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = (unsafe { f32_sub(rs1_data as u32, rs2_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }

    fn execute_fmul_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = (unsafe { f32_mul(rs1_data as u32, rs2_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }

    fn execute_fmv_x_w (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        self.write_reg(rd, rs1_data);
    }

    fn execute_fmadd_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        let rs3_data = self.read_freg32(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = (unsafe { f32_add(f32_mul(rs1_data as u32, rs2_data as u32), rs3_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }
    fn execute_fmsub_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        let rs3_data = self.read_freg32(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = (unsafe { f32_sub(f32_mul(rs1_data as u32, rs2_data as u32), rs3_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }
    fn execute_fnmsub_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        let rs3_data = self.read_freg32(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = self.neg_op_32(unsafe { f32_sub(f32_mul(rs1_data as u32, rs2_data as u32), rs3_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }
    fn execute_fnmadd_s (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg32(rs1);
        let rs2_data = self.read_freg32(rs2);
        let rs3_data = self.read_freg32(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = self.neg_op_32(unsafe { f32_add(f32_mul(rs1_data as u32, rs2_data as u32), rs3_data as u32) } as XlenT) as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg32(rd, reg_data);
    }


    fn execute_fld (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let addr = self.read_reg(rs1) + Self::extract_ifield(inst);
        match self.read_bus_dword(addr as Addr64T) {
            Ok(reg_data) => {
                self.write_freg64(rd, reg_data);
            },
            Err(_result) => {},
        }
    }

    fn execute_fsd (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);

        let rs2_data = self.read_freg64(rs2);
        let addr = self.read_reg(rs1) + Self::extract_sfield(inst);
        self.write_bus_dword(addr as Addr64T, rs2_data);
    }

    fn execute_fadd_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = unsafe { f64_add(rs1_data as u64, rs2_data as u64) } as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }

    fn execute_fsub_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = unsafe { f64_sub(rs1_data as u64, rs2_data as u64) } as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }

    fn execute_fmul_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = unsafe { f64_mul(rs1_data as u64, rs2_data as u64) } as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }

    fn execute_fmv_x_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        self.write_reg(rd, rs1_data);
    }

    fn execute_fmadd_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        let rs3_data = self.read_freg64(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = unsafe { f64_add(f64_mul(rs1_data as u64, rs2_data as u64), rs3_data as u64) } as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }
    fn execute_fmsub_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        let rs3_data = self.read_freg64(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = unsafe { f64_sub(f64_mul(rs1_data as u64, rs2_data as u64), rs3_data as u64) } as Xlen64T;
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }
    fn execute_fnmsub_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        let rs3_data = self.read_freg64(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = self.neg_op_64(unsafe { f64_sub(f64_mul(rs1_data as u64, rs2_data as u64), rs3_data as u64) } as Xlen64T);
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }
    fn execute_fnmadd_d (&mut self, inst: InstT) {
        let rs1 = Self::get_rs1_addr(inst);
        let rs2 = Self::get_rs2_addr(inst);
        let rs3 = Self::get_rs3_addr(inst);
        let rd = Self::get_rd_addr(inst);

        let rs1_data = self.read_freg64(rs1);
        let rs2_data = self.read_freg64(rs2);
        let rs3_data = self.read_freg64(rs3);
        unsafe { softfloat_exceptionFlags = 0; }

        let reg_data: Xlen64T = self.neg_op_64(unsafe { f64_add(f64_mul(rs1_data as u64, rs2_data as u64), rs3_data as u64) } as Xlen64T);
        self.m_csr.csrrw(CsrAddr::FFlags, unsafe { softfloat_exceptionFlags as i64 });
        self.write_freg64(rd, reg_data);
    }
}
