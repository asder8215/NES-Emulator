//! Contains the definition CPU and load, run, and reset functionalities
//! on game inserts

use crate::{mem::Mem, opcodes::{OpCode}};

pub struct CPU {
    pub register_a: u8,         // accumulator CPU register (temp storage of data for calc)
    pub register_x: u8,         // another register to store data
    pub register_y: u8,         // another register to store data
    // pub stack_register: u8,     // this is the P register in 6502
    pub stack_pointer: u8,      // 8 bit addr for the stack pointer
    pub status: u8,             // processor status register
    pub program_counter: u16,   // the pc (keeps track of our curr pos in the program)
    pub(crate) memory: [u8; 0xFFFF]
}

impl CPU {
    /// Instantiates the CPU (all set to 0)
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            // stack_register: 0,
            stack_pointer: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF]
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
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }
    
    #[inline]
    pub fn load_and_run(&mut self, program: Vec<u8>) {
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
                    crate::opcodes::OpCodeName::ADC => self.adc(&opcode_struct.mode),
                    crate::opcodes::OpCodeName::AND => self.and(&opcode_struct.mode),
                    crate::opcodes::OpCodeName::ASL => todo!(),
                    crate::opcodes::OpCodeName::BCC => todo!(),
                    crate::opcodes::OpCodeName::BCS => todo!(),
                    crate::opcodes::OpCodeName::BEQ => todo!(),
                    crate::opcodes::OpCodeName::BIT => todo!(),
                    crate::opcodes::OpCodeName::BMI => todo!(),
                    crate::opcodes::OpCodeName::BNE => todo!(),
                    crate::opcodes::OpCodeName::BPL => todo!(),
                    crate::opcodes::OpCodeName::BRK => return,
                    crate::opcodes::OpCodeName::BVC => todo!(),
                    crate::opcodes::OpCodeName::BVS => todo!(),
                    crate::opcodes::OpCodeName::CLC => todo!(),
                    crate::opcodes::OpCodeName::CLD => todo!(),
                    crate::opcodes::OpCodeName::CLI => todo!(),
                    crate::opcodes::OpCodeName::CLV => todo!(),
                    crate::opcodes::OpCodeName::CMP => todo!(),
                    crate::opcodes::OpCodeName::CPX => todo!(),
                    crate::opcodes::OpCodeName::CPY => todo!(),
                    crate::opcodes::OpCodeName::DEC => todo!(),
                    crate::opcodes::OpCodeName::DEX => todo!(),
                    crate::opcodes::OpCodeName::DEY => todo!(),
                    crate::opcodes::OpCodeName::EOR => todo!(),
                    crate::opcodes::OpCodeName::INC => todo!(),
                    crate::opcodes::OpCodeName::INX => self.inx(),
                    crate::opcodes::OpCodeName::INY => todo!(),
                    crate::opcodes::OpCodeName::JMP => todo!(),
                    crate::opcodes::OpCodeName::JSR => todo!(),
                    crate::opcodes::OpCodeName::LDA => self.lda(&opcode_struct.mode),
                    crate::opcodes::OpCodeName::LDX => todo!(),
                    crate::opcodes::OpCodeName::LDY => todo!(),
                    crate::opcodes::OpCodeName::LSR => todo!(),
                    crate::opcodes::OpCodeName::NOP => {}, // does nothing lol
                    crate::opcodes::OpCodeName::ORA => todo!(),
                    crate::opcodes::OpCodeName::PHA => todo!(),
                    crate::opcodes::OpCodeName::PHP => todo!(),
                    crate::opcodes::OpCodeName::PLA => todo!(),
                    crate::opcodes::OpCodeName::PLP => todo!(),
                    crate::opcodes::OpCodeName::ROL => todo!(),
                    crate::opcodes::OpCodeName::ROR => todo!(),
                    crate::opcodes::OpCodeName::RTI => todo!(),
                    crate::opcodes::OpCodeName::RTS => todo!(),
                    crate::opcodes::OpCodeName::SBC => todo!(),
                    crate::opcodes::OpCodeName::SEC => todo!(),
                    crate::opcodes::OpCodeName::SED => todo!(),
                    crate::opcodes::OpCodeName::SEI => todo!(),
                    crate::opcodes::OpCodeName::STA => todo!(),
                    crate::opcodes::OpCodeName::STX => todo!(),
                    crate::opcodes::OpCodeName::STY => todo!(),
                    crate::opcodes::OpCodeName::TAX => self.tax(),
                    crate::opcodes::OpCodeName::TAY => todo!(),
                    crate::opcodes::OpCodeName::TSX => todo!(),
                    crate::opcodes::OpCodeName::TXA => todo!(),
                    crate::opcodes::OpCodeName::TXS => todo!(),
                    crate::opcodes::OpCodeName::TYA => todo!(),
                }
                // move PC to the next instruction to process
                self.program_counter += (opcode_struct.len - 1) as u16;
            } else {
                panic!("Illegal instruction {} reached at address {:#x}", opcode, self.program_counter)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // LDA TESTS
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0x05, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0xa9, 0x00, 0x00]);
        cpu.reset();
        cpu.run();
        
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    // ===============

    // LDA, TAX, INX, TESTS
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        cpu.reset();
        cpu.run();
        
        assert_eq!(cpu.register_x, 0xc1)
    }

    // INX TESTS
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();

        cpu.register_x = 0xff;
        
        cpu.run();
        
        assert_eq!(cpu.register_x, 1)
    }

    // ===============

    // == ADC TESTS ==
    #[test]
    fn test_adc_no_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0x69, 0x10, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;
        
        cpu.run();
        
        assert_eq!(cpu.register_a, 0x60);
        assert!(cpu.status & 0b0000_0001 == 0);
        assert!(cpu.status & 0b0100_0000 == 0);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_adc_carry_out_and_overflow() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0x69, 0x90, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0xd0;
        
        cpu.run();
        
        assert_eq!(cpu.register_a, 0x60);
        assert!(cpu.status & 0b0000_0001 == 1);
        assert!(cpu.status & 0b0100_0000 == 0b0100_0000);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_adc_carry_out_and_no_overflow() {
        let mut cpu = CPU::new();

        cpu.load(vec![0x69, 0xd0, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0x50;
        cpu.status = cpu.status | 0b1; // set carry in
        
        cpu.run();
        
        assert_eq!(cpu.register_a, 0x21);
        assert!(cpu.status & 0b0000_0001 == 1);
        assert!(cpu.status & 0b0100_0000 == 0); // can't overflow if we're adding pos and neg
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_adc_no_carry_out_and_overflow() {
        let mut cpu = CPU::new();

        cpu.load(vec![0x69, 0x50, 0x00]);
        cpu.reset();

        cpu.register_a = 0x50;
        cpu.status = cpu.status | 0b1; // set carry in

        cpu.run();
        
        assert_eq!(cpu.register_a, 0xa1);
        assert!(cpu.status & 0b0000_0001 == 0);
        assert!(cpu.status & 0b0100_0000 == 0b0100_0000);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_adc_carry_in_sets_carry_out() {
        let mut cpu = CPU::new();

        cpu.load(vec![0x69, 0x9f, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0x60;
        cpu.status = cpu.status | 0b1; // set carry in

        cpu.run();

        assert_eq!(cpu.register_a, 0x0);
        assert!(cpu.status & 0b0000_0001 == 1);
        assert!(cpu.status & 0b0100_0000 == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_adc_carry_in_sets_carry_out_2() {
        let mut cpu = CPU::new();

        cpu.load(vec![0x69, 0x00, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0xFF;
        cpu.status = cpu.status | 0b1; // set carry in

        cpu.run();
        
        assert_eq!(cpu.register_a, 0x0);
        assert!(cpu.status & 0b0000_0001 == 1);
        assert!(cpu.status & 0b0100_0000 == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_adc_carry_in_sets_overflow() {
        let mut cpu = CPU::new();
        
        cpu.load(vec![0x69, 0x39, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0x46;
        cpu.status = cpu.status | 0b1; // set carry in
        
        cpu.run();

        assert_eq!(cpu.register_a, 0x80);
        assert!(cpu.status & 0b0000_0001 == 0);
        assert!(cpu.status & 0b0100_0000 == 0b0100_0000);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }
    // ===============

    // == AND TESTS ==

    #[test]
    fn test_and_non_zero_res() {
        let mut cpu = CPU::new();

        cpu.load(vec![0x29, 0x11, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0x01;

        cpu.run();

        assert_eq!(cpu.register_a, 0x01);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_and_zero_res() {
        let mut cpu = CPU::new();

        cpu.load(vec![0x29, 0x1F, 0x00]);
        cpu.reset();
        
        cpu.register_a = 0x00;

        cpu.run();

        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status & 0b1000_0000 == 0b00);
    }
    // ===============
}
