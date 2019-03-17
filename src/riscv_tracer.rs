use crate::riscv_core::PrivMode;
use crate::riscv_core::VmMode;

pub struct Tracer {
    m_priv: PrivMode,
    m_vmmode: VmMode,
    m_executed_pc: AddrType,
    m_executed_phypc: AddrType,
    m_inst_hex: InstType,
    m_step: u32,

    m_trace_type: Vec<TraceType>,

    /* for Register Read/Write */
    m_trace_size: Vec<u8>,
    m_trace_addr: Vec<AddrType>,
    m_trace_value: Vec<XlenType>,
    m_trace_memresult: Vec<MemResult>, /* Memory Access Result */
}

pub trait RiscvTracer {
    fn clear();
}

impl RiscvTracer for Trace {
    fn clear() {
        m_priv = PrivMode::Machine;
        m_vmmode = VmMode::VmBare;
        m_executed_pc = 0;
        m_executed_phypc = 0;
        m_inst_hex = 0;
        m_step = 0;

        m_trace_type = vec![0];

        /* for Register Read/Write */
        m_trace_size = vec![0];
        m_trace_addr = vec![0];
        m_trace_value = vec![0];
        m_trace_memresult = vec![MemResult::NoExcept]; /* Memory Access Result */
    }
}
