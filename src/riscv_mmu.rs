use num::iter::range;

use crate::riscv_core::EnvBase;
use crate::riscv_core::Riscv64Core;

use crate::riscv_core::AddrType;
use crate::riscv_core::XlenType;

use crate::riscv_core::MemAccType;
use crate::riscv_core::MemResult;
use crate::riscv_exception::ExceptCode;

use crate::riscv_core::PrivMode;
use crate::riscv_core::VMMode;

use crate::riscv_csr::CsrAddr;
use crate::riscv_csr::Riscv64Csr;

use crate::riscv_exception::RiscvException;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPP_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPRV_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPRV_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MXR_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MXR_MSB;

use crate::riscv_csr_bitdef::SYSREG_SATP_PPN_LSB;
use crate::riscv_csr_bitdef::SYSREG_SATP_PPN_MSB;

pub trait RiscvMmu {
    fn convert_virtual_address(
        &mut self,
        vaddr: AddrType,
        acc_type: MemAccType,
    ) -> (MemResult, AddrType);

    fn walk_page_table(
        &mut self,
        vaddr: AddrType,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u8>,
        pte_len: Vec<u8>,
        pte_idx: Vec<u8>,
        vpn_len: Vec<u8>,
        vpn_idx: Vec<u8>,
        PAGESIZE: u32,
        PTESIZE: u32,
    ) -> (MemResult, AddrType);

    fn is_allowed_access(&mut self, i_type: u8, acc_type: MemAccType, priv_mode: PrivMode) -> bool;
}

