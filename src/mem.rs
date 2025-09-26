//! Contains all memory related operation implementations for the CPU
//! like mem_read and mem_write

use crate::cpu::CPU;

pub(crate) trait Mem {
    /// Returns the value stored at provided memory address
    fn mem_read(&self, addr: u16) -> u8;

    /// Writes 8 bit of data into a specific memory address
    /// 
    /// As a result, if we're writing 16 bits of data, that means
    /// we are going to need two mem_write()s to write this data in    
    fn mem_write(&mut self, addr: u16, data: u8);

    /// Because the NES CPU uses little-endian addressing, that means
    /// a 16-bit value is written with the 8 least significant bit first
    /// and then 8 most significant bit after at the provided position.
    /// 
    /// In other words, our MSB comes from pos + 1, LSB comes from pos
    /// and we need to merge these together
    #[inline]
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    /// Writes 16 bit of data into a specific position in memory
    /// using little-endian addressing.
    /// 
    /// What this means is that our data will be split into 8 MSB bits
    /// and 8 LSB bits. Our LSB bits are written first at pos, and then
    /// our MSB bits are written next at pos + 1
    #[inline]
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos.wrapping_add(1), hi);
    }
}

impl Mem for CPU {
    #[inline]
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    #[inline]
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}