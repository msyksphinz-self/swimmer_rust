use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

mod core_base;
mod riscv_core;
mod riscv_csr;
mod riscv_csr_bitdef;
mod riscv_exception;
mod riscv_insts;
mod riscv_mmu;
mod riscv_tracer;

use crate::riscv_core::EnvBase;
use crate::riscv_core::Riscv32Core;

use crate::riscv_insts::Riscv32Insts;
use crate::riscv_insts::RiscvInst;

use crate::riscv_core::InstT;
use crate::riscv_core::XlenT;
use crate::riscv_core::DRAM_BASE;

use crate::riscv_core::MemResult;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: swimmer_rust_origin binfile").unwrap();
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let filebuf = BufReader::new(file);
    let mut hex_addr = 0;

    let mut riscv32_core = EnvBase::new();

    for result in filebuf.bytes() {
        let l = result?;
        riscv32_core.write_memory_byte(hex_addr + DRAM_BASE, l as XlenT);
        hex_addr = hex_addr + 1;
    }

    let mut count = 0;
    while count < 65535 && !riscv32_core.get_is_finish_cpu() {
        // println!("InstNo: {:10}", count);
        let (result, inst_data) = riscv32_core.fetch_bus();
        if result != MemResult::NoExcept {
            continue;
        }
        let inst_decode = riscv32_core.decode_inst(inst_data);
        riscv32_core.execute_inst(inst_decode, inst_data as InstT, count);

        count += 1;
    }

    if riscv32_core.get_tohost() == 1 {
        eprintln!("PASS : {}", args[1]);
    } else {
        eprintln!("FAIL : {}", args[1]);
    }

    Ok(())
}
