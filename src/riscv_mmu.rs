use num::iter::range;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv32_core::AddrT;
use crate::riscv32_core::XlenT;
use crate::riscv64_core::Addr64T;
use crate::riscv64_core::Xlen64T;

use crate::riscv32_core::MemAccType;
use crate::riscv32_core::MemResult;
use crate::riscv_exception::ExceptCode;

use crate::riscv32_core::PrivMode;
use crate::riscv32_core::VMMode;

use crate::riscv_csr::CsrAddr;

use crate::riscv_exception::RiscvException;

use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPP_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPP_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPRV_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MPRV_MSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MXR_LSB;
use crate::riscv_csr_bitdef::SYSREG_MSTATUS_MXR_MSB;

use crate::riscv_csr_bitdef::SYSREG_SATP_PPN_LSB;
use crate::riscv_csr_bitdef::SYSREG_SATP_PPN_MSB;

pub trait Riscv32Mmu {
    fn convert_virtual_address(&mut self, vaddr: AddrT, acc_type: MemAccType)
        -> Result<AddrT, MemResult>;

    fn walk_page_table(
        &mut self,
        vaddr: AddrT,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u8>,
        pte_len: Vec<u8>,
        pte_idx: Vec<u8>,
        vpn_len: Vec<u8>,
        vpn_idx: Vec<u8>,
        pagesize: u32,
        ptesize: u32,
    ) -> Result<AddrT, MemResult>;

    fn is_allowed_access(&mut self, i_type: u8, acc_type: MemAccType, priv_mode: PrivMode) -> bool;
}

