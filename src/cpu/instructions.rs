//! All 6502 instruction implementations are defined here for the CPU
//! These instruction are implemented in accordance with:
//! https://www.nesdev.org/obelisk-6502-guide/reference.html#

use crate::cpu::processor_status::ProcessorStatus;

use super::{CPU, addressing_mode::AddressingMode, mem::Mem};

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
            (self.status & 0b0000_0001 == 0b0000_0001) != (self.register_a <= value),
        );
        self.update_carry_flag(self.register_a <= value);
        self.update_zero_flag(self.register_a == 0);
        self.update_negative_flag(self.register_a & 0b1000_0000 == 0b1000_0000);
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
        self.register_a &= value;
        self.update_zero_flag(self.register_a == 0);
        self.update_negative_flag(self.register_a & 0b1000_0000 == 0b1000_0000);
    }

    /// ASL - Arithmetic Shift Left
    ///
    /// Shifts all bits of the accumulator register or memory to the left by one. Bit 0 is
    /// set to 0 as a result and bit 7 is placed in the carry flag of the processor
    /// status register. In effect, ASL multiplies the content of the accumulator
    /// register by 2. Sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn asl(&mut self, mode: &AddressingMode) {
        let old_value: u8;
        if matches!(mode, AddressingMode::Accumulator) {
            old_value = self.register_a;
            self.register_a <<= 1;
            self.update_zero_flag(self.register_a == 0);
            self.update_negative_flag(self.register_a & 0b1000_0000 == 0b1000_0000);
        } else {
            let addr = self.get_operand_address(mode);
            old_value = self.mem_read(addr);
            let new_val = old_value << 1;
            self.mem_write(addr, new_val);
            self.update_zero_flag(new_val == 0);
            self.update_negative_flag(new_val & 0b1000_0000 == 0b1000_0000);
        }

        self.update_carry_flag(old_value & 0b1000_0000 == 0b1000_0000);
    }

    /// Handles all branching instructions:
    /// * BCC - Branch if Carry Clear
    /// * BCS - Branch if Carry Set
    /// * BEQ - Branch if Equal
    /// * BMI - Branch if Minus
    /// * BNE - Branch if Not Equal
    /// * BPL - Branch if Positive
    /// * BVC - Branch if Overflow Clear
    /// * BVS - Branch if Overflow Set
    ///
    /// If the condition is true (implied by the instructions name),
    /// then add the relative displacement to the program counter to cause a
    /// branch to the new location
    #[inline]
    pub(crate) fn branch(&mut self, condition: bool, mode: &AddressingMode) {
        if condition {
            self.program_counter = self.get_operand_address(mode);
        }
    }

    /// BIT - Bit Test
    ///
    /// Test bits in memory against the accumulator register to set or
    /// clear the zero flag. Bits 7 and 6 of the value in memory are
    /// copied into the N and V flags
    #[inline]
    pub(crate) fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.update_zero_flag(self.register_a & value == 0);
        self.update_overflow_flag(value & 0b0100_0000 == 0b0100_0000);
        self.update_negative_flag(value & 0b1000_0000 == 0b1000_0000);
    }

    /// Handles all clear flag instructions:
    /// * CLC - Clear Carry Flag
    /// * CLD - Clear Decimal Flag
    /// * CLI - Clear Interrupt Flag
    /// * CLV - Clear Overflow Flag
    ///
    /// Clears a specific processor status flag
    #[inline]
    pub(crate) fn clear(&mut self, status: &ProcessorStatus) {
        match status {
            ProcessorStatus::Carry => self.update_carry_flag(false),
            ProcessorStatus::Zero => unreachable!(),
            ProcessorStatus::InterruptDisable => self.update_interrupt_flag(false),
            ProcessorStatus::Decimal => self.update_decimal_flag(false),
            ProcessorStatus::BFlag => unreachable!(),
            ProcessorStatus::Unused => unreachable!(),
            ProcessorStatus::Overflow => self.update_overflow_flag(false),
            ProcessorStatus::Negative => unreachable!(),
        }
    }

    /// CMP - Compare
    ///
    /// Compares the content of the accumulator register with a value held
    /// in memory. This is done through subtraction
    #[inline]
    pub(crate) fn cmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_a.wrapping_sub(value);
        self.update_carry_flag(self.register_a >= value);
        self.update_zero_flag(result == 0);
        self.update_negative_flag(result & 0b1000_0000 == 0b1000_0000);
    }

    /// CPX - Compare X Register
    ///
    /// Compares the content of the x register with a value held
    /// in memory. This is done through subtraction
    #[inline]
    pub(crate) fn cpx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_x.wrapping_sub(value);
        self.update_carry_flag(self.register_x >= value);
        self.update_zero_flag(result == 0);
        self.update_negative_flag(result & 0b1000_0000 == 0b1000_0000);
    }

    /// CPY - Compare Y Register
    ///
    /// Compares the content of the y register with a value held
    /// in memory. This is done through subtraction
    #[inline]
    pub(crate) fn cpy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_y.wrapping_sub(value);
        self.update_carry_flag(self.register_y >= value);
        self.update_zero_flag(result == 0);
        self.update_negative_flag(result & 0b1000_0000 == 0b1000_0000);
    }

    /// INX - Increment X Register
    ///
    /// Increments the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_flag(self.register_x == 0);
        self.update_negative_flag(self.register_x & 0b1000_0000 == 0b1000_0000);
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
        self.update_zero_flag(self.register_a == 0);
        self.update_negative_flag(self.register_a & 0b1000_0000 == 0b1000_0000);
    }

    /// TAX - Transfer of Accumulator to X
    ///
    /// Copies the content of the accumulator register into the X register
    /// and sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_flag(self.register_x == 0);
        self.update_negative_flag(self.register_x & 0b1000_0000 == 0b1000_0000);
    }

    /// Updates only the carry flag of the processor status
    #[inline]
    fn update_carry_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }
    }

    /// Updates only the zero flag of the processor status
    #[inline]
    fn update_zero_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    /// Updates only the interrupt flag of the processor status
    #[inline]
    fn update_interrupt_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_0100;
        } else {
            self.status &= 0b1111_1011;
        }
    }

    /// Updates only the decimal flag of the processor status
    #[inline]
    fn update_decimal_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_1000;
        } else {
            self.status &= 0b1111_0111;
        }
    }

    /// Updates only the overflow flag of the processor status
    #[inline]
    fn update_overflow_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    }

    /// Updates only the negative flag of the processor status
    #[inline]
    fn update_negative_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }
}
