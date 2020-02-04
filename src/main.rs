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
use crate::riscv64_core::Addr64T;
use crate::riscv64_core::Xlen64T;

#[derive(Debug)]
struct Args {
    bin_file: Vec<String>,
    rv_arch: Option<String>,
}

pub fn add_two(a: i32) -> i32 {
    a + 2
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

    let ret = swimmer_rust::swimmer_rust_exec(args.bin_file[0].clone());

    if ret == 1 {
        eprintln!("PASS : {}", args.bin_file[0]);
        Ok(())
    } else {
        eprintln!("FAIL : {}", args.bin_file[0]);
        Ok(())
    }
}
