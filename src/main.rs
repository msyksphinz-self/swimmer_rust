use std::io::{Write};
use std::{env};

extern crate getopts;
use getopts::Options;

mod core_base;
mod riscv64_core;
mod riscv64_insts;
mod riscv64_insts_fpu;
mod riscv_csr;
mod riscv_csr_bitdef;
mod riscv_exception;
mod riscv_mmu;
mod riscv_inst_mnemonic;
mod riscv_tracer;
mod riscv_inst_operand;
mod riscv_inst_id;

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

    let rv32_str = "rv32".to_string();
    let rv64_str = "rv64".to_string();
    let xlen = match args.rv_arch {
        Some(rv_arch_str) => match rv_arch_str {
            rv32_str => 32,
            rv64_str => 64,
            _ => panic!("Unknown XLEN! Should specify rv32 or rv64"),
        },
        None => 64  // Default XLEN=64
    };

    let ret = swimmer_rust::swimmer_rust_exec(xlen, args.bin_file[0].clone());

    if ret == 1 {
        eprintln!("PASS : {}", args.bin_file[0]);
        Ok(())
    } else {
        eprintln!("FAIL : {}", args.bin_file[0]);
        Ok(())
    }
}
