//! All 6502 instruction implementations are defined here for the CPU
//! These instruction are implemented in accordance with:
//! https://www.nesdev.org/obelisk-6502-guide/reference.html#

use crate::{addressing_mode::AddressingMode, cpu::CPU, mem::Mem};

impl CPU {
    /// ADC - Add with Carry
    ///
    /// Increments the accumulator register with the content of a memory location
    /// in combination with the carry bit. If unsigned overflow occurs, then carry bit
    /// is set. If signed overflow occurs, then the overflow flag is set.
    ///
    /// Ex:
    ///
    /// 0x50 + 0x50 = 0xa0 | 80 + 80 = 160 | 80 + 80 = -96 <-- pos + pos = negative from
    /// signed overflow
    ///
    /// 0xd0 + 0x90 = 0x60 | 208 + 144 = 352 (- 256) = 96 | -48 + -112 <-- neg + neg = pos
    /// from signed overflow (plus sets the carry bit)
    ///
    /// The overflow flag can be set accordingly based on what (input carry ^ output carry)
    /// results to
    #[inline]
    pub(crate) fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = self
            .register_a
            .wrapping_add(value)
            .wrapping_add(self.status & 0b0000_0001);

        self.update_overflow_flag(
            self.status & 0b0000_0001 == 0b0000_0001,
            self.register_a <= value,
        );

        if self.register_a <= value {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        self.update_zero_and_negative_flags(self.register_a);
    }

    /// AND - Logical AND
    ///
    /// Performs a logical AND with the accumulator register and the value given
    /// from memory, storing it back into the accumulator register. Sets the zero
    /// and negative flag as appropriate
    #[inline]
    pub(crate) fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a & value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// INX - Increment X Register
    ///
    /// Increments the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// LDA - Load Accumulator
    ///
    /// Takes in an accumulator value and sets the zero and negative
    /// flag as appropriate
    #[inline]
    pub(crate) fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// TAX - Transfer of Accumulator to X
    ///
    /// Copies the content of the accumulator register into the X register
    /// and sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// Sets the zero and negative flag accordingly based on the result
    /// (coming from a register)
    ///
    /// https://www.nesdev.org/obelisk-6502-guide/reference.html
    ///
    /// https://www.nesdev.org/wiki/Status_flags
    #[inline]
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010; // preserve prior status; set ZF to 1
        } else {
            self.status = self.status & 0b1111_1101; // preserve prior status; reset ZF to 0
        }

        if result & 0b1000_0000 == 0b1000_0000 {
            self.status = self.status | 0b1000_0000; // preserve prior status; set NF to 1
        } else {
            self.status = self.status & 0b0111_1111; // preserve prior status; reset NF to 0
        }
    }

    /// Sets the overflow flag accordingly in the status by doing a logical
    /// XOR on the input and output carry
    #[inline]
    fn update_overflow_flag(&mut self, input_carry: bool, output_carry: bool) {
        if input_carry != output_carry {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }
    }
}
