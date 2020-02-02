use crate::riscv32_core::Riscv32Core;
use crate::riscv32_core::Riscv32Env;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv32_core::PrivMode;

use crate::riscv_csr::CsrAddr;

use crate::riscv32_core::AddrT;
use crate::riscv32_core::XlenT;
use crate::riscv64_core::Xlen64T;

use crate::riscv_csr_bitdef::SYSREG_SSTATUS_SIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_SSTATUS_SIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_SSTATUS_SPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_SSTATUS_SPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_SSTATUS_SPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_SSTATUS_SPP_MSB;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPIE_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPIE_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPP_MSB;

#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(dead_code)]
pub enum ExceptCode {
    InstAddrMisalign = 0,
    InstAccessFault = 1,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisalign = 4,
    LoadAccessFault = 5,
    StoreAddrMisalign = 6,
    StoreAccessFault = 7,
    EcallFromUMode = 8,
    EcallFromSMode = 9,
    EcallFromHMode = 10,
    EcallFromMMode = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}

pub trait RiscvException<W> {
    fn generate_exception(&mut self, code: ExceptCode, tval: W);
}

impl RiscvException<XlenT> for Riscv32Env {
    fn generate_exception(&mut self, code: ExceptCode, tval: XlenT) {
        // FlushTlb();

        // if code != ExceptCode::IllegalInst && code != ExceptCode::EcallFromSMode {
        //     let paddr: AddrT = self.convert_virtual_address(self.m_pc, MemAccType::Read);
        //     // println!(
        //     //     "<Info: generate_exception Code={:d}, TVAL={:016x} PC={:016x},{:016x}>",
        //     //     code,
        //     //     tval,
        //     //     self.m_pc,
        //     //     paddr
        //     // );
        // }

        println!(
            "<Info: generate_exception Code={}, TVAL={:016x} PC={:016x}>",
            code as u32, tval, self.m_pc
        );

        let epc: AddrT;
        if code == ExceptCode::InstAddrMisalign {
            epc = self.get_previous_pc();
        } else {
            epc = self.get_pc();
        }

        let curr_priv: PrivMode = self.m_priv;

        let mut mstatus: XlenT;
        let mut sstatus: XlenT;
        let tvec: XlenT;
        let medeleg = self.m_csr.csrrs(CsrAddr::Medeleg, 0);
        let mut next_priv: PrivMode = PrivMode::Machine;

        self.set_priv_mode(next_priv);

        if (medeleg & (1 << (code as u32))) != 0 {
            // Delegation
            self.m_csr.csrrw(CsrAddr::Sepc, epc as XlenT);
            self.m_csr.csrrw(CsrAddr::Scause, code as XlenT);
            self.m_csr.csrrw(CsrAddr::Stval, tval);

            tvec = self.m_csr.csrrs(CsrAddr::Stvec, 0);
            next_priv = PrivMode::Supervisor;
        } else {
            self.m_csr.csrrw(CsrAddr::Mepc, epc as XlenT);
            self.m_csr.csrrw(CsrAddr::Mcause, code as XlenT);
            self.m_csr.csrrw(CsrAddr::Mtval, tval);

            tvec = self.m_csr.csrrs(CsrAddr::Mtvec, 0);
        }

        // Update status CSR
        if (medeleg & (1 << (code as u32))) != 0 {
            // Delegation
            sstatus = self.m_csr.csrrs(CsrAddr::Sstatus, 0);
            sstatus = Self::set_bit_field(
                sstatus,
                Self::extract_bit_field(sstatus, SYSREG_SSTATUS_SIE_MSB, SYSREG_SSTATUS_SIE_LSB),
                SYSREG_SSTATUS_SPIE_MSB,
                SYSREG_SSTATUS_SPIE_LSB,
            );
            sstatus = Self::set_bit_field(
                sstatus,
                curr_priv as XlenT,
                SYSREG_SSTATUS_SPP_MSB,
                SYSREG_SSTATUS_SPP_LSB,
            );
            sstatus =
                Self::set_bit_field(sstatus, 0, SYSREG_SSTATUS_SIE_MSB, SYSREG_SSTATUS_SIE_LSB);
            self.m_csr.csrrw(CsrAddr::Sstatus, sstatus);
        } else {
            mstatus = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
            mstatus = Self::set_bit_field(
                mstatus,
                Self::extract_bit_field(mstatus, SYSREG_MSTATUS_MIE_MSB, SYSREG_MSTATUS_MIE_LSB),
                SYSREG_MSTATUS_MPIE_MSB,
                SYSREG_MSTATUS_MPIE_LSB,
            );
            mstatus = Self::set_bit_field(
                mstatus,
                curr_priv as XlenT,
                SYSREG_MSTATUS_MPP_MSB,
                SYSREG_MSTATUS_MPP_LSB,
            );
            mstatus =
                Self::set_bit_field(mstatus, 0, SYSREG_MSTATUS_MIE_MSB, SYSREG_MSTATUS_MIE_LSB);

            self.m_csr.csrrw(CsrAddr::Mstatus, mstatus);
        }

        // if m_bit_mode == RiscvBitMode::Bit32 {
        // tvec = tvec & 0xffffffff;
        // }

        self.set_priv_mode(next_priv);

        self.set_pc(tvec as AddrT);
        self.set_update_pc(true);

        println!(
            "<Info: Exception. ChangeMode from {} to {}>",
            curr_priv as u32, next_priv as u32
        );
        println!("<Info: Set Program Counter = 0x{:16x}>", tvec);

        // Relesae Load Reservation
        // ClearLoadReservation();

        // // CountUp Timer
        // m_riscv_clint->Increment(INTERLEAVE / INSNS_PER_RTC_TICK);
        // m_count_timer = 0;

        return;
    }
}

