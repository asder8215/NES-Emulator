//! Contains the definition CPU and load, run, and reset functionalities
//! on game inserts

pub mod addressing_mode;
pub mod instructions;
pub mod mem;
pub mod opcodes;
pub mod processor_status;

use mem::Mem;
use opcodes::{OpCode, OpCodeName};

pub struct CPU {
    pub register_a: u8,    // accumulator CPU register (temp storage of data for calc)
    pub register_x: u8,    // another register to store data
    pub register_y: u8,    // another register to store data
    pub stack_pointer: u8, // 8 bit addr for the stack pointer
    pub status: u8,        // processor status register
    pub program_counter: u16, // the pc (keeps track of our curr pos in the program)
    pub(crate) memory: [u8; 0xFFFF],
}

// Processor Status
// 7  bit  0
// ---- ----
// NV1B DIZC
// |||| ||||
// |||| |||+- Carry
// |||| ||+-- Zero
// |||| |+--- Interrupt Disable
// |||| +---- Decimal
// |||+------ (No CPU effect; see: the B flag)
// ||+------- (No CPU effect; always pushed as 1)
// |+-------- Overflow
// +--------- Negative
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
            stack_pointer: 0,
            status: 0,
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
        self.status = 0;

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
            self.program_counter += 1;

            if let Some(opcode_struct) = OpCode::get(opcode) {
                match opcode_struct.mnemonic {
                    OpCodeName::ADC => self.adc(&opcode_struct.mode),
                    OpCodeName::AND => self.and(&opcode_struct.mode),
                    OpCodeName::ASL => self.asl(&opcode_struct.mode),
                    OpCodeName::BCC => todo!(),
                    OpCodeName::BCS => todo!(),
                    OpCodeName::BEQ => todo!(),
                    OpCodeName::BIT => todo!(),
                    OpCodeName::BMI => todo!(),
                    OpCodeName::BNE => todo!(),
                    OpCodeName::BPL => todo!(),
                    OpCodeName::BRK => return,
                    OpCodeName::BVC => todo!(),
                    OpCodeName::BVS => todo!(),
                    OpCodeName::CLC => todo!(),
                    OpCodeName::CLD => todo!(),
                    OpCodeName::CLI => todo!(),
                    OpCodeName::CLV => todo!(),
                    OpCodeName::CMP => todo!(),
                    OpCodeName::CPX => todo!(),
                    OpCodeName::CPY => todo!(),
                    OpCodeName::DEC => todo!(),
                    OpCodeName::DEX => todo!(),
                    OpCodeName::DEY => todo!(),
                    OpCodeName::EOR => todo!(),
                    OpCodeName::INC => todo!(),
                    OpCodeName::INX => self.inx(),
                    OpCodeName::INY => todo!(),
                    OpCodeName::JMP => todo!(),
                    OpCodeName::JSR => todo!(),
                    OpCodeName::LDA => self.lda(&opcode_struct.mode),
                    OpCodeName::LDX => todo!(),
                    OpCodeName::LDY => todo!(),
                    OpCodeName::LSR => todo!(),
                    OpCodeName::NOP => {} // does nothing lol
                    OpCodeName::ORA => todo!(),
                    OpCodeName::PHA => todo!(),
                    OpCodeName::PHP => todo!(),
                    OpCodeName::PLA => todo!(),
                    OpCodeName::PLP => todo!(),
                    OpCodeName::ROL => todo!(),
                    OpCodeName::ROR => todo!(),
                    OpCodeName::RTI => todo!(),
                    OpCodeName::RTS => todo!(),
                    OpCodeName::SBC => todo!(),
                    OpCodeName::SEC => todo!(),
                    OpCodeName::SED => todo!(),
                    OpCodeName::SEI => todo!(),
                    OpCodeName::STA => todo!(),
                    OpCodeName::STX => todo!(),
                    OpCodeName::STY => todo!(),
                    OpCodeName::TAX => self.tax(),
                    OpCodeName::TAY => todo!(),
                    OpCodeName::TSX => todo!(),
                    OpCodeName::TXA => todo!(),
                    OpCodeName::TXS => todo!(),
                    OpCodeName::TYA => todo!(),
                }
                // move PC to the next instruction to process
                self.program_counter += (opcode_struct.len - 1) as u16;
            } else {
                panic!(
                    "Illegal instruction {} reached at address {:#x}",
                    opcode, self.program_counter
                )
            }
        }
    }
}
