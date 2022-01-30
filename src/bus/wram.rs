const WRAM_SIZE: usize = 0x2000;
const WRAM_MASK: usize = WRAM_SIZE - 1;

pub struct WRam {
    data: Vec<u8>,
}

impl WRam {
    pub fn new() -> Self {
        Self {
            data: vec![0; WRAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[(address as usize) & WRAM_MASK]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[(address as usize) & WRAM_MASK] = data;
    }
}