impl RiscvMmu for EnvBase {
    fn walk_page_table(
        &mut self,
        vaddr: AddrType,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u8>,
        pte_len: Vec<u8>,
        pte_idx: Vec<u8>,
        vpn_len: Vec<u8>,
        vpn_idx: Vec<u8>,
        PAGESIZE: u32,
        PTESIZE: u32,
    ) -> (MemResult, AddrType) {
        let is_write_access = match acc_type {
            MemAccType::Write => true,
            _ => false,
        };

        //===================
        // Simple TLB Search
        //===================
        // let vaddr_vpn: AddrType = (vaddr >> 12);
        // let vaddr_tag: u8 = vaddr_vpn & (tlb_width-1);
        // if (m_tlb_en[vaddr_tag] && m_tlb_tag[vaddr_tag] == vaddr_vpn) {
        //     let paddr:AddrType = (m_tlb_addr[vaddr_tag] & !0x0fff) + (vaddr & 0x0fff);
        //     let pte_val:u64 = m_tlb_addr[vaddr_tag] & 0x0ff;
        //
        //     if (!is_allowed_access ((pte_val >> 1) & 0x0f, acc_type, self.m_priv)) {
        //         println! ("<Page Access Failed. Allowed Access Failed PTE_VAL=%016lx>", pte_val);
        //         return (MemResult::TlbError, paddr);
        //     }
        //     if (((pte_val & 0x40) == 0) || // PTE.A
        //         ((acc_type == MemAccType::Write) && (pte_val & 0x80) == 0)) { // PTE.D
        //         println!("<Access Fault : Page Permission Fault {:01x}>", (pte_val >> 1) & 0x0f);
        //         if (acc_type == MemAccType::Fetch) {
        //             generate_exception (self, ExceptCode::InstPageFault, vaddr as XlenType);
        //         }
        //         return (MemResult::TlbError, paddr);
        //     }
        //     return (MemResult::NoExcept, paddr);
        // }

        let satp = self.m_csr.csrrs(CsrAddr::Satp, 0);
        let pte_base: AddrType =
            Self::extract_bit_field(satp, SYSREG_SATP_PPN_MSB, SYSREG_SATP_PPN_LSB) as AddrType;

        let mut pte_val: XlenType = 0;
        let mut pte_addr: AddrType = (pte_base * PAGESIZE) as AddrType;
        let level: usize = 0;

        for level in range(0, init_level).rev() {
            let va_vpn_i: AddrType =
                (vaddr >> vpn_idx[level as usize]) & ((1 << vpn_len[level as usize]) - 1);
            pte_addr += (va_vpn_i * PTESIZE) as AddrType;
            pte_val = self.read_memory_word(pte_addr);

            // println!(
            //     "<Info: VAddr = 0x{:08x} PTEAddr = 0x{:08x} : PPTE = 0x{:08x}>",
            //     vaddr, pte_addr, pte_val
            // );

            // 3. If pte:v = 0, or if pte:r = 0 and pte:w = 1, stop and raise a page-fault exception.
            if (pte_val & 0x01) == 0 || (((pte_val & 0x02) == 0) && ((pte_val & 0x04) == 0x04)) {
                // let bit_length: u32 = m_bit_mode == RiscvBitMode_t::Bit32 ? 8 : 16;
                println!("<Page Table Error : 0x{:016x} = 0x{:08x} is not valid Page Table. Generate Exception>",
                         pte_addr, pte_val);

                match acc_type {
                    MemAccType::Fetch => {
                        self.generate_exception(ExceptCode::InstPageFault, vaddr as XlenType);
                    }
                    MemAccType::Read => {
                        self.generate_exception(ExceptCode::LoadPageFault, vaddr as XlenType);
                    }
                    MemAccType::Write => {
                        self.generate_exception(ExceptCode::StorePageFault, vaddr as XlenType);
                    }
                };
                return (MemResult::TlbError, 0);
            }

            // If pte:r = 1 or pte:x = 1, go to step 5. Otherwise, this PTE is a
            // pointer to the next level of the page table. Let i = i − 1. If i < 0, stop and raise a page-fault
            // exception. Otherwise, let a = pte:ppn × PAGESIZE and go to step 2.
            if ((pte_val & 0x08) == 0x08) || ((pte_val & 0x02) == 0x02) {
                break;
            } else {
                if level == 0 {
                    println!(
                        "<Access Fault : Tried to Access to Page {:01x}>",
                        ((pte_val >> 1) & 0x0f)
                    );

                    match acc_type {
                        MemAccType::Fetch => {
                            self.generate_exception(ExceptCode::InstPageFault, vaddr as XlenType);
                        }
                        MemAccType::Read => {
                            self.generate_exception(ExceptCode::LoadPageFault, vaddr as XlenType);
                        }
                        MemAccType::Write => {
                            self.generate_exception(ExceptCode::StorePageFault, vaddr as XlenType);
                        }
                    };
                    return (MemResult::TlbError, 0);
                }
            }
            let pte_ppn: AddrType = Self::extract_bit_field(
                pte_val as XlenType,
                pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
                pte_idx[0],
            ) as AddrType;
            pte_addr = pte_ppn * PAGESIZE;
        }

        let current_priv: PrivMode = self.m_priv.clone();
        if !self.is_allowed_access(
            ((pte_val >> 1) & 0x0f) as u8,
            acc_type.clone(),
            current_priv,
        ) {
            println!(
                "<Page Access Failed. Allowed Access Failed PTE_VAL={:016x}>",
                pte_val,
            );
            return (MemResult::TlbError, 0);
        }

        if level != 0
            && Self::extract_bit_field(
                pte_val as XlenType,
                pte_len[level - 1] + pte_idx[level - 1] - 1,
                pte_idx[0],
            ) != 0
        {
            // 6. If i > 0 and pa:ppn[i−1:0] != 0, this is a misaligned superpage
            // stop and raise a page-fault exception.
            // println! ("<Page Access Failed. Last PTE != 0>");
            return (MemResult::TlbError, 0);
        }

        if ((pte_val & 0x40) == 0) || // PTE.A
            (is_write_access && (pte_val & 0x80) == 0)
        {
            // PTE.D
            println!(
                "<Access Fault : Page Permission Fault {:01x}",
                ((pte_val >> 1) & 0x0f)
            );

            match acc_type {
                MemAccType::Fetch => {
                    self.generate_exception(ExceptCode::InstPageFault, vaddr as XlenType);
                }
                MemAccType::Read => {
                    self.generate_exception(ExceptCode::LoadPageFault, vaddr as XlenType);
                }
                MemAccType::Write => {
                    self.generate_exception(ExceptCode::StorePageFault, vaddr as XlenType);
                }
            };
            return (MemResult::TlbError, 0);
        }

        let mut phy_addr: AddrType = (Self::extract_bit_field(
            pte_val as XlenType,
            pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
            pte_idx[level],
        ) << ppn_idx[level]) as AddrType;

        // println!("Level = {}", level);

        for l in 0..(level + 1) {
            let vaddr_vpn: AddrType = Self::extract_bit_field(
                vaddr as XlenType,
                vpn_len[level - l as usize] + vpn_idx[level - l as usize] - 1,
                vpn_idx[level - l as usize],
            ) as AddrType;
            phy_addr |= vaddr_vpn << ppn_idx[level as usize];
        }

        // Finally Add Page Offset
        phy_addr |= Self::extract_bit_field(vaddr as XlenType, vpn_idx[0] - 1, 0) as AddrType;

        //==========================
        // Update Simple TLB Search
        //==========================
        // println!(
        //     "<Info: TLB[{:d}] <= 0x{:016x}(0x{:016x})>",
        //     vaddr as XlenType_tag,
        //     vaddr as XlenType_vpn,
        //     *paddr & !0x0fff
        // );
        // m_tlb_en  [vaddr_tag] = true;
        // m_tlb_tag [vaddr_tag] = vaddr_vpn;
        // m_tlb_addr[vaddr_tag] = (*paddr & !0x0fff) | (pte_val & 0x0ff);

        // println!("<Converted Virtual Address = {:08x}>", phy_addr);
        return (MemResult::NoExcept, phy_addr);
    }

