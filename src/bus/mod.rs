use crate::cpu;

use self::{
    cartridge::Cartridge,
    interrupts::{InterruptFlag, Interrupts},
    joypad::Joypad,
    ram::Ram,
    timer::Timer,
};

pub mod cartridge;
pub mod interrupts;
mod joypad;
mod ram;
mod timer;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xFDFF;

const JOYPAD: u16 = 0xFF00;

const SERIAL_DATA: u16 = 0xFF01;
const SERIAL_CTRL: u16 = 0xFF02;

const TIMER_START: u16 = 0xFF04;
const TIMER_END: u16 = 0xFF07;

const INTR_FLAG: u16 = 0xFF0F;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;

const INTR_ENABLE: u16 = 0xFFFF;

pub struct Bus {
    interrupts: Interrupts,
    cartridge: Cartridge,
    ram: Ram,
    joypad: Joypad,
    serial_data: [u8; 2],
    timer: Timer,
    cycles: u64,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            interrupts: Interrupts::new(),
            cartridge,
            ram: Ram::new(),
            joypad: Joypad::new(),
            serial_data: [0; 2],
            timer: Timer::new(),
            cycles: 0,
        }
    }
}

impl cpu::Interface for Bus {
    fn peek(&self, address: u16) -> u8 {
        match address {
            ROM_START..=ROM_END => self.cartridge.read(address),
            WRAM_START..=WRAM_END => self.ram.wram_read(address),
            JOYPAD => self.joypad.read(),
            SERIAL_DATA => self.serial_data[0],
            SERIAL_CTRL => self.serial_data[1],
            TIMER_START..=TIMER_END => self.timer.read(address),
            INTR_FLAG => self.interrupts.flags(),
            HRAM_START..=HRAM_END => self.ram.hram_read(address),
            INTR_ENABLE => self.interrupts.get_enable(),
            0xFF44 => 0x94,
            _ => 0,
        }
    }

    fn set(&mut self, address: u16, data: u8) {
        match address {
            ROM_START..=ROM_END => self.cartridge.write(address, data),
            WRAM_START..=WRAM_END => self.ram.wram_write(address, data),
            JOYPAD => self.joypad.write(data),
            SERIAL_DATA => self.serial_data[0] = data,
            SERIAL_CTRL => self.serial_data[1] = data,
            TIMER_START..=TIMER_END => self.timer.write(address, data),
            INTR_FLAG => self.interrupts.set_flags(data),
            HRAM_START..=HRAM_END => self.ram.hram_write(address, data),
            INTR_ENABLE => self.interrupts.set_enable(data),
            _ => {}
        };
    }

    fn tick(&mut self, count: usize) {
        for _ in 0..count {
            self.cycles = self.cycles.wrapping_add(1);
            for _ in 0..4 {
                self.timer.tick(&mut self.interrupts);
            }
        }
    }

    fn cycles(&self) -> u64 {
        self.cycles
    }

    fn check_interrupts(&self) -> InterruptFlag {
        self.interrupts.check()
    }

    fn interrupt_handled(&mut self, intr: InterruptFlag) {
        self.interrupts.handled(intr);
    }
}
