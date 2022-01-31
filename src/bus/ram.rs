const WRAM_SIZE: usize = 0x2000;
const WRAM_MASK: usize = WRAM_SIZE - 1;

const HRAM_SIZE: usize = 0x80;
const HRAM_MASK: usize = HRAM_SIZE - 1;

pub struct Ram {
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
        }
    }

    pub fn wram_read(&self, address: u16) -> u8 {
        self.wram[(address as usize) & WRAM_MASK]
    }

    pub fn wram_write(&mut self, address: u16, data: u8) {
        self.wram[(address as usize) & WRAM_MASK] = data;
    }

    pub fn hram_read(&self, address: u16) -> u8 {
        self.hram[(address as usize) & HRAM_MASK]
    }

    pub fn hram_write(&mut self, address: u16, data: u8) {
        self.hram[(address as usize) & HRAM_MASK] = data;
    }
}
