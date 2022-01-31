use crate::cpu;

use self::{cartridge::Cartridge, wram::WRam};

pub mod cartridge;
mod io;
mod wram;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xFDFF;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;

pub struct Bus {
    cartridge: Cartridge,
    wram: WRam,
    cycles: u64,
    serial_data: [u8; 2],
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: WRam::new(),
            cycles: 0,
            serial_data: [0; 2],
        }
    }
}

impl cpu::Interface for Bus {
    fn peek(&self, address: u16) -> u8 {
        match address {
            ROM_START..=ROM_END => self.cartridge.read(address),
            WRAM_START..=WRAM_END => self.wram.read(address),
            0xFF01 => self.serial_data[0],
            0xFF02 => self.serial_data[1],
            _ => 0,
        }
    }

    fn set(&mut self, address: u16, data: u8) {
        match address {
            ROM_START..=ROM_END => self.cartridge.write(address, data),
            WRAM_START..=WRAM_END => self.wram.write(address, data),
            0xFF01 => self.serial_data[0] = data,
            0xFF02 => self.serial_data[1] = data,
            _ => {}
        };
    }

    fn tick(&mut self, count: usize) {
        for _ in 0..count {
            self.cycles = self.cycles.wrapping_add(1);
        }
    }

    fn cycles(&self) -> u64 {
        self.cycles
    }
}
