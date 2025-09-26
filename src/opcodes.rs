//! Contains the definition of an OpCode and a static compile time
//! defined HashMap structure with all 56 instructions (and addressing mode
//! variants) 

use phf::phf_map;
use crate::addressing_mode::AddressingMode;

#[derive(Debug, Copy, Clone)]
/// Contains all op code mnemonics
pub(crate) enum OpCodeName {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
}

#[derive(Debug, Copy, Clone)]
/// Defines what an OpCode contains
pub(crate) struct OpCode {
    #[allow(dead_code)]
    /// signifies what op code it is (i.e. 0x00 is BRK)
    pub code: u8,
    /// mnemonic name of the OpCode
    pub mnemonic: OpCodeName,
    /// how many bytes does this OpCode take
    pub len: u8,
    #[allow(dead_code)]
    /// how many cycles does this OpCode run for
    pub cycles: u8,
    /// the addressing mode of the OpCode (i.e. Immediate, Zero Page, etc.)
    pub mode: AddressingMode,
}

impl OpCode {
    /// Instantiates the OpCode struct on compile time
    const fn new(code: u8, mnemonic: OpCodeName, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Self {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }

    /// Given an operation code value, returns the OpCode struct
    /// associating with it
    pub fn get(code: u8) -> Option<OpCode> {
        CPU_OPS_CODES.get(&[code]).cloned()
    }
}

/// Contains all CPU op codes in a compile time hashmap
/// Note to self: key cannot be u8, it has to be [u8; 1] (kinda silly tbh)
static CPU_OPS_CODES: phf::Map<[u8; 1], OpCode> = phf_map! {
    // ADC - Add with Carry
    [0x69] => OpCode::new(0x69, OpCodeName::ADC, 2, 2, AddressingMode::Immediate),
    [0x65] => OpCode::new(0x65, OpCodeName::ADC, 2, 3, AddressingMode::ZeroPage),
    [0x75] => OpCode::new(0x75, OpCodeName::ADC, 2, 4, AddressingMode::ZeroPageX),
    [0x6D] => OpCode::new(0x6D, OpCodeName::ADC, 3, 4, AddressingMode::Absolute),
    [0x7D] => OpCode::new(0x7D, OpCodeName::ADC, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0x79] => OpCode::new(0x79, OpCodeName::ADC, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0x61] => OpCode::new(0x61, OpCodeName::ADC, 2, 6, AddressingMode::IndirectX),
    [0x71] => OpCode::new(0x71, OpCodeName::ADC, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // AND - Logical AND
    [0x29] => OpCode::new(0x29, OpCodeName::AND, 2, 2, AddressingMode::Immediate),
    [0x25] => OpCode::new(0x25, OpCodeName::AND, 2, 3, AddressingMode::ZeroPage),
    [0x35] => OpCode::new(0x35, OpCodeName::AND, 2, 4, AddressingMode::ZeroPageX),
    [0x2D] => OpCode::new(0x2D, OpCodeName::AND, 3, 4, AddressingMode::Absolute),
    [0x3D] => OpCode::new(0x3D, OpCodeName::AND, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0x39] => OpCode::new(0x39, OpCodeName::AND, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0x21] => OpCode::new(0x21, OpCodeName::AND, 2, 6, AddressingMode::IndirectX),
    [0x31] => OpCode::new(0x31, OpCodeName::AND, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // ASL - Arithmetic Shift Left
    [0x0A] => OpCode::new(0x0A, OpCodeName::ASL, 1, 2, AddressingMode::Accumulator),
    [0x06] => OpCode::new(0x06, OpCodeName::ASL, 2, 5, AddressingMode::ZeroPage),
    [0x16] => OpCode::new(0x16, OpCodeName::ASL, 2, 6, AddressingMode::ZeroPageX),
    [0x0E] => OpCode::new(0x0E, OpCodeName::ASL, 3, 6, AddressingMode::Absolute),
    [0x1E] => OpCode::new(0x1E, OpCodeName::ASL, 3, 7, AddressingMode::AbsoluteX),

    // BCC - Branch if Carry Clear
    [0x90] => OpCode::new(0x90, OpCodeName::BCC, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BCS - Branch if Carry Set
    [0xB0] => OpCode::new(0xB0, OpCodeName::BCS, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BEQ - Branch if Equal
    [0xF0] => OpCode::new(0xF0, OpCodeName::BEQ, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BIT - Bit Test
    [0x24] => OpCode::new(0x24, OpCodeName::BIT, 2, 3, AddressingMode::ZeroPage),
    [0x2C] => OpCode::new(0x2C, OpCodeName::BIT, 3, 4, AddressingMode::Absolute),

    // BMI - Branch if Minus
    [0x30] => OpCode::new(0x30, OpCodeName::BMI, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BNE - Branch if Not Equal
    [0xD0] => OpCode::new(0xD0, OpCodeName::BNE, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BPL - Branch if Positive
    [0x10] => OpCode::new(0x10, OpCodeName::BPL, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BRK - Force Interrupt
    [0x00] => OpCode::new(0x00, OpCodeName::BRK, 1, 7, AddressingMode::Implicit),

    // BVC - Branch if Overflow Clear
    [0x50] => OpCode::new(0x50, OpCodeName::BVC, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // BVS - Branch if Overflow Set
    [0x70] => OpCode::new(0x70, OpCodeName::BVS, 2, 2 /*(+1) if branch succeeds +2 if to a new page*/, AddressingMode::Relative),

    // CLC - Clear Carry Flag
    [0x18] => OpCode::new(0x18, OpCodeName::CLC, 1, 2, AddressingMode::Implicit),

    // CLD - Clear Decimal Mode
    [0xD8] => OpCode::new(0xD8, OpCodeName::CLD, 1, 2, AddressingMode::Implicit),

    // CLI - Clear Interrupt Disable
    [0x58] => OpCode::new(0x58, OpCodeName::CLI, 1, 2, AddressingMode::Implicit),

    // CLV - Clear Overflow Flag
    [0xB8] => OpCode::new(0xB8, OpCodeName::CLV, 1, 2, AddressingMode::Implicit),

    // CMP - Compare
    [0xC9] => OpCode::new(0xC9, OpCodeName::CMP, 2, 2, AddressingMode::Immediate),
    [0xC5] => OpCode::new(0xC5, OpCodeName::CMP, 2, 3, AddressingMode::ZeroPage),
    [0xD5] => OpCode::new(0xD5, OpCodeName::CMP, 2, 4, AddressingMode::ZeroPageX),
    [0xCD] => OpCode::new(0xCD, OpCodeName::CMP, 3, 4, AddressingMode::Absolute),
    [0xDD] => OpCode::new(0xDD, OpCodeName::CMP, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0xD9] => OpCode::new(0xD9, OpCodeName::CMP, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0xC1] => OpCode::new(0xC1, OpCodeName::CMP, 2, 6, AddressingMode::IndirectX),
    [0xD1] => OpCode::new(0xD1, OpCodeName::CMP, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // CPX - Compare X Register
    [0xE0] => OpCode::new(0xE0, OpCodeName::CPX, 2, 2, AddressingMode::Immediate),
    [0xE4] => OpCode::new(0xE4, OpCodeName::CPX, 2, 3, AddressingMode::ZeroPage),
    [0xEC] => OpCode::new(0xEC, OpCodeName::CPX, 3, 4, AddressingMode::Absolute),

    // CPY - Compare Y Register
    [0xC0] => OpCode::new(0xC0, OpCodeName::CPY, 2, 2, AddressingMode::Immediate),
    [0xC4] => OpCode::new(0xC4, OpCodeName::CPY, 2, 3, AddressingMode::ZeroPage),
    [0xCC] => OpCode::new(0xCC, OpCodeName::CPY, 3, 4, AddressingMode::Absolute),

    // DEC - Decrement Memory
    [0xC6] => OpCode::new(0xC6, OpCodeName::DEC, 2, 5, AddressingMode::ZeroPage),
    [0xD6] => OpCode::new(0xD6, OpCodeName::DEC, 2, 6, AddressingMode::ZeroPageX),
    [0xCE] => OpCode::new(0xCE, OpCodeName::DEC, 3, 6, AddressingMode::Absolute),
    [0xDE] => OpCode::new(0xDE, OpCodeName::DEC, 3, 7, AddressingMode::AbsoluteX),

    // DEX - Decrement X Register
    [0xCA] => OpCode::new(0xCA, OpCodeName::DEX, 1, 2, AddressingMode::Implicit),

    // DEY - Decrement Y Register
    [0x88] => OpCode::new(0x88, OpCodeName::DEY, 1, 2, AddressingMode::Implicit),

    // EOR - Exclusive OR
    [0x49] => OpCode::new(0x49, OpCodeName::EOR, 2, 2, AddressingMode::Immediate),
    [0x45] => OpCode::new(0x45, OpCodeName::EOR, 2, 3, AddressingMode::ZeroPage),
    [0x55] => OpCode::new(0x55, OpCodeName::EOR, 2, 4, AddressingMode::ZeroPageX),
    [0x4D] => OpCode::new(0x4D, OpCodeName::EOR, 3, 4, AddressingMode::Absolute),
    [0x5D] => OpCode::new(0x5D, OpCodeName::EOR, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0x59] => OpCode::new(0x59, OpCodeName::EOR, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0x41] => OpCode::new(0x41, OpCodeName::EOR, 2, 6, AddressingMode::IndirectX),
    [0x51] => OpCode::new(0x51, OpCodeName::EOR, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // INC - Increment Memory
    [0xE6] => OpCode::new(0xE6, OpCodeName::INC, 2, 5, AddressingMode::ZeroPage),
    [0xF6] => OpCode::new(0xF6, OpCodeName::INC, 2, 6, AddressingMode::ZeroPageX),
    [0xEE] => OpCode::new(0xEE, OpCodeName::INC, 3, 6, AddressingMode::Absolute),
    [0xFE] => OpCode::new(0xFE, OpCodeName::INC, 3, 7, AddressingMode::AbsoluteX),

    // INX - Increment X Register
    [0xE8] => OpCode::new(0xE8, OpCodeName::INX, 1, 2, AddressingMode::Implicit),

    // INY - Increment Y Register
    [0xC8] => OpCode::new(0xC8, OpCodeName::INY, 1, 2, AddressingMode::Implicit),

    // JMP - Jump
    [0x4C] => OpCode::new(0x4C, OpCodeName::JMP, 3, 3, AddressingMode::Absolute),
    [0x6C] => OpCode::new(0x6C, OpCodeName::JMP, 3, 5, AddressingMode::Indirect),

    // JSR - Jump to Subroutine
    [0x20] => OpCode::new(0x20, OpCodeName::JSR, 3, 6, AddressingMode::Absolute),

    // LDA - Load Accumulator
    [0xA9] => OpCode::new(0xA9, OpCodeName::LDA, 2, 2, AddressingMode::Immediate),
    [0xA5] => OpCode::new(0xA5, OpCodeName::LDA, 2, 3, AddressingMode::ZeroPage),
    [0xB5] => OpCode::new(0xB5, OpCodeName::LDA, 2, 4, AddressingMode::ZeroPageX),
    [0xAD] => OpCode::new(0xAD, OpCodeName::LDA, 3, 4, AddressingMode::Absolute),
    [0xBD] => OpCode::new(0xBD, OpCodeName::LDA, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0xB9] => OpCode::new(0xB9, OpCodeName::LDA, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0xA1] => OpCode::new(0xA1, OpCodeName::LDA, 2, 6, AddressingMode::IndirectX),
    [0xB1] => OpCode::new(0xB1, OpCodeName::LDA, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // LDX - Load X Register
    [0xA2] => OpCode::new(0xA2, OpCodeName::LDX, 2, 2, AddressingMode::Immediate),
    [0xA6] => OpCode::new(0xA6, OpCodeName::LDX, 2, 3, AddressingMode::ZeroPage),
    [0xB6] => OpCode::new(0xB6, OpCodeName::LDX, 2, 4, AddressingMode::ZeroPageY),
    [0xAE] => OpCode::new(0xAE, OpCodeName::LDX, 3, 4, AddressingMode::Absolute),
    [0xBE] => OpCode::new(0xBE, OpCodeName::LDX, 3, 4/*(+1 if page crossed)*/, AddressingMode::AbsoluteY),

    // LDY - Load Y Register
    [0xA0] => OpCode::new(0xA0, OpCodeName::LDY, 2, 2, AddressingMode::Immediate),
    [0xA4] => OpCode::new(0xA4, OpCodeName::LDY, 2, 3, AddressingMode::ZeroPage),
    [0xB4] => OpCode::new(0xB4, OpCodeName::LDY, 2, 4, AddressingMode::ZeroPageX),
    [0xAC] => OpCode::new(0xAC, OpCodeName::LDY, 3, 4, AddressingMode::Absolute),
    [0xBC] => OpCode::new(0xBC, OpCodeName::LDY, 3, 4/*(+1 if page crossed)*/, AddressingMode::AbsoluteX),

    // LSR - Logical Shift Right
    [0x4A] => OpCode::new(0x4A, OpCodeName::LSR, 1, 2, AddressingMode::Accumulator),
    [0x46] => OpCode::new(0x46, OpCodeName::LSR, 2, 5, AddressingMode::ZeroPage),
    [0x56] => OpCode::new(0x56, OpCodeName::LSR, 2, 6, AddressingMode::ZeroPageX),
    [0x4E] => OpCode::new(0x4E, OpCodeName::LSR, 3, 6, AddressingMode::Absolute),
    [0x5E] => OpCode::new(0x5E, OpCodeName::LSR, 3, 7, AddressingMode::AbsoluteX),

    // NOP - No Operation
    [0xEA] => OpCode::new(0xEA, OpCodeName::NOP, 1, 2, AddressingMode::Implicit),

    // ORA - Logical Inclusive OR
    [0x09] => OpCode::new(0x09, OpCodeName::ORA, 2, 2, AddressingMode::Immediate),
    [0x05] => OpCode::new(0x05, OpCodeName::ORA, 2, 3, AddressingMode::ZeroPage),
    [0x15] => OpCode::new(0x15, OpCodeName::ORA, 2, 4, AddressingMode::ZeroPageX),
    [0x0D] => OpCode::new(0x0D, OpCodeName::ORA, 3, 4, AddressingMode::Absolute),
    [0x1D] => OpCode::new(0x1D, OpCodeName::ORA, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0x19] => OpCode::new(0x19, OpCodeName::ORA, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0x01] => OpCode::new(0x01, OpCodeName::ORA, 2, 6, AddressingMode::IndirectX),
    [0x11] => OpCode::new(0x11, OpCodeName::ORA, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // PHA - Push Accumulator
    [0x48] => OpCode::new(0x48, OpCodeName::PHA, 1, 3, AddressingMode::Implicit),

    // PHP - Push Processor Status
    [0x08] => OpCode::new(0x08, OpCodeName::PHP, 1, 3, AddressingMode::Implicit),

    // PLA - Pull Accumulator
    [0x68] => OpCode::new(0x68, OpCodeName::PLA, 1, 4, AddressingMode::Implicit),

    // PLP - Pull Processor Status
    [0x28] => OpCode::new(0x28, OpCodeName::PLP, 1, 4, AddressingMode::Implicit),

    // ROL - Rotate Left
    [0x2A] => OpCode::new(0x2A, OpCodeName::ROL, 1, 2, AddressingMode::Accumulator),
    [0x26] => OpCode::new(0x26, OpCodeName::ROL, 2, 5, AddressingMode::ZeroPage),
    [0x36] => OpCode::new(0x36, OpCodeName::ROL, 2, 6, AddressingMode::ZeroPageX),
    [0x2E] => OpCode::new(0x2E, OpCodeName::ROL, 3, 6, AddressingMode::Absolute),
    [0x3E] => OpCode::new(0x3E, OpCodeName::ROL, 3, 7, AddressingMode::AbsoluteX),

    // ROR - Rotate Right
    [0x6A] => OpCode::new(0x6A, OpCodeName::ROR, 1, 2, AddressingMode::Accumulator),
    [0x66] => OpCode::new(0x66, OpCodeName::ROR, 2, 5, AddressingMode::ZeroPage),
    [0x76] => OpCode::new(0x76, OpCodeName::ROR, 2, 6, AddressingMode::ZeroPageX),
    [0x6E] => OpCode::new(0x6E, OpCodeName::ROR, 3, 6, AddressingMode::Absolute),
    [0x7E] => OpCode::new(0x7E, OpCodeName::ROR, 3, 7, AddressingMode::AbsoluteX),

    // RTI - Return from Interrupt
    [0x40] => OpCode::new(0x40, OpCodeName::RTI, 1, 6, AddressingMode::Implicit),

    // RTS - Return from Subroutine
    [0x60] => OpCode::new(0x60, OpCodeName::RTS, 1, 6, AddressingMode::Implicit),

    // SBC - Subtract with Carry
    [0xE9] => OpCode::new(0xE9, OpCodeName::SBC, 2, 2, AddressingMode::Immediate),
    [0xE5] => OpCode::new(0xE5, OpCodeName::SBC, 2, 3, AddressingMode::ZeroPage),
    [0xF5] => OpCode::new(0xF5, OpCodeName::SBC, 2, 4, AddressingMode::ZeroPageX),
    [0xED] => OpCode::new(0xED, OpCodeName::SBC, 3, 4, AddressingMode::Absolute),
    [0xFD] => OpCode::new(0xFD, OpCodeName::SBC, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteX),
    [0xF9] => OpCode::new(0xF9, OpCodeName::SBC, 3, 4 /*+1 if page crossed*/, AddressingMode::AbsoluteY),
    [0xE1] => OpCode::new(0xE1, OpCodeName::SBC, 2, 6, AddressingMode::IndirectX),
    [0xF1] => OpCode::new(0xF1, OpCodeName::SBC, 2, 5 /*+1 if page crossed*/, AddressingMode::IndirectY),

    // SEC - Set Carry Flag
    [0x38] => OpCode::new(0x38, OpCodeName::SEC, 1, 2, AddressingMode::Implicit),

    // SED - Set Decimal Flag
    [0xF8] => OpCode::new(0xF8, OpCodeName::SED, 1, 2, AddressingMode::Implicit),

    // SEI - Set Interrupt Disable
    [0x78] => OpCode::new(0x78, OpCodeName::SEI, 1, 2, AddressingMode::Implicit),

    // STA - Store Accumulator
    [0x85] => OpCode::new(0x85, OpCodeName::STA, 2, 3, AddressingMode::ZeroPage),
    [0x95] => OpCode::new(0x95, OpCodeName::STA, 2, 4, AddressingMode::ZeroPageX),
    [0x8D] => OpCode::new(0x8D, OpCodeName::STA, 3, 4, AddressingMode::Absolute),
    [0x9D] => OpCode::new(0x9D, OpCodeName::STA, 3, 5, AddressingMode::AbsoluteX),
    [0x99] => OpCode::new(0x99, OpCodeName::STA, 3, 5, AddressingMode::AbsoluteY),
    [0x81] => OpCode::new(0x81, OpCodeName::STA, 2, 6, AddressingMode::IndirectX),
    [0x91] => OpCode::new(0x91, OpCodeName::STA, 2, 6, AddressingMode::IndirectY),

    // STX - Store X Register
    [0x86] => OpCode::new(0x86, OpCodeName::STX, 2, 3, AddressingMode::ZeroPage),
    [0x96] => OpCode::new(0x96, OpCodeName::STX, 2, 4, AddressingMode::ZeroPageY),
    [0x8E] => OpCode::new(0x8E, OpCodeName::STX, 3, 4, AddressingMode::Absolute),

    // STY - Store Y Register
    [0x84] => OpCode::new(0x84, OpCodeName::STY, 2, 3, AddressingMode::ZeroPage),
    [0x94] => OpCode::new(0x94, OpCodeName::STY, 2, 4, AddressingMode::ZeroPageX),
    [0x8C] => OpCode::new(0x8C, OpCodeName::STY, 3, 4, AddressingMode::Absolute),

    // TAX - Transfer Accumulator to X
    [0xAA] => OpCode::new(0xAA, OpCodeName::TAX, 1, 2, AddressingMode::Implicit),

    // TAY - Transfer Accumulator to Y
    [0xA8] => OpCode::new(0xA8, OpCodeName::TAY, 1, 2, AddressingMode::Implicit),

    // TSX - Transfer Stack Pointer to X
    [0xBA] => OpCode::new(0xBA, OpCodeName::TSX, 1, 2, AddressingMode::Implicit),

    // TXA - Transfer X to Accumulator
    [0x8A] => OpCode::new(0x8A, OpCodeName::TXA, 1, 2, AddressingMode::Implicit),

    // TXS - Transfer X to Stack Pointer
    [0x9A] => OpCode::new(0x9A, OpCodeName::TXS, 1, 2, AddressingMode::Implicit),

    // TYA - Transfer Y to Accumulator
    [0x98] => OpCode::new(0x98, OpCodeName::TYA, 1, 2, AddressingMode::Implicit)
};