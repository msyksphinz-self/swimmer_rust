use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::{env};

extern crate getopts;
use getopts::Options;

mod core_base;
mod riscv32_core;
mod riscv32_insts;
mod riscv64_core;
mod riscv64_insts;
mod riscv_csr;
mod riscv_csr_bitdef;
mod riscv_exception;
mod riscv_mmu;
mod riscv_inst_mnemonic;
mod riscv_tracer;
mod riscv_inst_operand;

use crate::riscv64_core::Riscv64Core;
use crate::riscv64_core::Riscv64Env;

use crate::riscv32_insts::RiscvInsts;

use crate::riscv32_core::InstT;
use crate::riscv32_core::DRAM_BASE;
use crate::riscv64_core::Xlen64T;

use crate::riscv32_core::MemResult;

#[derive(Debug)]
struct Args {
    bin_file: Vec<String>,
    rv_arch: Option<String>,
}

#[allow(unused_variables)]
fn print_usage(program: &str, opts: &Options) {
    writeln!(std::io::stderr(), "Usage: {} binfile", program).unwrap();
    std::process::exit(1);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "arch", "select RISC-V architecture", "ARCH");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    if matches.free.is_empty() {
        print_usage(&program, &opts);
    }

    Args {
        bin_file: matches.free.clone(),
        rv_arch: matches.opt_str("arch"),
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let args = parse_args();

    println!("RISC-V Arch = {:?}", Some(args.rv_arch));

    let filename = args.bin_file[0].clone();
    let file = File::open(filename.clone()).unwrap();
    let filebuf = BufReader::new(file);
    let mut hex_addr = 0;

    let mut riscv64_core = Riscv64Env::new();

    for result in filebuf.bytes() {
        let l = result?;
        riscv64_core.write_memory_byte(hex_addr + DRAM_BASE, l as Xlen64T);
        hex_addr = hex_addr + 1;
    }

    let mut count = 0;
    while count < 65535 && !riscv64_core.get_is_finish_cpu() {
        // println!("InstNo: {:10}", count);
        let (result, inst_data) = riscv64_core.fetch_bus();
        if result != MemResult::NoExcept {
            continue;
        }
        riscv64_core.m_trace.format_operand();
        match riscv64_core.decode_inst(inst_data) {
            None => panic!("<Error: Unknown instruction : inst={:08x}>\n", inst_data),
            Some(inst_decode) => riscv64_core.execute_inst(inst_decode, inst_data as InstT, count),
        }

        count += 1;
    }

    if riscv64_core.get_tohost() == 1 {
        eprintln!("PASS : {}", filename.clone());
    } else {
        eprintln!("FAIL : {}", filename.clone());
    }

    Ok(())
}