impl RiscvException<Xlen64T> for Riscv64Env {
    fn generate_exception(&mut self, code: ExceptCode, tval: Xlen64T) {
        // FlushTlb();

        // if code != ExceptCode::IllegalInst && code != ExceptCode::EcallFromSMode {
        //     let paddr: AddrT = self.convert_virtual_address(self.m_pc, MemAccType::Read);
        //     // println!(
        //     //     "<Info: generate_exception Code={:d}, TVAL={:016x} PC={:016x},{:016x}>",
        //     //     code,
        //     //     tval,
        //     //     self.m_pc,
        //     //     paddr
        //     // );
        // }

        println!(
            "<Info: generate_exception Code={}, TVAL={:016x} PC={:016x}>",
            code as u32, tval, self.m_pc
        );

        let epc: AddrT;
        if code == ExceptCode::InstAddrMisalign {
            epc = self.get_previous_pc();
        } else {
            epc = self.get_pc();
        }

        let curr_priv: PrivMode = self.m_priv;

        let mut mstatus: Xlen64T;
        let mut sstatus: Xlen64T;
        let tvec: Xlen64T;
        let medeleg = self.m_csr.csrrs(CsrAddr::Medeleg, 0);
        let mut next_priv: PrivMode = PrivMode::Machine;

        self.set_priv_mode(next_priv);

        if (medeleg & (1 << (code as u32))) != 0 {
            // Delegation
            self.m_csr.csrrw(CsrAddr::Sepc, epc as Xlen64T);
            self.m_csr.csrrw(CsrAddr::Scause, code as Xlen64T);
            self.m_csr.csrrw(CsrAddr::Stval, tval as Xlen64T);

            tvec = self.m_csr.csrrs(CsrAddr::Stvec, 0 as Xlen64T);
            next_priv = PrivMode::Supervisor;
        } else {
            self.m_csr.csrrw(CsrAddr::Mepc, epc as Xlen64T);
            self.m_csr.csrrw(CsrAddr::Mcause, code as Xlen64T);
            self.m_csr.csrrw(CsrAddr::Mtval, tval as Xlen64T);

            tvec = self.m_csr.csrrs(CsrAddr::Mtvec, 0 as Xlen64T);
        }

        // Update status CSR
        if (medeleg & (1 << (code as u32))) != 0 {
            // Delegation
            sstatus = self.m_csr.csrrs(CsrAddr::Sstatus, 0 as Xlen64T);
            sstatus = Self::set_bit_field(
                sstatus,
                Self::extract_bit_field(sstatus, SYSREG_SSTATUS_SIE_MSB, SYSREG_SSTATUS_SIE_LSB),
                SYSREG_SSTATUS_SPIE_MSB,
                SYSREG_SSTATUS_SPIE_LSB,
            );
            sstatus = Self::set_bit_field(
                sstatus,
                curr_priv as Xlen64T,
                SYSREG_SSTATUS_SPP_MSB,
                SYSREG_SSTATUS_SPP_LSB,
            );
            sstatus =
                Self::set_bit_field(sstatus, 0, SYSREG_SSTATUS_SIE_MSB, SYSREG_SSTATUS_SIE_LSB);
            self.m_csr.csrrw(CsrAddr::Sstatus, sstatus as Xlen64T);
        } else {
            mstatus = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
            mstatus = Self::set_bit_field(
                mstatus,
                Self::extract_bit_field(mstatus, SYSREG_MSTATUS_MIE_MSB, SYSREG_MSTATUS_MIE_LSB),
                SYSREG_MSTATUS_MPIE_MSB,
                SYSREG_MSTATUS_MPIE_LSB,
            );
            mstatus = Self::set_bit_field(
                mstatus,
                curr_priv as Xlen64T,
                SYSREG_MSTATUS_MPP_MSB,
                SYSREG_MSTATUS_MPP_LSB,
            );
            mstatus =
                Self::set_bit_field(mstatus, 0, SYSREG_MSTATUS_MIE_MSB, SYSREG_MSTATUS_MIE_LSB);

            self.m_csr.csrrw(CsrAddr::Mstatus, mstatus);
        }

        // if m_bit_mode == RiscvBitMode::Bit32 {
        // tvec = tvec & 0xffffffff;
        // }

        self.set_priv_mode(next_priv);

        self.set_pc(tvec as AddrT);
        self.set_update_pc(true);

        println!(
            "<Info: Exception. ChangeMode from {} to {}>",
            curr_priv as u32, next_priv as u32
        );
        println!("<Info: Set Program Counter = 0x{:16x}>", tvec);

        // Relesae Load Reservation
        // ClearLoadReservation();

        // // CountUp Timer
        // m_riscv_clint->Increment(INTERLEAVE / INSNS_PER_RTC_TICK);
        // m_count_timer = 0;

        return;
    }
}
