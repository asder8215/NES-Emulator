//! Contains the definition CPU and load, run, and reset functionalities
//! on game inserts

pub mod addressing_mode;
pub mod instructions;
pub mod mem;
pub mod opcodes;
pub mod processor_status;

use mem::Mem;
use opcodes::{OpCode, OpCodeName};

/// The stack pointer offsets from this
/// base address
const STACK: u16 = 0x100;
const STACK_RESET: u8 = 0xfd;

pub struct CPU {
    /// accumulator CPU register
    pub register_a: u8,
    /// another register to store data
    pub register_x: u8,
    /// another register to store data
    pub register_y: u8,
    /// 8 bit addr for the stack pointer
    pub stack_pointer: u8,
    /// processor status register
    pub status: u8,
    /// the pc (keeps track of our curr pos in the program)
    pub program_counter: u16,
    /// contains the CPU's Memory Map
    /// * RAM - [0x0000 ... 0x2000]
    /// * PPU, APU, GamePads, etc: [0x4020 ... 0x6000]
    /// * Special storage for cartridges: [0x4020 ... 0x6000]
    /// * RAM Space for Cartridge: [0x6000 ... 0x8000]
    /// * PRG ROM: [0x8000 ... 0xFFFF]
    pub(crate) memory: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    /// Instantiates the CPU (all set to 0)
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: STACK_RESET,
            status: 0b10_0100, // decimal and interrupt disable flag is turned on
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    /// This is the NES mechanism to mark where the CPU should start execution.
    /// When a new cartridge is inserted, the CPU receives a reset interrupt signal
    /// to instruct to CPU on the following:
    /// * reset all states (registers and status)
    /// * set the PC to the 16-bit address that is stored at 0xFFFC
    #[inline]
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = 0b10_0100;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    /// Copies the program data into Program ROM (PRG ROM) space of memory.
    ///
    /// PRG ROM space refers to [0x8000 ... 0xFFFF] region.
    ///
    /// This is where cartridges load their data into the NES's memory.
    ///
    /// We mark the address 0xFFFC and 0xFFFD with the data 0x8000 because
    /// when a different cartridge is inserted or if there is some sort of retry
    /// logic within the cartridge, then the PC needs to relearn that it should
    /// reading from 0x8000 again.
    #[inline]
    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    #[doc(hidden)]
    #[inline]
    pub fn test_load(&mut self, program: &[u8]) {
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x0600);
    }

    /// Loads the program into memory, reset all registers and PC to default state,
    /// and runs instruction in the ROM
    #[inline]
    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    /// Runs in an infinite loop (until BRK) to do the following:
    /// - Fetch next exec instruction from instruction mem
    /// - Decode instruction
    /// - Exec instruction
    /// - Rinse and repeat
    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter = self.program_counter.wrapping_add(1);
            if let Some(opcode_struct) = OpCode::get(opcode) {
                match opcode_struct.mnemonic {
                    OpCodeName::ADC => self.adc(opcode_struct.mode),
                    OpCodeName::AND => self.and(opcode_struct.mode),
                    OpCodeName::ASL => self.asl(opcode_struct.mode),
                    OpCodeName::BCC => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Carry),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BCS => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Carry),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BEQ => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Zero),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BIT => self.bit(opcode_struct.mode),
                    OpCodeName::BMI => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Negative),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BNE => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Zero),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BPL => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Negative),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BRK => {
                        // By technicality on 6502, the PC should not be incremented
                        // when a BRK instruction is reached. Since I don't want
                        // to rewrite this code to have incrementation occur everywhere
                        // except BRK, I'll just decrement the PC by 1 here.
                        self.program_counter = self.program_counter.wrapping_sub(1);
                        return;
                    }
                    OpCodeName::BVC => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Overflow),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BVS => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Overflow),
                        opcode_struct.mode,
                    ),
                    OpCodeName::CLC => self.clear(processor_status::ProcessorStatus::Carry),
                    OpCodeName::CLD => self.clear(processor_status::ProcessorStatus::Decimal),
                    OpCodeName::CLI => {
                        self.clear(processor_status::ProcessorStatus::InterruptDisable)
                    }
                    OpCodeName::CLV => self.clear(processor_status::ProcessorStatus::Overflow),
                    OpCodeName::CMP => self.cmp(opcode_struct.mode),
                    OpCodeName::CPX => self.cpx(opcode_struct.mode),
                    OpCodeName::CPY => self.cpy(opcode_struct.mode),
                    OpCodeName::DEC => self.dec(opcode_struct.mode),
                    OpCodeName::DEX => self.dex(),
                    OpCodeName::DEY => self.dey(),
                    OpCodeName::EOR => self.eor(opcode_struct.mode),
                    OpCodeName::INC => self.inc(opcode_struct.mode),
                    OpCodeName::INX => self.inx(),
                    OpCodeName::INY => self.iny(),
                    OpCodeName::JMP => {
                        self.jmp(opcode_struct.mode);
                        // the reason why we continue is bc
                        // jump instructions are not *relative*
                        // to the next instruction, so we do not
                        // add the opcode_struct.len() - 1 bytes
                        // to the PC
                        continue;
                    }
                    OpCodeName::JSR => {
                        self.jsr(opcode_struct.mode);
                        continue;
                    }
                    OpCodeName::LDA => self.lda(opcode_struct.mode),
                    OpCodeName::LDX => self.ldx(opcode_struct.mode),
                    OpCodeName::LDY => self.ldy(opcode_struct.mode),
                    OpCodeName::LSR => self.lsr(opcode_struct.mode),
                    OpCodeName::NOP => {} // does nothing lol
                    OpCodeName::ORA => self.ora(opcode_struct.mode),
                    OpCodeName::PHA => self.pha(),
                    OpCodeName::PHP => self.php(),
                    OpCodeName::PLA => self.pla(),
                    OpCodeName::PLP => self.plp(),
                    OpCodeName::ROL => self.rol(opcode_struct.mode),
                    OpCodeName::ROR => self.ror(opcode_struct.mode),
                    OpCodeName::RTI => self.rti(),
                    OpCodeName::RTS => self.rts(),
                    OpCodeName::SBC => self.sbc(opcode_struct.mode),
                    OpCodeName::SEC => self.sec(),
                    OpCodeName::SED => self.sed(),
                    OpCodeName::SEI => self.sei(),
                    OpCodeName::STA => self.sta(opcode_struct.mode),
                    OpCodeName::STX => self.stx(opcode_struct.mode),
                    OpCodeName::STY => self.sty(opcode_struct.mode),
                    OpCodeName::TAX => self.tax(),
                    OpCodeName::TAY => self.tay(),
                    OpCodeName::TSX => self.tsx(),
                    OpCodeName::TXA => self.txa(),
                    OpCodeName::TXS => self.txs(),
                    OpCodeName::TYA => self.tya(),
                }
                // move PC to the next instruction to process
                self.program_counter = self
                    .program_counter
                    .wrapping_add((opcode_struct.len - 1) as u16);
            } else {
                panic!(
                    "Illegal instruction {} reached at address {:#x}",
                    opcode, self.program_counter
                )
            }
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn test_run(&mut self) {
        self.test_run_with_callback(|_| {});
    }

    #[doc(hidden)]
    #[inline]
    pub fn test_run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter = self.program_counter.wrapping_add(1);
            // println!(
            //     "Reading instruction: {:#x} with PC at: {:#x}",
            //     opcode, self.program_counter
            // );
            if let Some(opcode_struct) = OpCode::get(opcode) {
                match opcode_struct.mnemonic {
                    OpCodeName::ADC => self.adc(opcode_struct.mode),
                    OpCodeName::AND => self.and(opcode_struct.mode),
                    OpCodeName::ASL => self.asl(opcode_struct.mode),
                    OpCodeName::BCC => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Carry),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BCS => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Carry),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BEQ => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Zero),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BIT => self.bit(opcode_struct.mode),
                    OpCodeName::BMI => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Negative),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BNE => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Zero),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BPL => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Negative),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BRK => {
                        // By technicality on 6502, the PC should not be incremented
                        // when a BRK instruction is reached. Since I don't want
                        // to rewrite this code to have incrementation occur everywhere
                        // except BRK, I'll just decrement the PC by 1 here.
                        self.program_counter = self.program_counter.wrapping_sub(1);
                        return;
                    }
                    OpCodeName::BVC => self.branch(
                        !self.is_status_flag_set(processor_status::ProcessorStatus::Overflow),
                        opcode_struct.mode,
                    ),
                    OpCodeName::BVS => self.branch(
                        self.is_status_flag_set(processor_status::ProcessorStatus::Overflow),
                        opcode_struct.mode,
                    ),
                    OpCodeName::CLC => self.clear(processor_status::ProcessorStatus::Carry),
                    OpCodeName::CLD => self.clear(processor_status::ProcessorStatus::Decimal),
                    OpCodeName::CLI => {
                        self.clear(processor_status::ProcessorStatus::InterruptDisable)
                    }
                    OpCodeName::CLV => self.clear(processor_status::ProcessorStatus::Overflow),
                    OpCodeName::CMP => self.cmp(opcode_struct.mode),
                    OpCodeName::CPX => self.cpx(opcode_struct.mode),
                    OpCodeName::CPY => self.cpy(opcode_struct.mode),
                    OpCodeName::DEC => self.dec(opcode_struct.mode),
                    OpCodeName::DEX => self.dex(),
                    OpCodeName::DEY => self.dey(),
                    OpCodeName::EOR => self.eor(opcode_struct.mode),
                    OpCodeName::INC => self.inc(opcode_struct.mode),
                    OpCodeName::INX => self.inx(),
                    OpCodeName::INY => self.iny(),
                    OpCodeName::JMP => {
                        self.jmp(opcode_struct.mode);
                        // the reason why we continue is bc
                        // jump instructions are not *relative*
                        // to the next instruction, so we do not
                        // add the opcode_struct.len() - 1 bytes
                        // to the PC
                        continue;
                    }
                    OpCodeName::JSR => {
                        self.jsr(opcode_struct.mode);
                        continue;
                    }
                    OpCodeName::LDA => self.lda(opcode_struct.mode),
                    OpCodeName::LDX => self.ldx(opcode_struct.mode),
                    OpCodeName::LDY => self.ldy(opcode_struct.mode),
                    OpCodeName::LSR => self.lsr(opcode_struct.mode),
                    OpCodeName::NOP => {} // does nothing lol
                    OpCodeName::ORA => self.ora(opcode_struct.mode),
                    OpCodeName::PHA => self.pha(),
                    OpCodeName::PHP => self.php(),
                    OpCodeName::PLA => self.pla(),
                    OpCodeName::PLP => self.plp(),
                    OpCodeName::ROL => self.rol(opcode_struct.mode),
                    OpCodeName::ROR => self.ror(opcode_struct.mode),
                    OpCodeName::RTI => self.rti(),
                    OpCodeName::RTS => self.rts(),
                    OpCodeName::SBC => self.sbc(opcode_struct.mode),
                    OpCodeName::SEC => self.sec(),
                    OpCodeName::SED => self.sed(),
                    OpCodeName::SEI => self.sei(),
                    OpCodeName::STA => self.sta(opcode_struct.mode),
                    OpCodeName::STX => self.stx(opcode_struct.mode),
                    OpCodeName::STY => self.sty(opcode_struct.mode),
                    OpCodeName::TAX => self.tax(),
                    OpCodeName::TAY => self.tay(),
                    OpCodeName::TSX => self.tsx(),
                    OpCodeName::TXA => self.txa(),
                    OpCodeName::TXS => self.txs(),
                    OpCodeName::TYA => self.tya(),
                }
                // move PC to the next instruction to process
                self.program_counter = self
                    .program_counter
                    .wrapping_add((opcode_struct.len - 1) as u16);
            } else {
                panic!(
                    "Illegal instruction {} reached at address {:#x}",
                    opcode, self.program_counter
                )
            }
            callback(self);
        }
    }
}
