//! All 6502 instruction implementations are defined here for the CPU
//! These instruction are implemented in accordance with:
//! https://www.nesdev.org/obelisk-6502-guide/reference.html#

use crate::cpu::{
    STACK,
    processor_status::{CARRY_BIT, NEGATIVE_BIT, OVERFLOW_BIT, ProcessorStatus},
};

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
            .wrapping_add(self.status & CARRY_BIT);

        self.update_overflow_flag(
            (self.status & CARRY_BIT == CARRY_BIT) != (self.register_a <= value),
        );
        self.update_carry_flag(self.register_a <= value);
        self.update_zero_flag(self.register_a == 0);
        self.update_negative_flag(self.register_a & NEGATIVE_BIT == NEGATIVE_BIT);
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
        self.update_negative_flag(self.register_a & NEGATIVE_BIT == NEGATIVE_BIT);
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
            self.update_negative_flag(self.register_a & NEGATIVE_BIT == NEGATIVE_BIT);
        } else {
            let addr = self.get_operand_address(mode);
            old_value = self.mem_read(addr);
            let new_val = old_value << 1;
            self.mem_write(addr, new_val);
            self.update_zero_flag(new_val == 0);
            self.update_negative_flag(new_val & NEGATIVE_BIT == NEGATIVE_BIT);
        }

        self.update_carry_flag(old_value & NEGATIVE_BIT == NEGATIVE_BIT);
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
        self.update_overflow_flag(value & OVERFLOW_BIT == OVERFLOW_BIT);
        self.update_negative_flag(value & NEGATIVE_BIT == NEGATIVE_BIT);
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
        self.update_negative_flag(result & NEGATIVE_BIT == NEGATIVE_BIT);
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
        self.update_negative_flag(result & NEGATIVE_BIT == NEGATIVE_BIT);
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
        self.update_negative_flag(result & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// DEC - Decrement Memory
    ///
    /// Decrements the value held in memory by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_sub(1);

        self.mem_write(addr, result);
        self.update_zero_flag(result == 0);
        self.update_negative_flag(result & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// DEX - Decrement X Register
    ///
    /// Decrements the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_flag(self.register_x == 0);
        self.update_negative_flag(self.register_x & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// DEY - Decrement Y Register
    ///
    /// Decrements the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_flag(self.register_y == 0);
        self.update_negative_flag(self.register_y & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// EOR - Exclusive OR
    ///
    /// Performs an exclusive OR operation on the accumulator register
    /// with the content in memory
    #[inline]
    pub(crate) fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a ^= value;

        self.update_zero_flag(self.register_a == 0);
        self.update_negative_flag(self.register_a & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// INC - Increment Memory
    ///
    /// Increments the value held in memory by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_add(1);
        self.mem_write(addr, result);

        self.update_zero_flag(result == 0);
        self.update_negative_flag(result & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// INX - Increment X Register
    ///
    /// Increments the X register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_flag(self.register_x == 0);
        self.update_negative_flag(self.register_x & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// INY - Increment Y Register
    ///
    /// Increments the Y register by 1 (wraps around on overflow) and sets
    /// the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_flag(self.register_y == 0);
        self.update_negative_flag(self.register_y & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// JMP - Jump
    ///
    /// Sets the program counter to the address specified by operand
    #[inline]
    pub(crate) fn jmp(&mut self, mode: &AddressingMode) {
        self.program_counter = self.get_operand_address(mode);
    }

    /// JSR - Jump to Subroutine
    ///
    /// Pushes the address (minus one) of the return point to the stack
    /// and then sets the program counter to the target memory address
    ///
    /// This is what is used to return back to the address after a function
    /// call
    #[inline]
    pub(crate) fn jsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        // Since we want to return back to the address where this
        // subroutine was invoked, we store PC + 2 - 1 (2 bytes
        // bc JSR is 3 bytes long, one of them already processed
        // for the instruction itself, and subtract by 1 to get the
        // addr to the last byte before viewing the next instruction
        // inside the parent region of code).
        let next_ins = self.program_counter.wrapping_add(1);
        let (msb_byte, lsb_byte) = ((next_ins >> 8) as u8, next_ins as u8);

        // we do two separate writes to the STACK because we want to only
        // write within the STACK region 0x100 - 0x1FF inclusive (do not touch
        // below 0x100 or above 0x1FF)
        self.mem_write(STACK + self.stack_pointer as u16, msb_byte);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.mem_write(STACK + self.stack_pointer as u16, lsb_byte);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.program_counter = addr;
    }

    /// LDA - Load Accumulator
    ///
    /// Loads a value in memory into the accumulator register and
    /// sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_flag(self.register_a == 0);
        self.update_negative_flag(self.register_a & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// LDX - Load X Register
    ///
    /// Loads a value in memory into the x register and
    /// sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_flag(self.register_x == 0);
        self.update_negative_flag(self.register_x & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// LDY - Load Y Register
    ///
    /// Loads a value in memory into the y register and
    /// sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_flag(self.register_y == 0);
        self.update_negative_flag(self.register_y & NEGATIVE_BIT == NEGATIVE_BIT);
    }

    /// LSR - Logical Shift Right
    ///
    /// Shifts all bits of the accumulator register or memory to the right by one. Bit 7 is
    /// set to 0 as a result and bit 0 is placed in the carry flag of the processor
    /// status register. In effect, ASL divides the content of the accumulator
    /// register by 2. Sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn lsr(&mut self, mode: &AddressingMode) {
        let old_value: u8;
        if matches!(mode, AddressingMode::Accumulator) {
            old_value = self.register_a;
            self.register_a >>= 1;
            self.update_zero_flag(self.register_a == 0);
        } else {
            let addr = self.get_operand_address(mode);
            old_value = self.mem_read(addr);
            let new_val = old_value >> 1;
            self.mem_write(addr, new_val);
            self.update_zero_flag(new_val == 0);
        }

        // if the bit 7 is set to 0 always, then the negative flag will always
        // be cleared
        self.update_negative_flag(false);
        // carry bit is set based on whether bit 0 was 1 or 0 in old_value
        self.update_carry_flag(old_value & CARRY_BIT == CARRY_BIT);
    }

    /// TAX - Transfer of Accumulator to X
    ///
    /// Copies the content of the accumulator register into the X register
    /// and sets the zero and negative flag as appropriate
    #[inline]
    pub(crate) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_flag(self.register_x == 0);
        self.update_negative_flag(self.register_x & NEGATIVE_BIT == NEGATIVE_BIT);
    }
}
