//! Contains functions that can inform you whether a certain flag in the processor
//! status is set or not
//!
//! Here's a visual on how the NES's processor status flags are represented
//! in a u8
//!
//! Processor Status
//! 7  bit  0
//! ---- ----
//! NV1B DIZC
//! |||| ||||
//! |||| |||+- Carry (C)
//! |||| ||+-- Zero (Z)
//! |||| |+--- Interrupt Disable (I)
//! |||| +---- Decimal (D)
//! |||+------ (No CPU effect; see: the B flag)
//! ||+------- (No CPU effect; always pushed as 1)
//! |+-------- Overflow (V)
//! +--------- Negative (N)

use super::CPU;

pub(crate) const CARRY_BIT: u8 = 0b0000_0001;
pub(crate) const ZERO_BIT: u8 = 0b0000_0010;
pub(crate) const INTERRUPT_DISABLE_BIT: u8 = 0b0000_0100;
pub(crate) const DECIMAL_BIT: u8 = 0b0000_1000;
pub(crate) const OVERFLOW_BIT: u8 = 0b0100_0000;
pub(crate) const NEGATIVE_BIT: u8 = 0b1000_0000;

/// This is an enum used for testing check if a certain processor
/// status was set or not
///
/// See: https://www.nesdev.org/wiki/Status_flags
///
/// For more details on what each processor status means
pub enum ProcessorStatus {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    // No CPU effect, though the B Flag was used as a
    // transient signal in the CPU controlling whether
    // it was processing an interrupt when flags were pushed
    BFlag,
    // No CPU effect
    Unused,
    Overflow,
    Negative,
}

impl CPU {
    /// Checks if a specific processor status flag is set
    pub fn is_status_flag_set(&self, flag: &ProcessorStatus) -> bool {
        match flag {
            ProcessorStatus::Carry => self.status & CARRY_BIT == 1,
            ProcessorStatus::Zero => self.status & ZERO_BIT == 0b10,
            ProcessorStatus::InterruptDisable => self.status & INTERRUPT_DISABLE_BIT == 0b100,
            // the NES doesn't actually use decimal mode so this gets unused
            ProcessorStatus::Decimal => self.status & DECIMAL_BIT == 0b1000,
            ProcessorStatus::BFlag => unreachable!(),
            ProcessorStatus::Unused => unreachable!(),
            ProcessorStatus::Overflow => self.status & OVERFLOW_BIT == 0b100_0000,
            ProcessorStatus::Negative => self.status & NEGATIVE_BIT == 0b1000_0000,
        }
    }

    /// Updates only the carry flag of the processor status
    #[inline]
    pub(crate) fn update_carry_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_0001;
        } else {
            self.status &= 0b1111_1110;
        }
    }

    /// Updates only the zero flag of the processor status
    #[inline]
    pub(crate) fn update_zero_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    /// Updates only the interrupt flag of the processor status
    #[inline]
    pub(crate) fn update_interrupt_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_0100;
        } else {
            self.status &= 0b1111_1011;
        }
    }

    /// Updates only the decimal flag of the processor status
    #[inline]
    pub(crate) fn update_decimal_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0000_1000;
        } else {
            self.status &= 0b1111_0111;
        }
    }

    /// Updates only the overflow flag of the processor status
    #[inline]
    pub(crate) fn update_overflow_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    }

    /// Updates only the negative flag of the processor status
    #[inline]
    pub(crate) fn update_negative_flag(&mut self, condition: bool) {
        if condition {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }
}
