/* lib.rs */

use std::fs::File;
use std::io::{BufReader, Read};

mod core_base;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv32_insts::RiscvInsts;

use crate::riscv32_core::InstT;
use crate::riscv64_core::DRAM_BASE;
use crate::riscv64_core::Xlen64T;
use crate::riscv64_core::Addr64T;
use crate::riscv64_decoder::RiscvDecoder;

pub mod riscv32_core;
pub mod riscv64_core;
pub mod riscv_csr;
pub mod riscv_csr_bitdef;
pub mod riscv_exception;
pub mod riscv32_insts;
pub mod riscv64_insts;
pub mod riscv64_insts_fpu;
pub mod riscv64_decoder;
pub mod riscv_mmu;
pub mod riscv_inst_mnemonic;
pub mod riscv_tracer;
pub mod riscv_inst_operand;
pub mod riscv_inst_id;

pub fn swimmer_rust_exec(xlen: i32, filename: String) -> i64
{
    let file = File::open(filename.clone()).unwrap();
    let filebuf = BufReader::new(file);
    let mut hex_addr = 0;

    let mut riscv64_core = Riscv64Env::new(xlen);

    for result in filebuf.bytes() {
        match result {
            Ok(l) => {
                riscv64_core.write_memory_byte(hex_addr + DRAM_BASE as Addr64T, l as Xlen64T);
                hex_addr = hex_addr + 1;
            },
            Err(_result) => { panic!("Exit"); }
        }
    }

    let mut count = 0;
    while count < 65535 && !riscv64_core.get_is_finish_cpu() {
        // println!("InstNo: {:10}", count);
        let inst_data: InstT;
        match riscv64_core.fetch_bus() {
            Ok(v) => { inst_data = v; },
            Err(_result) => { continue; },
        }
        riscv64_core.m_trace.format_operand();
        match riscv64_core.decode_inst(inst_data) {
            None => panic!("<Error: Unknown instruction : inst={:08x}>\n", inst_data),
            Some(inst_decode) => riscv64_core.execute_inst(inst_decode, inst_data as InstT, count),
        }

        count += 1;
    }

    return riscv64_core.get_tohost();
}
