use crate::riscv_tracer::Tracer;
use crate::riscv32_insts::RiscvInstId;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum OperandType {
    TypeXReg,
    TypeFreg,
    TypeDreg,
    TypeSign,
    TypeBit,
    TypeUnSign,
    TypeUnSignJ,
    TypeSignBit,
    TypeHex,
    TypeRoundMode,
    TypeCompactReg,
    TypeCompactFReg,
    TypeNone,
}


pub struct OperandInfo {
    pub m_size: u32,
    pub m_type_lst: [OperandType; 256],
    pub m_msb_lst: [u32; 256],
    pub m_lsb_lst: [u32; 256],
    pub m_connect: [bool; 256],
}


impl OperandInfo {
    pub fn new() -> OperandInfo {
        OperandInfo {
            m_size: 0,
            m_type_lst:[OperandType::TypeNone; 256],
            m_msb_lst: [0; 256],
            m_lsb_lst: [0; 256],
            m_connect: [false; 256],
        }
    }
}


impl Tracer {
    pub fn FormatOperand(&mut self)
    {
        {
	        let mut inst_operand = OperandInfo::new();

	        // InstId_t::INST_ID_LUI
            inst_operand.m_size = 2;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:12]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 12;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:12]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LUI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_AUIPC
            inst_operand.m_size = 2;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:12]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 12;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:12]"]

	        self.m_inst_operand_map.insert(RiscvInstId::AUIPC, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_JAL
            inst_operand.m_size = 2;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "uj[31:12]"]
            inst_operand.m_type_lst[1] = OperandType::TypeUnSignJ;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 12;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "uj[31:12]"]

	        self.m_inst_operand_map.insert(RiscvInstId::JAL, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_JALR
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::JALR, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_BEQ
            inst_operand.m_size = 6;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 19;
            inst_operand.m_lsb_lst[0] = 15;
            inst_operand.m_connect[0] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 24;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 31;
            inst_operand.m_connect[2] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[3] = 7;
            inst_operand.m_lsb_lst[3] = 7;
            inst_operand.m_connect[3] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[4] = 30;
            inst_operand.m_lsb_lst[4] = 25;
            inst_operand.m_connect[4] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[5] = 11;
            inst_operand.m_lsb_lst[5] = 8;
            inst_operand.m_connect[5] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]

	        self.m_inst_operand_map.insert(RiscvInstId::BEQ, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_BNE
            inst_operand.m_size = 6;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 19;
            inst_operand.m_lsb_lst[0] = 15;
            inst_operand.m_connect[0] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 24;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 31;
            inst_operand.m_connect[2] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[3] = 7;
            inst_operand.m_lsb_lst[3] = 7;
            inst_operand.m_connect[3] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[4] = 30;
            inst_operand.m_lsb_lst[4] = 25;
            inst_operand.m_connect[4] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[5] = 11;
            inst_operand.m_lsb_lst[5] = 8;
            inst_operand.m_connect[5] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]

	        self.m_inst_operand_map.insert(RiscvInstId::BNE, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_BLT
            inst_operand.m_size = 6;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 19;
            inst_operand.m_lsb_lst[0] = 15;
            inst_operand.m_connect[0] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 24;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 31;
            inst_operand.m_connect[2] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[3] = 7;
            inst_operand.m_lsb_lst[3] = 7;
            inst_operand.m_connect[3] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[4] = 30;
            inst_operand.m_lsb_lst[4] = 25;
            inst_operand.m_connect[4] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[5] = 11;
            inst_operand.m_lsb_lst[5] = 8;
            inst_operand.m_connect[5] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]

	        self.m_inst_operand_map.insert(RiscvInstId::BLT, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_BGE
            inst_operand.m_size = 6;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 19;
            inst_operand.m_lsb_lst[0] = 15;
            inst_operand.m_connect[0] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 24;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 31;
            inst_operand.m_connect[2] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[3] = 7;
            inst_operand.m_lsb_lst[3] = 7;
            inst_operand.m_connect[3] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[4] = 30;
            inst_operand.m_lsb_lst[4] = 25;
            inst_operand.m_connect[4] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[5] = 11;
            inst_operand.m_lsb_lst[5] = 8;
            inst_operand.m_connect[5] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]

	        self.m_inst_operand_map.insert(RiscvInstId::BGE, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_BLTU
            inst_operand.m_size = 6;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 19;
            inst_operand.m_lsb_lst[0] = 15;
            inst_operand.m_connect[0] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 24;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 31;
            inst_operand.m_connect[2] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[3] = 7;
            inst_operand.m_lsb_lst[3] = 7;
            inst_operand.m_connect[3] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[4] = 30;
            inst_operand.m_lsb_lst[4] = 25;
            inst_operand.m_connect[4] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[5] = 11;
            inst_operand.m_lsb_lst[5] = 8;
            inst_operand.m_connect[5] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]

	        self.m_inst_operand_map.insert(RiscvInstId::BLTU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_BGEU
            inst_operand.m_size = 6;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 19;
            inst_operand.m_lsb_lst[0] = 15;
            inst_operand.m_connect[0] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 24;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 31;
            inst_operand.m_connect[2] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[3] = 7;
            inst_operand.m_lsb_lst[3] = 7;
            inst_operand.m_connect[3] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[4] = 30;
            inst_operand.m_lsb_lst[4] = 25;
            inst_operand.m_connect[4] = true;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]
            inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
            inst_operand.m_msb_lst[5] = 11;
            inst_operand.m_lsb_lst[5] = 8;
            inst_operand.m_connect[5] = false;
            // ["r[19:15]", "r[24:20]", "u[31:31]|", "u[7:7]|", "u[30:25]|", "u[11:8]"]

	        self.m_inst_operand_map.insert(RiscvInstId::BGEU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LB
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LB, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LH
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LH, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LBU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LBU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LHU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LHU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SB
            inst_operand.m_size = 4;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 24;
            inst_operand.m_lsb_lst[0] = 20;
            inst_operand.m_connect[0] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 25;
            inst_operand.m_connect[1] = true;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 11;
            inst_operand.m_lsb_lst[2] = 7;
            inst_operand.m_connect[2] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[3] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[3] = 19;
            inst_operand.m_lsb_lst[3] = 15;
            inst_operand.m_connect[3] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SB, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SH
            inst_operand.m_size = 4;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 24;
            inst_operand.m_lsb_lst[0] = 20;
            inst_operand.m_connect[0] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 25;
            inst_operand.m_connect[1] = true;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 11;
            inst_operand.m_lsb_lst[2] = 7;
            inst_operand.m_connect[2] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[3] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[3] = 19;
            inst_operand.m_lsb_lst[3] = 15;
            inst_operand.m_connect[3] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SH, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SW
            inst_operand.m_size = 4;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 24;
            inst_operand.m_lsb_lst[0] = 20;
            inst_operand.m_connect[0] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 25;
            inst_operand.m_connect[1] = true;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 11;
            inst_operand.m_lsb_lst[2] = 7;
            inst_operand.m_connect[2] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[3] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[3] = 19;
            inst_operand.m_lsb_lst[3] = 15;
            inst_operand.m_connect[3] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ADDI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::ADDI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLTI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLTI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLTIU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLTIU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_XORI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::XORI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ORI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::ORI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ANDI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::ANDI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLLI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[25:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[25:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 25;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[25:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLLI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRLI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRLI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRAI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRAI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ADD
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::ADD, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SUB
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SUB, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLL
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLL, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLT
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLT, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLTU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLTU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_XOR
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::XOR, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRL
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRL, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRA
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRA, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_OR
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::OR, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_AND
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::AND, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_FENCE
            inst_operand.m_size = 0;

	        self.m_inst_operand_map.insert(RiscvInstId::FENCE, inst_operand);
        }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FENCE_I
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FENCE_I, inst_operand);
        // }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_MUL
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::MUL, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_MULH
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::MULH, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_MULHSU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::MULHSU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_MULHU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::MULHU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_DIV
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::DIV, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_DIVU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::DIVU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_REM
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::REM, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_REMU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::REMU, inst_operand);
        }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_LR_W
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::LR_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_SC_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::SC_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOSWAP_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOSWAP_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOADD_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOADD_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOXOR_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOXOR_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOAND_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOAND_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOOR_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOOR_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMIN_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMIN_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMAX_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMAX_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMINU_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMINU_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMAXU_W
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMAXU_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FLW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "h[31:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[1] = 31;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "h[31:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "h[31:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FLW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSW
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 24;
        //     inst_operand.m_lsb_lst[0] = 20;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[1] = 31;
        //     inst_operand.m_lsb_lst[1] = 25;
        //     inst_operand.m_connect[1] = true;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 11;
        //     inst_operand.m_lsb_lst[2] = 7;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[3] = 19;
        //     inst_operand.m_lsb_lst[3] = 15;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMADD_S
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMADD_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMSUB_S
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMSUB_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FNMSUB_S
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FNMSUB_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FNMADD_S
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FNMADD_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FADD_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FADD_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSUB_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSUB_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMUL_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMUL_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FDIV_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FDIV_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSQRT_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSQRT_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSGNJ_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSGNJ_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSGNJN_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSGNJN_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSGNJX_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSGNJX_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMIN_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMIN_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMAX_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMAX_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_W_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_W_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_WU_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_WU_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMV_X_W
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMV_X_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FEQ_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FEQ_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FLT_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FLT_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FLE_S
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FLE_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCLASS_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCLASS_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_S_W
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_S_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_S_WU
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_S_WU, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMV_W_X
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMV_W_X, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FLD
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]", "h[31:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]", "h[31:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 31;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "r[19:15]", "h[31:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FLD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSD
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 24;
        //     inst_operand.m_lsb_lst[0] = 20;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[1] = 31;
        //     inst_operand.m_lsb_lst[1] = 25;
        //     inst_operand.m_connect[1] = true;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 11;
        //     inst_operand.m_lsb_lst[2] = 7;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[3] = 19;
        //     inst_operand.m_lsb_lst[3] = 15;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMADD_D
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMADD_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMSUB_D
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMSUB_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FNMSUB_D
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FNMSUB_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FNMADD_D
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[3] = 31;
        //     inst_operand.m_lsb_lst[3] = 27;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]", "f[31:27]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FNMADD_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FADD_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FADD_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSUB_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSUB_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMUL_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMUL_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FDIV_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FDIV_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSQRT_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSQRT_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSGNJ_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSGNJ_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSGNJN_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSGNJN_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FSGNJX_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSGNJX_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMIN_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMIN_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMAX_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMAX_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_S_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_S_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_D_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_D_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FEQ_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FEQ_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FLT_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FLT_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FLE_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "f[19:15]", "f[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FLE_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCLASS_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCLASS_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_W_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_W_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_WU_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_WU_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_D_W
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_D_W, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_D_WU
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_D_WU, inst_operand);
        // }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_CSRRW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::CSRRW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_CSRRS
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::CSRRS, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_CSRRC
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::CSRRC, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_CSRRWI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::CSRRWI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_CSRRSI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::CSRRSI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_CSRRCI
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "h[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::CSRRCI, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ECALL
            inst_operand.m_size = 0;

	        self.m_inst_operand_map.insert(RiscvInstId::ECALL, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_EBREAK
            inst_operand.m_size = 0;

	        self.m_inst_operand_map.insert(RiscvInstId::EBREAK, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_URET
            inst_operand.m_size = 0;

	        self.m_inst_operand_map.insert(RiscvInstId::URET, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRET
            inst_operand.m_size = 0;

	        self.m_inst_operand_map.insert(RiscvInstId::SRET, inst_operand);
        }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_HRET
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::HRET, inst_operand);
        // }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_MRET
            inst_operand.m_size = 0;

	        self.m_inst_operand_map.insert(RiscvInstId::MRET, inst_operand);
        }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_MRTS
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::MRTS, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_MRTH
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::MRTH, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_WFI
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::WFI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_SFENCE_VMA
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 19;
        //     inst_operand.m_lsb_lst[0] = 15;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::SFENCE_VMA, inst_operand);
        // }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LWU
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LWU, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_LD
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 20;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 19;
            inst_operand.m_lsb_lst[2] = 15;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "h[31:20]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::LD, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SD
            inst_operand.m_size = 4;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 24;
            inst_operand.m_lsb_lst[0] = 20;
            inst_operand.m_connect[0] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[1] = OperandType::TypeHex;
            inst_operand.m_msb_lst[1] = 31;
            inst_operand.m_lsb_lst[1] = 25;
            inst_operand.m_connect[1] = true;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 11;
            inst_operand.m_lsb_lst[2] = 7;
            inst_operand.m_connect[2] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]
            inst_operand.m_type_lst[3] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[3] = 19;
            inst_operand.m_lsb_lst[3] = 15;
            inst_operand.m_connect[3] = false;
            // ["r[24:20]", "h[31:25]|", "h[11:7]", "r[19:15]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SD, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ADDIW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeHex;
            inst_operand.m_msb_lst[2] = 31;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "h[31:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::ADDIW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLLIW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLLIW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRLIW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRLIW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRAIW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRAIW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_ADDW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::ADDW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SUBW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SUBW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SLLW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SLLW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRLW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRLW, inst_operand);
        }
        {
	        let mut inst_operand = OperandInfo::new();

            // InstId_t::INST_ID_SRAW
            inst_operand.m_size = 3;
            inst_operand.m_type_lst[0] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[0] = 11;
            inst_operand.m_lsb_lst[0] = 7;
            inst_operand.m_connect[0] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[1] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[1] = 19;
            inst_operand.m_lsb_lst[1] = 15;
            inst_operand.m_connect[1] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]
            inst_operand.m_type_lst[2] = OperandType::TypeXReg;
            inst_operand.m_msb_lst[2] = 24;
            inst_operand.m_lsb_lst[2] = 20;
            inst_operand.m_connect[2] = false;
            // ["r[11:7]", "r[19:15]", "r[24:20]"]

	        self.m_inst_operand_map.insert(RiscvInstId::SRAW, inst_operand);
        }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_MULW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::MULW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_DIVW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::DIVW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_DIVUW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::DIVUW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_REMW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::REMW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_REMUW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::REMUW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_LR_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::LR_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_SC_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 24;
        //     inst_operand.m_lsb_lst[2] = 20;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[19:15]", "r[24:20]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::SC_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOSWAP_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOSWAP_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOADD_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOADD_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOXOR_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOXOR_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOAND_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOAND_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOOR_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOOR_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMIN_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMIN_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMAX_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMAX_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMINU_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMINU_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_AMOMAXU_D
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 24;
        //     inst_operand.m_lsb_lst[1] = 20;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[2] = 19;
        //     inst_operand.m_lsb_lst[2] = 15;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "r[24:20]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::AMOMAXU_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_L_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_L_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_LU_S
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_LU_S, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_S_L
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_S_L, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_S_LU
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_S_LU, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_L_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_L_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_LU_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_LU_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMV_X_D
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "f[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMV_X_D, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_D_L
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_D_L, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FCVT_D_LU
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FCVT_D_LU, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_FMV_D_X
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 19;
        //     inst_operand.m_lsb_lst[1] = 15;
        //     inst_operand.m_connect[1] = false;
        //     // ["f[11:7]", "r[19:15]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FMV_D_X, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ADDI4SPN
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "u[12:5]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 5;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "u[12:5]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_ADDI4SPN, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FLD
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactFReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cf[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cf[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cf[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = false;
        //     // ["cf[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FLD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_LW
        //     inst_operand.m_size = 5;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 5;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 6;
        //     inst_operand.m_lsb_lst[4] = 6;
        //     inst_operand.m_connect[4] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_LW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FLW
        //     inst_operand.m_size = 5;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 5;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 6;
        //     inst_operand.m_lsb_lst[4] = 6;
        //     inst_operand.m_connect[4] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FLW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_LD
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_LD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FSD
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FSD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SW
        //     inst_operand.m_size = 5;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 5;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 6;
        //     inst_operand.m_lsb_lst[4] = 6;
        //     inst_operand.m_connect[4] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FSW
        //     inst_operand.m_size = 5;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 5;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 6;
        //     inst_operand.m_lsb_lst[4] = 6;
        //     inst_operand.m_connect[4] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[5:5]|", "u[12:10]|", "u[6:6]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FSW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SD
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 12;
        //     inst_operand.m_lsb_lst[3] = 10;
        //     inst_operand.m_connect[3] = false;
        //     // ["cr[4:2]", "cr[9:7]", "u[6:5]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_NOP
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_NOP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ADDI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[12:12]|", "u[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[12:12]|", "u[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "u[12:12]|", "u[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_ADDI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_JAL
        //     inst_operand.m_size = 8;
        //     inst_operand.m_type_lst[0] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[0] = 12;
        //     inst_operand.m_lsb_lst[0] = 12;
        //     inst_operand.m_connect[0] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 7;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 10;
        //     inst_operand.m_lsb_lst[2] = 9;
        //     inst_operand.m_connect[2] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 6;
        //     inst_operand.m_lsb_lst[3] = 6;
        //     inst_operand.m_connect[3] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 7;
        //     inst_operand.m_lsb_lst[4] = 7;
        //     inst_operand.m_connect[4] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[5] = 2;
        //     inst_operand.m_lsb_lst[5] = 2;
        //     inst_operand.m_connect[5] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[6] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[6] = 11;
        //     inst_operand.m_lsb_lst[6] = 11;
        //     inst_operand.m_connect[6] = true;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[7] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[7] = 5;
        //     inst_operand.m_lsb_lst[7] = 3;
        //     inst_operand.m_connect[7] = false;
        //     // ["u[12:12]|", "u[7:7]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_JAL, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ADDIW
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "h[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "h[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "h[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_ADDIW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_LI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_LI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ADDI16SP
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 4;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[4:2]", "u[12:5]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 5;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[4:2]", "u[12:5]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::ADDI16SP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_LUI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_LUI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SRLI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SRLI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SRLI64
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SRLI64, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SRAI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SRAI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SRAI64
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SRAI64, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ANDI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeHex;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["cr[9:7]", "u[12:12]|", "h[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_ANDI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SUB
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 4;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SUB, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_XOR
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 4;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_XOR, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_OR
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 4;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_OR, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_AND
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[1] = 4;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[9:7]", "cr[4:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_AND, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SUBW
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "r[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 6;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[9:7]", "r[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SUBW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ADDW
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "r[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 6;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["cr[9:7]", "r[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_ADDW, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_J
        //     inst_operand.m_size = 8;
        //     inst_operand.m_type_lst[0] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[0] = 12;
        //     inst_operand.m_lsb_lst[0] = 12;
        //     inst_operand.m_connect[0] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 8;
        //     inst_operand.m_lsb_lst[1] = 8;
        //     inst_operand.m_connect[1] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 10;
        //     inst_operand.m_lsb_lst[2] = 9;
        //     inst_operand.m_connect[2] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 6;
        //     inst_operand.m_lsb_lst[3] = 6;
        //     inst_operand.m_connect[3] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 7;
        //     inst_operand.m_lsb_lst[4] = 7;
        //     inst_operand.m_connect[4] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[5] = 2;
        //     inst_operand.m_lsb_lst[5] = 2;
        //     inst_operand.m_connect[5] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[6] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[6] = 11;
        //     inst_operand.m_lsb_lst[6] = 11;
        //     inst_operand.m_connect[6] = true;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //     inst_operand.m_type_lst[7] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[7] = 5;
        //     inst_operand.m_lsb_lst[7] = 3;
        //     inst_operand.m_connect[7] = false;
        //     // ["u[12:12]|", "u[8:8]|", "u[10:9]|", "u[6:6]|", "u[7:7]|", "u[2:2]|", "u[11:11]|", "u[5:3]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_J, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_BEQZ
        //     inst_operand.m_size = 6;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 2;
        //     inst_operand.m_lsb_lst[3] = 2;
        //     inst_operand.m_connect[3] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 11;
        //     inst_operand.m_lsb_lst[4] = 10;
        //     inst_operand.m_connect[4] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[5] = 4;
        //     inst_operand.m_lsb_lst[5] = 3;
        //     inst_operand.m_connect[5] = false;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_BEQZ, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_BNEZ
        //     inst_operand.m_size = 6;
        //     inst_operand.m_type_lst[0] = OperandType::TypeCompactReg;
        //     inst_operand.m_msb_lst[0] = 9;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 5;
        //     inst_operand.m_connect[2] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 2;
        //     inst_operand.m_lsb_lst[3] = 2;
        //     inst_operand.m_connect[3] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[4] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[4] = 11;
        //     inst_operand.m_lsb_lst[4] = 10;
        //     inst_operand.m_connect[4] = true;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //     inst_operand.m_type_lst[5] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[5] = 4;
        //     inst_operand.m_lsb_lst[5] = 3;
        //     inst_operand.m_connect[5] = false;
        //     // ["cr[9:7]", "u[12:12]|", "u[6:5]|", "u[2:2]|", "u[11:10]|", "u[4:3]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_BNEZ, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SLLI
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[12:12]|", "u[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 12;
        //     inst_operand.m_lsb_lst[1] = 12;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[12:12]|", "u[6:2]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 6;
        //     inst_operand.m_lsb_lst[2] = 2;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[11:7]", "u[12:12]|", "u[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SLLI, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FLDSP
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 4;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 12;
        //     inst_operand.m_connect[2] = true;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 6;
        //     inst_operand.m_lsb_lst[3] = 5;
        //     inst_operand.m_connect[3] = false;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FLDSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_LWSP
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 3;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 12;
        //     inst_operand.m_connect[2] = true;
        //     // ["r[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 6;
        //     inst_operand.m_lsb_lst[3] = 4;
        //     inst_operand.m_connect[3] = false;
        //     // ["r[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_LWSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FLWSP
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 3;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = true;
        //     // ["f[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 12;
        //     inst_operand.m_connect[2] = true;
        //     // ["f[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 6;
        //     inst_operand.m_lsb_lst[3] = 4;
        //     inst_operand.m_connect[3] = false;
        //     // ["f[11:7]", "u[3:2]|", "u[12:12]|", "u[6:4]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FLWSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_LDSP
        //     inst_operand.m_size = 4;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 4;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 12;
        //     inst_operand.m_connect[2] = true;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //     inst_operand.m_type_lst[3] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[3] = 6;
        //     inst_operand.m_lsb_lst[3] = 5;
        //     inst_operand.m_connect[3] = false;
        //     // ["r[11:7]", "u[4:2]|", "u[12:12]|", "u[6:5]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_LDSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_JR
        //     inst_operand.m_size = 1;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_JR, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_MV
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 6;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_MV, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_EBREAK
        //     inst_operand.m_size = 0;
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_EBREAK, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_JALR
        //     inst_operand.m_size = 1;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_JALR, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_ADD
        //     inst_operand.m_size = 2;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 11;
        //     inst_operand.m_lsb_lst[0] = 7;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[11:7]", "r[6:2]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[1] = 6;
        //     inst_operand.m_lsb_lst[1] = 2;
        //     inst_operand.m_connect[1] = false;
        //     // ["r[11:7]", "r[6:2]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_ADD, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FSDSP
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 6;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[6:2]", "u[9:7]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = true;
        //     // ["f[6:2]", "u[9:7]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 10;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[6:2]", "u[9:7]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_FSDSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SWSP
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 6;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[6:2]", "u[8:7]|", "u[12:9]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 8;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[6:2]", "u[8:7]|", "u[12:9]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 9;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[6:2]", "u[8:7]|", "u[12:9]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SWSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_FSWSP
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeFreg;
        //     inst_operand.m_msb_lst[0] = 6;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["f[6:2]", "u[9:7]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = true;
        //     // ["f[6:2]", "u[9:7]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 10;
        //     inst_operand.m_connect[2] = false;
        //     // ["f[6:2]", "u[9:7]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::FSWSP, inst_operand);
        // }
        // {
	    //     let mut inst_operand = OperandInfo::new();
        //
        //     // InstId_t::INST_ID_C_SDSP
        //     inst_operand.m_size = 3;
        //     inst_operand.m_type_lst[0] = OperandType::TypeXReg;
        //     inst_operand.m_msb_lst[0] = 6;
        //     inst_operand.m_lsb_lst[0] = 2;
        //     inst_operand.m_connect[0] = false;
        //     // ["r[6:2]", "u[9:7]|", "u[12:10]"]
        //     inst_operand.m_type_lst[1] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[1] = 9;
        //     inst_operand.m_lsb_lst[1] = 7;
        //     inst_operand.m_connect[1] = true;
        //     // ["r[6:2]", "u[9:7]|", "u[12:10]"]
        //     inst_operand.m_type_lst[2] = OperandType::TypeUnSign;
        //     inst_operand.m_msb_lst[2] = 12;
        //     inst_operand.m_lsb_lst[2] = 10;
        //     inst_operand.m_connect[2] = false;
        //     // ["r[6:2]", "u[9:7]|", "u[12:10]"]
        //
	    //     self.m_inst_operand_map.insert(RiscvInstId::C_SDSP, inst_operand);
        // }
    }
}
