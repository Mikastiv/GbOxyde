use crate::cpu;

use self::{cartridge::Cartridge, ram::Ram};

pub mod cartridge;
mod ram;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;

pub const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;

pub struct Bus {
    cartridge: Cartridge,
    ram: Ram,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            ram: Ram::new(),
        }
    }
}

impl cpu::Interface for Bus {
    fn peek(&self, address: u16) -> u8 {
        match address {
            ROM_START..=ROM_END => self.cartridge.read(address),
            WRAM_START..=WRAM_END => self.ram.read(address),
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            ROM_START..=ROM_END => self.cartridge.write(address, data),
            WRAM_START..=WRAM_END => self.ram.write(address, data),
            _ => {}
        };
        self.tick(1);
    }

    fn tick(&mut self, count: usize) {
        for _ in 0..count {}
    }
}
