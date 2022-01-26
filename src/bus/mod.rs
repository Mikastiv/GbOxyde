use self::{cartridge::Cartridge, ram::Ram};

mod cartridge;
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
