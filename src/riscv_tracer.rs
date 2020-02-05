use std::collections::HashMap;

use crate::riscv32_core::PrivMode;
use crate::riscv32_core::VMMode;

use crate::riscv32_core::MemResult;

use crate::riscv32_core::InstT;
use crate::riscv64_core::Addr64T;
use crate::riscv64_core::Xlen64T;

use crate::riscv_inst_id::RiscvInstId;

use crate::riscv_inst_mnemonic::*;

use crate::riscv_inst_operand::OperandType;
use crate::riscv_inst_operand::OperandInfo;

use std::f32;

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
    pub m_trace_addr: Addr64T,
    pub m_trace_value: Xlen64T,
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
    pub m_executed_pc: Addr64T,
    pub m_executed_phypc: Addr64T,
    pub m_inst_hex: InstT,
    pub m_dec_inst: Option<RiscvInstId>,
    pub m_step: u32,

    pub m_trace_info: Vec<TraceInfo>,

    pub m_inst_operand_map: HashMap<RiscvInstId, OperandInfo>,
}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {
            m_priv: PrivMode::Machine,
            m_vmmode: VMMode::Mbare,
            m_executed_pc: 0,
            m_executed_phypc: 0,
            m_inst_hex: 0,
            m_dec_inst: None,
            m_step: 0,

            m_trace_info: vec![],
            m_inst_operand_map: HashMap::new(),
        }
    }
}

pub trait RiscvTracer {
    fn clear(&mut self);
    fn print_trace(&mut self);
    fn extract_bit_field(&mut self, hex: InstT, left: &u32, right: &u32) -> InstT;
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

    fn extract_bit_field(&mut self, hex: InstT, left: &u32, right: &u32) -> InstT {
        let mask: InstT = (1 << (left - right + 1)) - 1;
        return (hex >> right) & mask;
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

        match self.m_dec_inst {
            Some(id) => {
                let inst_str = get_riscv_inst_mnemonic(id);
                let operand_info = self.m_inst_operand_map.get(&id);
                match operand_info {
                    Some(operand_info) => {
                        let mut at_index = 0;
                        let mut consume_idx = 0;
                        for c in inst_str.chars() {
                            if c == '@' {
                                let msb = operand_info.m_msb_lst[at_index];
                                let lsb = operand_info.m_lsb_lst[at_index];

                                let mask = (1 << (msb - lsb + 1)) - 1;
                                let opr_val = (self.m_inst_hex >> lsb) & mask;

                                match operand_info.m_type_lst[at_index] {
                                    OperandType::TypeXReg    => { print!("x{:02}", opr_val); consume_idx = consume_idx + 3; },
                                    OperandType::TypeFreg    => { print!("f{:02}", opr_val); consume_idx = consume_idx + 3; },
                                    OperandType::TypeSign    => { print!("{:}",  opr_val);   consume_idx = consume_idx + ((opr_val as f32).log10() as u32); },
                                    OperandType::TypeBit     => { print!("0b{:b}", opr_val); consume_idx = consume_idx + (msb - lsb + 1); },
                                    OperandType::TypeUnSign  => { print!("0x{:x}", opr_val); consume_idx = consume_idx + 2 + ((opr_val as f32).log10() as u32); },
                                    OperandType::TypeUnSignJ => { print!("0x{:x}", opr_val); consume_idx = consume_idx + 2 + ((opr_val as f32).log10() as u32); },
                                    OperandType::TypeSignBit => { print!("0b{:b}", opr_val); consume_idx = consume_idx + 2 + (msb - lsb + 1); },
                                    OperandType::TypeHex     => { let bit_width = ((msb as f32 - lsb as f32 + 1.0) / 4.0).ceil() as u32;
                                                                  print!("0x{:0>width$x}", opr_val, width = bit_width as usize);
                                                                  consume_idx = consume_idx + 2 + bit_width; }
                                    OperandType::TypeRoundMode => panic!("TypeRoundMode is currently not supported"),
                                    OperandType::TypeCompactReg => panic!("TypeCompactReg is currently not supported"),
                                    OperandType::TypeCompactFReg => panic!("TypeCompactFReg is currently not supported"),
                                    _ => panic!("Unknown operand type {:?}", operand_info.m_type_lst[at_index] as u32),
                                }
                                at_index = at_index + 1;
                            } else {
                                print!("{:}", c);
                                consume_idx = consume_idx + 1
                            }
                        }
                        for _idx in consume_idx..30 {
                            print!(" ");
                        }
                    }
                    None => panic!("Implementation Error: No operand information in this inst."),
                }
                print!(":");
            }
            _ => {}
        }

        for trace_idx in 0..self.m_trace_info.len() {
            match self.m_trace_info[trace_idx].m_trace_type {
                TraceType::XRegWrite => {
                    print!(
                        "x{:02}<={:016x} ",
                        self.m_trace_info[trace_idx].m_trace_addr,
                        self.m_trace_info[trace_idx].m_trace_value
                    );
                }
                TraceType::XRegRead => {
                    print!(
                        "x{:02}=>{:016x} ",
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
        print!("\n");
    }
}
