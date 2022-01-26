use crate::{
    bus::{cartridge::Cartridge, Bus},
    cpu::Cpu,
};

pub struct Gameboy {
    cpu: Cpu,
    bus: Bus,
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Self {
        let cartridge = Cartridge::new(rom);
        cartridge.print_header();

        Self {
            cpu: Cpu::new(),
            bus: Bus::new(cartridge),
        }
    }

    pub fn run(&mut self) {
        self.cpu.execute(&mut self.bus);
    }
}
