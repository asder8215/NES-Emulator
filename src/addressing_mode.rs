//! Contains an enum with addressing modes that an op code can 
//! use and provides a method to returning the target address that
//! the op code retrieves a value from

use crate::{cpu::CPU, mem::Mem};

#[derive(Debug, Copy, Clone)]
/// See https://www.nesdev.org/obelisk-6502-guide/addressing.html#IMP
/// for details on what each addressing mode does
pub(crate) enum AddressingMode {
    Implicit, // aka Implied
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // aka Indexed Indirect
    IndirectY, // aka Indirect Indexed
}

impl CPU {
    /// Given an addressing mode for an op code, return the target address of 
    /// that the op code wants to operate on 
    pub(crate) fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => {
                // certain instructions do not need a target
                // addr in this case because they are going to
                // modify certain CPU fields like processor status
                unreachable!()
            },
            AddressingMode::Accumulator => {
                // note that certain instruction can operate
                // directly on the accumulator register (i.e.
                // LSR or ROR), so they don't need a target addr
                unreachable!()
            },
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            },
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            },
            AddressingMode::Relative => {
                let addr = self.mem_read(self.program_counter) as u16;
                // Is overflow something that a branch instruction do?
                let addr = self.program_counter.wrapping_add(addr);
                addr
            },
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => {
                let pos = self.mem_read(self.program_counter) as u16;
                let addr = pos.wrapping_add(self.register_x as u16);
                addr
            },
            AddressingMode::AbsoluteY => {
                let pos = self.mem_read(self.program_counter) as u16;
                let addr = pos.wrapping_add(self.register_y as u16);
                addr
            },
            AddressingMode::Indirect => {
                // we have a 16 bit ptr in memory
                let ptr = self.mem_read_u16(self.program_counter);

                // we have to point to the actual 16 bit target location
                let addr = self.mem_read_u16(ptr);

                addr
            },
            AddressingMode::IndirectX => {
                // contains an 8 bit addr in memory
                let base = self.mem_read(self.program_counter);
                
                // the target addr is located with base + register x
                // (points to the LSB byte of the addr)
                let ptr = base.wrapping_add(self.register_x);

                let addr = self.mem_read_u16(ptr as u16);
                addr
            },
            AddressingMode::IndirectY => {
                // contains an 8 bit address that points to a 16 bit address in memory
                let base = self.mem_read(self.program_counter);

                // grab the 16 bit addr
                let pos = self.mem_read_u16(base as u16);
                // add with register y to fetch the target address 
                let addr = pos.wrapping_add(self.register_y as u16);
                addr
            },
        }
    }
}