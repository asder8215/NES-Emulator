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
    pub fn is_status_flag_set(&self, flag: &ProcessorStatus) -> bool {
        match flag {
            ProcessorStatus::Carry => self.status & 0b0000_0001 == 1,
            ProcessorStatus::Zero => self.status & 0b0000_0010 == 0b10,
            ProcessorStatus::InterruptDisable => self.status & 0b0000_0100 == 0b100,
            ProcessorStatus::Decimal => self.status & 0b0000_1000 == 0b1000,
            ProcessorStatus::BFlag => unreachable!(),
            ProcessorStatus::Unused => unreachable!(),
            ProcessorStatus::Overflow => self.status & 0b0100_0000 == 0b100_0000,
            ProcessorStatus::Negative => self.status & 0b1000_0000 == 0b1000_0000,
        }
    }
}
