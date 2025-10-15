/// Contains the definition of the Bus Module, which is responsible for
/// * Intra device communication:
///     - Data reads/writes
///     - Routing hardware interrupts to CPU
/// * Handling memory mappings
/// * Coordinating PPU and CPU clock cycles
use crate::Mem;

/// NES's Memory Map Regions:
/// * RAM - [0x0000 ... 0x2000]
/// * PPU, APU, GamePads, etc: [0x2000 ... 0x4020]
/// * Special storage for cartridges: [0x4020 ... 0x6000]
/// * RAM Space for Cartridge: [0x6000 ... 0x8000]
/// * PRG ROM: [0x8000 ... 0xFFFF]
pub struct Bus {
    cpu_vram: [u8; 2048],
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_vram: [0; 2048],
        }
    }
}

// CPU RAM memory space
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;

// PPU memory space
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mir_dn_addr = addr & 0b111_11111111;
                self.cpu_vram[mir_dn_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mir_dn_addr = addr & 0b100000_00000111;
                todo!("Need to implement PPU first before working on this");
            }
            _ => {
                println!("Can't perform mem access for {addr} yet");
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mir_dn_addr = addr & 0b111_11111111;
                self.cpu_vram[mir_dn_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mir_dn_addr = addr & 0b100000_00000111;
                todo!("Need to implement PPU first before working on this");
            }
            _ => {
                println!("Can't perform mem write for {addr} yet");
            }
        }
    }
}
