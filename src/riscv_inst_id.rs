#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RiscvInstId {
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    LUI,
    AUIPC,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SW,
    SH,
    SB,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
    FENCE,
    FENCEI,
    SFENCEVMA,
    ECALL,
    EBREAK,
    MRET,
    SRET,
    URET,
    WFI,
    LWU,   // for RV64
    LD,    // for RV64
    SD,    // for RV64
    ADDIW, // for RV64
    SLLIW, // for RV64
    SRLIW, // for RV64
    SRAIW, // for RV64
    ADDW,  // for RV64
    SUBW,  // for RV64
    SLLW,  // for RV64
    SRLW,  // for RV64
    SRAW,  // for RV64
    MULW,  // for RV64
    DIVW,  // for RV64
    DIVUW, // for RV64
    REMW,  // for RV64
    REMUW, // for RV64
}
