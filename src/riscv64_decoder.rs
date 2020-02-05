use crate::riscv32_insts::RiscvInstId;
use crate::riscv64_core::Riscv64Env;

use crate::riscv32_core::InstT;

pub trait RiscvDecoder {
    fn decode_inst(&mut self, inst: InstT) -> Option<RiscvInstId>;
}

impl RiscvDecoder for Riscv64Env {
    fn decode_inst(&mut self, inst: InstT) -> Option<RiscvInstId> {
        let opcode = inst & 0x7f;
        let funct3 = (inst >> 12) & 0x07;
        let funct7 = (inst >> 25) & 0x7f;
        let funct6 = (inst >> 26) & 0x3f; // RV64 shamt
        let rs2 = (inst >> 20) & 0x1f;

        return match opcode {
            0x0f => match funct3 {
                0b000 => Some(RiscvInstId::FENCE),
                0b001 => Some(RiscvInstId::FENCEI),
                _ => None,
            },
            0x1b => match funct3 {
                0b000 => Some(RiscvInstId::ADDIW),
                0b001 => Some(RiscvInstId::SLLIW),
                0b101 => match funct7 {
                    0b0000000 => Some(RiscvInstId::SRLIW),
                    0b0100000 => Some(RiscvInstId::SRAIW),
                    _ => None,
                },
                _ => None,
            },
            0x3b => match funct3 {
                0b000 => match funct7 {
                    0b0000000 => Some(RiscvInstId::ADDW),
                    0b0100000 => Some(RiscvInstId::SUBW),
                    0b0000001 => Some(RiscvInstId::MULW),
                    _ => None,
                },
                0b001 => match funct7 {
                    0b0000000 => Some(RiscvInstId::SLLW),
                    _ => None,
                },
                0b100 => match funct7 {
                    0b0000001 => Some(RiscvInstId::DIVW),
                    _ => None,
                },
                0b101 => match funct7 {
                    0b0000000 => Some(RiscvInstId::SRLW),
                    0b0100000 => Some(RiscvInstId::SRAW),
                    0b0000001 => Some(RiscvInstId::DIVUW),
                    _ => None,
                },
                0b110 => match funct7 {
                    0b0000001 => Some(RiscvInstId::REMW),
                    _ => None,
                },
                0b111 => match funct7 {
                    0b0000001 => Some(RiscvInstId::REMUW),
                    _ => None,
                },
                _ => None,
            },
            0x33 => match funct7 {
                0b0000000 => match funct3 {
                    0b000 => Some(RiscvInstId::ADD),
                    0b001 => Some(RiscvInstId::SLL),
                    0b010 => Some(RiscvInstId::SLT),
                    0b011 => Some(RiscvInstId::SLTU),
                    0b100 => Some(RiscvInstId::XOR),
                    0b101 => Some(RiscvInstId::SRL),
                    0b110 => Some(RiscvInstId::OR),
                    0b111 => Some(RiscvInstId::AND),
                    _ => None,
                },
                0b0100000 => match funct3 {
                    0b000 => Some(RiscvInstId::SUB),
                    0b101 => Some(RiscvInstId::SRA),
                    _ => None,
                },
                0b0000001 => match funct3 {
                    0b000 => Some(RiscvInstId::MUL),
                    0b001 => Some(RiscvInstId::MULH),
                    0b010 => Some(RiscvInstId::MULHSU),
                    0b011 => Some(RiscvInstId::MULHU),
                    0b100 => Some(RiscvInstId::DIV),
                    0b101 => Some(RiscvInstId::DIVU),
                    0b110 => Some(RiscvInstId::REM),
                    0b111 => Some(RiscvInstId::REMU),
                    _ => None,
                },
                _ => None,
            },
            0x03 => match funct3 {
                0b000 => Some(RiscvInstId::LB),
                0b001 => Some(RiscvInstId::LH),
                0b010 => Some(RiscvInstId::LW),
                0b100 => Some(RiscvInstId::LBU),
                0b101 => Some(RiscvInstId::LHU),
                0b110 => Some(RiscvInstId::LWU),
                0b011 => Some(RiscvInstId::LD),
                _ => None,
            },
            0x23 => match funct3 {
                0b000 => Some(RiscvInstId::SB),
                0b001 => Some(RiscvInstId::SH),
                0b010 => Some(RiscvInstId::SW),
                0b011 => Some(RiscvInstId::SD),
                _ => None,
            },
            0x37 => Some(RiscvInstId::LUI),
            0x17 => Some(RiscvInstId::AUIPC),
            0x63 => match funct3 {
                0b000 => Some(RiscvInstId::BEQ),
                0b001 => Some(RiscvInstId::BNE),
                0b100 => Some(RiscvInstId::BLT),
                0b101 => Some(RiscvInstId::BGE),
                0b110 => Some(RiscvInstId::BLTU),
                0b111 => Some(RiscvInstId::BGEU),
                _ => None,
            },
            0x13 => match funct3 {
                0b000 => Some(RiscvInstId::ADDI),
                0b010 => Some(RiscvInstId::SLTI),
                0b011 => Some(RiscvInstId::SLTIU),
                0b100 => Some(RiscvInstId::XORI),
                0b110 => Some(RiscvInstId::ORI),
                0b111 => Some(RiscvInstId::ANDI),
                0b001 => Some(RiscvInstId::SLLI),
                0b101 => match funct6 {
                    0b000000 => Some(RiscvInstId::SRLI),
                    0b010000 => Some(RiscvInstId::SRAI),
                    _ => None,
                },
                _ => None,
            },
            0x6f => Some(RiscvInstId::JAL),
            0x67 => Some(RiscvInstId::JALR),
            0x73 => match funct3 {
                0x000 => match funct7 {
                    0b0001001 => Some(RiscvInstId::SFENCEVMA),
                    0b0000000 => match rs2 {
                        0b00001 => Some(RiscvInstId::EBREAK),
                        0b00010 => Some(RiscvInstId::URET),
                        0b00000 => Some(RiscvInstId::ECALL),
                        _ => None,
                    }
                    0b0001000 => match rs2 {
                        0b00010 => Some(RiscvInstId::SRET),
                        _ => None,
                    }
                    0b0011000 => match rs2 {
                        0b00010 => Some(RiscvInstId::MRET),
                        _ => None,
                    }
                    _ => None,
                }
                0b001 => Some(RiscvInstId::CSRRW),
                0b010 => Some(RiscvInstId::CSRRS),
                0b011 => Some(RiscvInstId::CSRRC),
                0b101 => Some(RiscvInstId::CSRRWI),
                0b110 => Some(RiscvInstId::CSRRSI),
                0b111 => Some(RiscvInstId::CSRRCI),
                _ => None,
            },
            _ => Some(RiscvInstId::WFI),
        };
    }
}
