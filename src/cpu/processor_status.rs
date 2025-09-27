//! All processor status function for debugging and testing will be contained here

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
    pub fn is_status_flag_set(&self, flag: ProcessorStatus) -> bool {
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
