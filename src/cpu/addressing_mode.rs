//! Contains an enum with addressing modes that an op code can
//! use and provides a method to returning the target address that
//! the op code retrieves a value from

use super::CPU;
use crate::Mem;

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
    pub(crate) fn get_operand_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implicit => {
                // certain instructions do not need a target
                // addr in this case because they are going to
                // modify certain CPU fields like processor status
                unreachable!()
            }
            AddressingMode::Accumulator => {
                // note that certain instruction can operate
                // directly on the accumulator register (i.e.
                // LSR or ROR), so they don't need a target addr
                unreachable!()
            }
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Relative => {
                // Important to interpret this input as an i8 because a branch instruction
                // is relative to the current position of the program counter
                // if you're doing a for loop or a while loop, you're moving the PC
                // back a certain amount of position
                let addr = self.mem_read(self.program_counter) as i8;

                // casting a i8 -> u16 keeps the signed offset of negative values (i.e. -16i8 -> 65520 in u16
                // whille -16i8 in u8 is 240u8 -> 240 in u16)
                self.program_counter.wrapping_add(addr as u16)
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => {
                let pos = self.mem_read(self.program_counter) as u16;
                pos.wrapping_add(self.register_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let pos = self.mem_read(self.program_counter) as u16;
                pos.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect => {
                // we have a 16 bit ptr in memory
                let ptr = self.mem_read_u16(self.program_counter);
                // we have to point to the actual 16 bit target location
                self.mem_read_u16(ptr)
            }
            AddressingMode::IndirectX => {
                // contains an 8 bit addr in memory
                let base = self.mem_read(self.program_counter);
                // the target addr is located with base + register x
                // (points to the LSB byte of the addr)
                let ptr = base.wrapping_add(self.register_x);

                // Note the follow is NOT the same as a mem_read_u16 because
                // we are allowing wrapping add on a ptr that is a u8 which is
                // then transformed into a u16 value
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                // contains an 8 bit address that points to a 16 bit address in memory
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                // add with register y to fetch the target address
                deref_base.wrapping_add(self.register_y as u16)
            }
        }
    }
}