// impl Riscv32Mmu for Riscv32Env {
//     fn walk_page_table(
//         &mut self,
//         vaddr: AddrT,
//         acc_type: MemAccType,
//         init_level: u32,
//         ppn_idx: Vec<u8>,
//         pte_len: Vec<u8>,
//         pte_idx: Vec<u8>,
//         vpn_len: Vec<u8>,
//         vpn_idx: Vec<u8>,
//         pagesize: u32,
//         ptesize: u32,
//     ) -> Result<AddrT, MemResult> {
//         let is_write_access = match acc_type {
//             MemAccType::Write => true,
//             _ => false,
//         };
//
//         //===================
//         // Simple TLB Search
//         //===================
//         // let vaddr_vpn: AddrT = (vaddr >> 12);
//         // let vaddr_tag: u8 = vaddr_vpn & (tlb_width-1);
//         // if (m_tlb_en[vaddr_tag] && m_tlb_tag[vaddr_tag] == vaddr_vpn) {
//         //     let paddr:AddrT = (m_tlb_addr[vaddr_tag] & !0x0fff) + (vaddr & 0x0fff);
//         //     let pte_val:u64 = m_tlb_addr[vaddr_tag] & 0x0ff;
//         //
//         //     if (!is_allowed_access ((pte_val >> 1) & 0x0f, acc_type, self.m_priv)) {
//         //         println! ("<Page Access Failed. Allowed Access Failed PTE_VAL=%016lx>", pte_val);
//         //         return Err(MemResult::TlbError);
//         //     }
//         //     if (((pte_val & 0x40) == 0) || // PTE.A
//         //         ((acc_type == MemAccType::Write) && (pte_val & 0x80) == 0)) { // PTE.D
//         //         println!("<Access Fault : Page Permission Fault {:01x}>", (pte_val >> 1) & 0x0f);
//         //         if (acc_type == MemAccType::Fetch) {
//         //             generate_exception (self, ExceptCode::InstPageFault, vaddr as XlenT);
//         //         }
//         //         return Err(MemResult::TlbError);
//         //     }
//         //     return Ok(paddr);
//         // }
//
//         let satp = self.m_csr.csrrs(CsrAddr::Satp, 0);
//         let pte_base: AddrT =
//             Self::extract_bit_field(satp, SYSREG_SATP_PPN_MSB, SYSREG_SATP_PPN_LSB) as AddrT;
//
//         let mut pte_val: XlenT = 0;
//         let mut pte_addr: AddrT = (pte_base * pagesize) as AddrT;
//         let level: usize = 0;
//
//         for level in range(0, init_level).rev() {
//             let va_vpn_i: AddrT =
//                 (vaddr >> vpn_idx[level as usize]) & ((1 << vpn_len[level as usize]) - 1);
//             pte_addr += (va_vpn_i * ptesize) as AddrT;
//             pte_val = self.read_memory_word(pte_addr);
//
//             // println!(
//             //     "<Info: VAddr = 0x{:08x} PTEAddr = 0x{:08x} : PPTE = 0x{:08x}>",
//             //     vaddr, pte_addr, pte_val
//             // );
//
//             // 3. If pte:v = 0, or if pte:r = 0 and pte:w = 1, stop and raise a page-fault exception.
//             if (pte_val & 0x01) == 0 || (((pte_val & 0x02) == 0) && ((pte_val & 0x04) == 0x04)) {
//                 // let bit_length: u32 = m_bit_mode == RiscvBitMode_t::Bit32 ? 8 : 16;
//                 println!("<Page Table Error : 0x{:016x} = 0x{:08x} is not valid Page Table. Generate Exception>",
//                          pte_addr, pte_val);
//
//                 match acc_type {
//                     MemAccType::Fetch => {
//                         self.generate_exception(ExceptCode::InstPageFault, vaddr as XlenT);
//                     }
//                     MemAccType::Read => {
//                         self.generate_exception(ExceptCode::LoadPageFault, vaddr as XlenT);
//                     }
//                     MemAccType::Write => {
//                         self.generate_exception(ExceptCode::StorePageFault, vaddr as XlenT);
//                     }
//                 };
//                 return Err(MemResult::TlbError);
//             }
//
//             // If pte:r = 1 or pte:x = 1, go to step 5. Otherwise, this PTE is a
//             // pointer to the next level of the page table. Let i = i − 1. If i < 0, stop and raise a page-fault
//             // exception. Otherwise, let a = pte:ppn × pagesize and go to step 2.
//             if ((pte_val & 0x08) == 0x08) || ((pte_val & 0x02) == 0x02) {
//                 break;
//             } else {
//                 if level == 0 {
//                     println!(
//                         "<Access Fault : Tried to Access to Page {:01x}>",
//                         ((pte_val >> 1) & 0x0f)
//                     );
//
//                     match acc_type {
//                         MemAccType::Fetch => {
//                             self.generate_exception(ExceptCode::InstPageFault, vaddr as XlenT);
//                         }
//                         MemAccType::Read => {
//                             self.generate_exception(ExceptCode::LoadPageFault, vaddr as XlenT);
//                         }
//                         MemAccType::Write => {
//                             self.generate_exception(ExceptCode::StorePageFault, vaddr as XlenT);
//                         }
//                     };
//                     return Err(MemResult::TlbError);
//                 }
//             }
//             let pte_ppn: AddrT = Self::extract_bit_field(
//                 pte_val as XlenT,
//                 pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
//                 pte_idx[0],
//             ) as AddrT;
//             pte_addr = pte_ppn * pagesize;
//         }
//
//         let current_priv: PrivMode = self.m_priv.clone();
//         if !self.is_allowed_access(
//             ((pte_val >> 1) & 0x0f) as u8,
//             acc_type.clone(),
//             current_priv,
//         ) {
//             println!(
//                 "<Page Access Failed. Allowed Access Failed PTE_VAL={:016x}>",
//                 pte_val,
//             );
//             return Err(MemResult::TlbError);
//         }
//
//         if level != 0
//             && Self::extract_bit_field(
//                 pte_val as XlenT,
//                 pte_len[level - 1] + pte_idx[level - 1] - 1,
//                 pte_idx[0],
//             ) != 0
//         {
//             // 6. If i > 0 and pa:ppn[i−1:0] != 0, this is a misaligned superpage
//             // stop and raise a page-fault exception.
//             println! ("<Page Access Failed. Last PTE != 0>");
//             return Err(MemResult::TlbError);
//         }
//
//         if ((pte_val & 0x40) == 0) || // PTE.A
//             (is_write_access && (pte_val & 0x80) == 0)
//         {
//             // PTE.D
//             println!(
//                 "<Access Fault : Page Permission Fault {:01x}",
//                 ((pte_val >> 1) & 0x0f)
//             );
//
//             match acc_type {
//                 MemAccType::Fetch => {
//                     self.generate_exception(ExceptCode::InstPageFault, vaddr as XlenT);
//                 }
//                 MemAccType::Read => {
//                     self.generate_exception(ExceptCode::LoadPageFault, vaddr as XlenT);
//                 }
//                 MemAccType::Write => {
//                     self.generate_exception(ExceptCode::StorePageFault, vaddr as XlenT);
//                 }
//             };
//             return Err(MemResult::TlbError);
//         }
//
//         let mut phy_addr: AddrT = (Self::extract_bit_field(
//             pte_val as XlenT,
//             pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
//             pte_idx[level],
//         ) << ppn_idx[level]) as AddrT;
//
//         // println!("Level = {}", level);
//
//         for l in 0..(level + 1) {
//             let vaddr_vpn: AddrT = Self::extract_bit_field(
//                 vaddr as XlenT,
//                 vpn_len[level - l as usize] + vpn_idx[level - l as usize] - 1,
//                 vpn_idx[level - l as usize],
//             ) as AddrT;
//             phy_addr |= vaddr_vpn << ppn_idx[level as usize];
//         }
//
//         // Finally Add Page Offset
//         phy_addr |= Self::extract_bit_field(vaddr as XlenT, vpn_idx[0] - 1, 0) as AddrT;
//
//         //==========================
//         // Update Simple TLB Search
//         //==========================
//         // println!(
//         //     "<Info: TLB[{:d}] <= 0x{:016x}(0x{:016x})>",
//         //     vaddr as XlenT_tag,
//         //     vaddr as XlenT_vpn,
//         //     *paddr & !0x0fff
//         // );
//         // m_tlb_en  [vaddr_tag] = true;
//         // m_tlb_tag [vaddr_tag] = vaddr_vpn;
//         // m_tlb_addr[vaddr_tag] = (*paddr & !0x0fff) | (pte_val & 0x0ff);
//
//         println!("<Converted Virtual Address = {:08x}>", phy_addr);
//         return Ok(phy_addr);
//     }
//
//     fn convert_virtual_address(
//         &mut self,
//         vaddr: AddrT,
//         acc_type: MemAccType,
//     ) -> Result<AddrT, MemResult> {
//         let is_fetch_access = match acc_type {
//             MemAccType::Fetch => true,
//             _ => false,
//         };
//
//         let mstatus: XlenT = self
//             .m_csr
//             .csrrs(CsrAddr::Mstatus, PrivMode::Machine as XlenT);
//         let mprv: u8 =
//             Self::extract_bit_field(mstatus, SYSREG_MSTATUS_MPRV_MSB, SYSREG_MSTATUS_MPRV_LSB)
//                 as u8;
//         let mpp_u8: u8 =
//             Self::extract_bit_field(mstatus, SYSREG_MSTATUS_MPP_MSB, SYSREG_MSTATUS_MPP_LSB) as u8;
//         let mpp: PrivMode = PrivMode::from_u8(mpp_u8);
//
//         let priv_mode: PrivMode = if !is_fetch_access && (mprv != 0) {
//             mpp
//         } else {
//             self.m_priv
//         };
//
//         // println!("<Convert Virtual Addres : vm_mode = {}, priv_mode = {}>",
//         //          self.get_vm_mode() as u32, priv_mode as u32);
//
//         if self.get_vm_mode() == VMMode::Sv39
//             && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
//         {
//             let ppn_idx: Vec<u8> = vec![12, 21, 30];
//             let pte_len: Vec<u8> = vec![9, 9, 26];
//             let pte_idx: Vec<u8> = vec![10, 19, 28];
//             let vpn_len: Vec<u8> = vec![9, 9, 9];
//             let vpn_idx: Vec<u8> = vec![12, 21, 30];
//             let pagesize: u32 = 4096; // num::pow(2, 12);
//             let ptesize: u32 = 8;
//
//             return self.walk_page_table(
//                 vaddr, acc_type, 3, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, pagesize, ptesize,
//             );
//         } else if self.get_vm_mode() == VMMode::Sv32
//             && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
//         {
//             let ppn_idx: Vec<u8> = vec![12, 22];
//             let pte_len: Vec<u8> = vec![10, 12];
//             let pte_idx: Vec<u8> = vec![10, 20];
//             let vpn_len: Vec<u8> = vec![10, 10];
//             let vpn_idx: Vec<u8> = vec![12, 22];
//             let pagesize: u32 = 4096; // num::pow(2, 12);
//             let ptesize: u32 = 4;
//
//             return self.walk_page_table(
//                 vaddr, acc_type, 2, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, pagesize, ptesize,
//             );
//         } else {
//             return Ok(vaddr);
//         }
//     }
//
//     fn is_allowed_access(&mut self, i_type: u8, acc_type: MemAccType, priv_mode: PrivMode) -> bool {
//         let is_user_mode = match priv_mode {
//             PrivMode::User => true,
//             _ => false,
//         };
//         if is_user_mode && !((i_type & 0x08) != 0) {
//             return false;
//         }
//         let allowed_access = match acc_type {
//             MemAccType::Fetch => (i_type & 0x04) != 0,
//             MemAccType::Write => ((i_type & 0x01) != 0) && ((i_type & 0x02) != 0),
//             MemAccType::Read => {
//                 let mstatus: XlenT = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
//                 let mxr: u8 = Self::extract_bit_field(
//                     mstatus,
//                     SYSREG_MSTATUS_MXR_MSB,
//                     SYSREG_MSTATUS_MXR_LSB,
//                 ) as u8;
//                 ((i_type & 0x01) != 0) | ((mxr & (i_type & 0x04)) != 0)
//             }
//         };
//         return allowed_access;
//     }
// }