    fn convert_virtual_address(
        &mut self,
        vaddr: AddrType,
        acc_type: MemAccType,
    ) -> (MemResult, AddrType) {
        let is_fetch_access = match acc_type {
            MemAccType::Fetch => true,
            _ => false,
        };

        let mstatus: XlenType = self
            .m_csr
            .csrrs(CsrAddr::Mstatus, PrivMode::Machine as XlenType);
        let mprv: u8 =
            Self::extract_bit_field(mstatus, SYSREG_MSTATUS_MPRV_MSB, SYSREG_MSTATUS_MPRV_LSB)
                as u8;
        let mpp_u8: u8 =
            Self::extract_bit_field(mstatus, SYSREG_MSTATUS_MPP_MSB, SYSREG_MSTATUS_MPP_LSB) as u8;
        let mpp: PrivMode = PrivMode::from_u8(mpp_u8);

        let priv_mode: PrivMode = if !is_fetch_access && (mprv != 0) {
            mpp
        } else {
            self.m_priv
        };

        // println!("<Convert Virtual Addres : vm_mode = {}, priv_mode = {}>",
        //          self.get_vm_mode() as u32, priv_mode as u32);

        if self.get_vm_mode() == VMMode::Sv39
            && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
        {
            let ppn_idx: Vec<u8> = vec![12, 21, 30];
            let pte_len: Vec<u8> = vec![9, 9, 26];
            let pte_idx: Vec<u8> = vec![10, 19, 28];
            let vpn_len: Vec<u8> = vec![9, 9, 9];
            let vpn_idx: Vec<u8> = vec![12, 21, 30];
            let PAGESIZE: u32 = num::pow(2, 12);
            let PTESIZE: u32 = 8;

            return self.walk_page_table(
                vaddr, acc_type, 3, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, PAGESIZE, PTESIZE,
            );
        } else if self.get_vm_mode() == VMMode::Sv32
            && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
        {
            let ppn_idx: Vec<u8> = vec![12, 22];
            let pte_len: Vec<u8> = vec![10, 12];
            let pte_idx: Vec<u8> = vec![10, 20];
            let vpn_len: Vec<u8> = vec![10, 10];
            let vpn_idx: Vec<u8> = vec![12, 22];
            let PAGESIZE: u32 = num::pow(2, 12);
            let PTESIZE: u32 = 4;

            return self.walk_page_table(
                vaddr, acc_type, 2, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, PAGESIZE, PTESIZE,
            );
        } else {
            return (MemResult::NoExcept, vaddr);
        }
    }

    fn is_allowed_access(&mut self, i_type: u8, acc_type: MemAccType, priv_mode: PrivMode) -> bool {
        let is_user_mode = match priv_mode {
            PrivMode::User => true,
            _ => false,
        };
        if is_user_mode && !((i_type & 0x08) != 0) {
            return false;
        }
        let allowed_access = match acc_type {
            MemAccType::Fetch => (i_type & 0x04) != 0,
            MemAccType::Write => ((i_type & 0x01) != 0) && ((i_type & 0x02) != 0),
            MemAccType::Read => {
                let mstatus: XlenType = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
                let mxr: u8 = Self::extract_bit_field(
                    mstatus,
                    SYSREG_MSTATUS_MXR_MSB,
                    SYSREG_MSTATUS_MXR_LSB,
                ) as u8;
                ((i_type & 0x01) != 0) | ((mxr & (i_type & 0x04)) != 0)
            }
        };
        return allowed_access;
    }
}
