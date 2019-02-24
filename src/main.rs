use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::env;

mod core_base;
mod riscv_csr;
mod riscv_core;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::EnvBase;

use crate::riscv_core::DRAM_BASE;
use crate::riscv_core::InstType;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: swimmer_rust_origin hexfile").unwrap();
        std::process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let filebuf = BufReader::new (file);
    let mut hex_addr = 0;

    let mut riscv64_core = EnvBase::new();

    for result in filebuf.bytes() {
        let l = result?;
        riscv64_core.write_memory_byte (hex_addr + DRAM_BASE, l as u8);
        hex_addr = hex_addr + 1;
    }

    for count in 0..128 {
        let inst_data = riscv64_core.fetch_memory ();
        let inst_decode = riscv64_core.decode_inst(inst_data);
        riscv64_core.execute_inst(inst_decode, inst_data as InstType);
    }

    Ok(())
}
