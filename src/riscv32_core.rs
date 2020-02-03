use crate::riscv_csr::CsrAddr;
use crate::riscv_csr::RiscvCsr;

use crate::riscv_csr_bitdef::SYSREG_SATP_MODE_LSB;
use crate::riscv_csr_bitdef::SYSREG_SATP_MODE_MSB;

use crate::riscv_tracer::TraceInfo;
use crate::riscv_tracer::TraceType;
use crate::riscv_tracer::Tracer;

use crate::riscv_mmu::Riscv32Mmu;

pub type XlenT = i32;
pub type UXlenT = u32;
pub type InstT = u32;
pub type AddrT = u32;
pub type RegAddrT = u8;

use crate::riscv64_core::Addr64T;
use crate::riscv64_core::Xlen64T;

pub const DRAM_BASE: AddrT = 0x8000_0000;
pub const DRAM_SIZE: usize = 0x10_0000;

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
#[allow(dead_code)]
pub enum MemResult {
    NoExcept = 0,
    MisAlign = 1 << 0,
    NotDefined = 1 << 1,
    NewRegion = 1 << 2,
    TlbError = 1 << 3,
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum VMMode {
    Mbare = 0,
    Sv32 = 1,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}
