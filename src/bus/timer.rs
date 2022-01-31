use super::interrupts::{InterruptFlag, Interrupts};

const DIV: u16 = 0xFF04;
const TIMA: u16 = 0xFF05;
const TMA: u16 = 0xFF06;
const TAC: u16 = 0xFF07;

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn tick(&mut self, intr: &mut Interrupts) {
        let old = self.div;

        self.div = self.div.wrapping_add(1);

        let bit = match self.tac & 0b11 {
            0b00 => 1 << 9,
            0b01 => 1 << 3,
            0b10 => 1 << 5,
            0b11 => 1 << 7,
            _ => unreachable!(),
        };

        let update = (old & bit != 0) && (self.div & bit == 0);
        if update && self.tac & 0b100 != 0 {
            self.tima = self.tima.wrapping_add(1);

            if self.tima == 0xFF {
                self.tima = self.tma;
                intr.request(InterruptFlag::TIMER);
            }
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            DIV => self.div = 0,
            TIMA => self.tima = data,
            TMA => self.tma = data,
            TAC => self.tac = data,
            _ => panic!("Bad timer address {:04X}", address),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            DIV => (self.div >> 8) as u8,
            TIMA => self.tima,
            TMA => self.tma,
            TAC => self.tac,
            _ => panic!("Bad timer address {:04X}", address),
        }
    }
}
