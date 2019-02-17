use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::env;

mod core_base;
mod riscv_core;

use crate::riscv_core::Riscv64Core;
use crate::riscv_core::EnvBase;

use crate::riscv_core::DRAM_BASE;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    
    if args.len() != 2 {
        writeln!(std::io::stderr(), "Usage: swimmer_rust_origin hexfile").unwrap();
        std::process::exit(1);
    }
    
    let file = File::open(&args[1]).unwrap();
    let filebuf = BufReader::new (file);
    for result in filebuf.bytes() {
        let l = result?;
        println!("{:x}", l);
    }
    
    let mut riscv64_core = EnvBase::new();
    for addr in 0..32 {
        riscv64_core.write_memory (addr * 4 + DRAM_BASE, addr as u32);
    }
    for addr in 0..32 {
        let data = riscv64_core.read_memory (addr * 4 + DRAM_BASE);
        println!("Read Data = {:08x}", data);
    }
    for addr in 0..32 {
        let data = riscv64_core.fetch_memory (addr * 4 + DRAM_BASE);
        println!("Fetch Data = {:08x}", data);
    }

    Ok(())
}
