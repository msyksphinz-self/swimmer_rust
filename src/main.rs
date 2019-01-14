mod core_base;
mod riscv_core;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::EnvBase;

fn main() {

    let mut riscv64_core = EnvBase::new();
    for addr in 0..32 {
        riscv64_core.WriteMemory (addr*4+1, addr as u32);
    }
    for addr in 0..32 {
        let data = riscv64_core.ReadMemory (addr*4+1);
        println!("Read Data = {:08x}", data);
    }
    for addr in 0..32 {
        let mut ref_data:u32;
        let data = riscv64_core.FetchMemory (addr*4+1, &mut ref_data);
        println!("Fetch Data = {:08x}", data);
    }
}
