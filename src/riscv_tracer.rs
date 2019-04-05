use crate::riscv_core::PrivMode;
use crate::riscv_core::VMMode;

use crate::riscv_core::MemResult;

use crate::riscv_core::AddrT;
use crate::riscv_core::InstT;
use crate::riscv_core::XlenT;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TraceType {
    XRegWrite,
    XRegRead, // Integer
    // FRegWrite,
    // FRegRead,    // Single-Precision Float
    // DRegWrite,
    // DRegRead,    // Double-Precision Float
    MemRead,
    MemWrite, // Memory Write
    // CsrWrite,
    // CsrRead,
    None,
}

pub struct TraceInfo {
    pub m_trace_type: TraceType,
    pub m_trace_size: u32,
    pub m_trace_addr: AddrT,
    pub m_trace_value: XlenT,
    pub m_trace_memresult: MemResult, /* Memory Access Result */
}

impl TraceInfo {
    pub fn new() -> TraceInfo {
        TraceInfo {
            m_trace_type: TraceType::None,
            m_trace_size: 0,
            m_trace_addr: 0,
            m_trace_value: 0,
            m_trace_memresult: MemResult::NoExcept, /* Memory Access Result */
        }
    }
}

pub struct Tracer {
    pub m_priv: PrivMode,
    pub m_vmmode: VMMode,
    pub m_executed_pc: AddrT,
    pub m_executed_phypc: AddrT,
    pub m_inst_hex: InstT,
    pub m_step: u32,

    pub m_trace_info: Vec<TraceInfo>,
}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {
            m_priv: PrivMode::Machine,
            m_vmmode: VMMode::Mbare,
            m_executed_pc: 0,
            m_executed_phypc: 0,
            m_inst_hex: 0,
            m_step: 0,

            m_trace_info: vec![],
        }
    }
}

pub trait RiscvTracer {
    fn clear(&mut self);
    fn print_trace(&mut self);
}

impl RiscvTracer for Tracer {
    fn clear(&mut self) {
        self.m_priv = PrivMode::Machine;
        self.m_vmmode = VMMode::Mbare;
        self.m_executed_pc = 0;
        self.m_executed_phypc = 0;
        self.m_inst_hex = 0;
        self.m_step = 0;

        self.m_trace_info = vec![];
    }

    fn print_trace(&mut self) {
        print!("{:10}:", self.m_step);
        print!(
            "{}:",
            match self.m_priv {
                PrivMode::User => "U",
                PrivMode::Supervisor => "S",
                PrivMode::Hypervisor => "H",
                PrivMode::Machine => "M",
            }
        );
        print!(
            "{}:",
            match self.m_vmmode {
                VMMode::Mbare => "Bare",
                VMMode::Sv32 => "Sv32",
                VMMode::Sv39 => "Sv39",
                VMMode::Sv48 => "Sv48",
                VMMode::Sv57 => "Sv57",
                VMMode::Sv64 => "Sv64",
            }
        );
        print!("{:08x}:{:08x}:", self.m_executed_pc, self.m_inst_hex,);

        for trace_idx in 0..self.m_trace_info.len() {
            match self.m_trace_info[trace_idx].m_trace_type {
                TraceType::XRegWrite => {
                    print!(
                        "x{:02}<={:08x} ",
                        self.m_trace_info[trace_idx].m_trace_addr,
                        self.m_trace_info[trace_idx].m_trace_value
                    );
                }
                TraceType::XRegRead => {
                    print!(
                        "x{:02}=>{:08x} ",
                        self.m_trace_info[trace_idx].m_trace_addr,
                        self.m_trace_info[trace_idx].m_trace_value
                    );
                }
                TraceType::MemWrite => {
                    print!(
                        "({:08x})<={:08x} ",
                        self.m_trace_info[trace_idx].m_trace_addr,
                        self.m_trace_info[trace_idx].m_trace_value
                    );
                }
                TraceType::MemRead => {
                    print!(
                        "({:08x})=>{:08x} ",
                        self.m_trace_info[trace_idx].m_trace_addr,
                        self.m_trace_info[trace_idx].m_trace_value
                    );
                }
                _ => {
                    print!(
                        "[{:} is not supported] ",
                        self.m_trace_info[trace_idx].m_trace_type as u32
                    );
                }
            }
        }
        println!("  // DASM({:08x})", self.m_inst_hex);
    }
}