pub trait Riscv64Mmu {
    fn convert_virtual_address(&mut self, vaddr: Addr64T, acc_type: MemAccType)
        -> Result<Addr64T, MemResult>;

    fn walk_page_table(
        &mut self,
        vaddr: Addr64T,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u8>,
        pte_len: Vec<u8>,
        pte_idx: Vec<u8>,
        vpn_len: Vec<u8>,
        vpn_idx: Vec<u8>,
        pagesize: u32,
        ptesize: u32,
    ) -> Result<Addr64T, MemResult>;

    fn is_allowed_access(&mut self, i_type: u8, acc_type: MemAccType, priv_mode: PrivMode) -> bool;
}


impl Riscv64Mmu for Riscv64Env {
    fn walk_page_table(
        &mut self,
        vaddr: Addr64T,
        acc_type: MemAccType,
        init_level: u32,
        ppn_idx: Vec<u8>,
        pte_len: Vec<u8>,
        pte_idx: Vec<u8>,
        vpn_len: Vec<u8>,
        vpn_idx: Vec<u8>,
        pagesize: u32,
        ptesize: u32,
    ) -> Result<Addr64T, MemResult> {
        let is_write_access = match acc_type {
            MemAccType::Write => true,
            _ => false,
        };

        //===================
        // Simple TLB Search
        //===================
        // let vaddr_vpn: Addr64T = (vaddr >> 12);
        // let vaddr_tag: u8 = vaddr_vpn & (tlb_width-1);
        // if (m_tlb_en[vaddr_tag] && m_tlb_tag[vaddr_tag] == vaddr_vpn) {
        //     let paddr:Addr64T = (m_tlb_addr[vaddr_tag] & !0x0fff) + (vaddr & 0x0fff);
        //     let pte_val:u64 = m_tlb_addr[vaddr_tag] & 0x0ff;
        //
        //     if (!is_allowed_access ((pte_val >> 1) & 0x0f, acc_type, self.m_priv)) {
        //         println! ("<Page Access Failed. Allowed Access Failed PTE_VAL=%016lx>", pte_val);
        //         return Err(MemResult::TlbError);
        //     }
        //     if (((pte_val & 0x40) == 0) || // PTE.A
        //         ((acc_type == MemAccType::Write) && (pte_val & 0x80) == 0)) { // PTE.D
        //         println!("<Access Fault : Page Permission Fault {:01x}>", (pte_val >> 1) & 0x0f);
        //         if (acc_type == MemAccType::Fetch) {
        //             generate_exception (self, ExceptCode::InstPageFault, vaddr as Xlen64T);
        //         }
        //         return Err(MemResult::TlbError);
        //     }
        //     return Ok(paddr);
        // }

        let satp = self.m_csr.csrrs(CsrAddr::Satp, 0) as Xlen64T;
        let pte_base: Addr64T =
            Self::extract_bit_field(satp, SYSREG_SATP_PPN_MSB, SYSREG_SATP_PPN_LSB) as Addr64T;

        let mut pte_val: Xlen64T = 0;
        let mut pte_addr: Addr64T = (pte_base * (pagesize as Addr64T)) as Addr64T;
        let level: usize = 0;

        for level in range(0, init_level).rev() {
            let va_vpn_i: Addr64T =
                (vaddr >> vpn_idx[level as usize]) & ((1 << vpn_len[level as usize]) - 1);
            pte_addr += (va_vpn_i * (ptesize as Addr64T)) as Addr64T;

            match self.read_memory_word(pte_addr) {
                Ok(pte_value) => { pte_val = pte_value as Xlen64T; }
                Err(result) => match result {
                    MemResult::NotDefined => { pte_val = 0; },
                    _ => { panic!("Illegal PTE Access Detectd."); },
                }
            }

            println!(
                "<Info: VAddr = 0x{:016x} PTEAddr = 0x{:016x} : PPTE = 0x{:08x}>",
                vaddr, pte_addr, pte_val
            );

            // 3. If pte:v = 0, or if pte:r = 0 and pte:w = 1, stop and raise a page-fault exception.
            if (pte_val & 0x01) == 0 || (((pte_val & 0x02) == 0) && ((pte_val & 0x04) == 0x04)) {
                // let bit_length: u32 = m_bit_mode == RiscvBitMode_t::Bit32 ? 8 : 16;
                println!("<Page Table Error : 0x{:016x} = 0x{:08x} is not valid Page Table. Generate Exception>",
                         pte_addr, pte_val);

                match acc_type {
                    MemAccType::Fetch => {
                        self.generate_exception(ExceptCode::InstPageFault, vaddr as Xlen64T);
                    }
                    MemAccType::Read => {
                        self.generate_exception(ExceptCode::LoadPageFault, vaddr as Xlen64T);
                    }
                    MemAccType::Write => {
                        self.generate_exception(ExceptCode::StorePageFault, vaddr as Xlen64T);
                    }
                };
                return Err(MemResult::TlbError);
            }

            // If pte:r = 1 or pte:x = 1, go to step 5. Otherwise, this PTE is a
            // pointer to the next level of the page table. Let i = i − 1. If i < 0, stop and raise a page-fault
            // exception. Otherwise, let a = pte:ppn × pagesize and go to step 2.
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
                            self.generate_exception(ExceptCode::InstPageFault, vaddr as Xlen64T);
                        }
                        MemAccType::Read => {
                            self.generate_exception(ExceptCode::LoadPageFault, vaddr as Xlen64T);
                        }
                        MemAccType::Write => {
                            self.generate_exception(ExceptCode::StorePageFault, vaddr as Xlen64T);
                        }
                    };
                    return Err(MemResult::TlbError);
                }
            }
            let pte_ppn: Addr64T = Self::extract_bit_field(
                pte_val as Xlen64T,
                pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
                pte_idx[0],
            ) as Addr64T;
            pte_addr = pte_ppn * (pagesize as Addr64T);
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
            return Err(MemResult::TlbError);
        }

        if level != 0
            && Self::extract_bit_field(
                pte_val as Xlen64T,
                pte_len[level - 1] + pte_idx[level - 1] - 1,
                pte_idx[0],
            ) != 0
        {
            // 6. If i > 0 and pa:ppn[i−1:0] != 0, this is a misaligned superpage
            // stop and raise a page-fault exception.
            // println! ("<Page Access Failed. Last PTE != 0>");
            return Err(MemResult::TlbError);
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
                    self.generate_exception(ExceptCode::InstPageFault, vaddr as Xlen64T);
                }
                MemAccType::Read => {
                    self.generate_exception(ExceptCode::LoadPageFault, vaddr as Xlen64T);
                }
                MemAccType::Write => {
                    self.generate_exception(ExceptCode::StorePageFault, vaddr as Xlen64T);
                }
            };
            return Err(MemResult::TlbError);
        }

        let mut phy_addr: Addr64T = (Self::extract_bit_field(
            pte_val as Xlen64T,
            pte_len[(init_level - 1) as usize] + pte_idx[(init_level - 1) as usize] - 1,
            pte_idx[level],
        ) << ppn_idx[level]) as Addr64T;

        // println!("Level = {}", level);

        for l in 0..(level + 1) {
            let vaddr_vpn: Addr64T = Self::extract_bit_field(
                vaddr as Xlen64T,
                vpn_len[level - l as usize] + vpn_idx[level - l as usize] - 1,
                vpn_idx[level - l as usize],
            ) as Addr64T;
            phy_addr |= vaddr_vpn << ppn_idx[level as usize];
        }

        // Finally Add Page Offset
        phy_addr |= Self::extract_bit_field(vaddr as Xlen64T, vpn_idx[0] - 1, 0) as Addr64T;

        //==========================
        // Update Simple TLB Search
        //==========================
        // println!(
        //     "<Info: TLB[{:d}] <= 0x{:016x}(0x{:016x})>",
        //     vaddr as Xlen64T_tag,
        //     vaddr as Xlen64T_vpn,
        //     *paddr & !0x0fff
        // );
        // m_tlb_en  [vaddr_tag] = true;
        // m_tlb_tag [vaddr_tag] = vaddr_vpn;
        // m_tlb_addr[vaddr_tag] = (*paddr & !0x0fff) | (pte_val & 0x0ff);

        println!("<Converted Virtual Address = {:08x}>", phy_addr);
        return Ok(phy_addr);
    }

    fn convert_virtual_address(
        &mut self,
        vaddr: Addr64T,
        acc_type: MemAccType,
    ) -> Result<Addr64T, MemResult> {
        let is_fetch_access = match acc_type {
            MemAccType::Fetch => true,
            _ => false,
        };

        let mstatus: Xlen64T = self
            .m_csr
            .csrrs(CsrAddr::Mstatus, PrivMode::Machine as Xlen64T);
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

        // println!("<Convert_Virtual_Address. vaddr={:016x} : vm_mode = {}, priv_mode = {}>",
        //          vaddr, self.get_vm_mode() as u32, priv_mode as u32);

        if self.get_vm_mode() == VMMode::Sv39
            && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
        {
            let ppn_idx: Vec<u8> = vec![12, 21, 30];
            let pte_len: Vec<u8> = vec![9, 9, 26];
            let pte_idx: Vec<u8> = vec![10, 19, 28];
            let vpn_len: Vec<u8> = vec![9, 9, 9];
            let vpn_idx: Vec<u8> = vec![12, 21, 30];
            let pagesize: u32 = 4096; // num::pow(2, 12);
            let ptesize: u32 = 8;

            return self.walk_page_table(
                vaddr, acc_type, 3, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, pagesize, ptesize,
            );
        } else if self.get_vm_mode() == VMMode::Sv32
            && (priv_mode == PrivMode::Supervisor || priv_mode == PrivMode::User)
        {
            let ppn_idx: Vec<u8> = vec![12, 22];
            let pte_len: Vec<u8> = vec![10, 12];
            let pte_idx: Vec<u8> = vec![10, 20];
            let vpn_len: Vec<u8> = vec![10, 10];
            let vpn_idx: Vec<u8> = vec![12, 22];
            let pagesize: u32 = 4096; // num::pow(2, 12);
            let ptesize: u32 = 4;

            return self.walk_page_table(
                vaddr, acc_type, 2, ppn_idx, pte_len, pte_idx, vpn_len, vpn_idx, pagesize, ptesize,
            );
        } else {
            return Ok(vaddr);
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
                let mstatus: Xlen64T = self.m_csr.csrrs(CsrAddr::Mstatus, 0);
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
