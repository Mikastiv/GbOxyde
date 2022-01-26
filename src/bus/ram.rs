use super::WRAM_START;

const RAM_SIZE: usize = 0x400 * 8;

pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            data: vec![0; RAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address.wrapping_sub(WRAM_START) as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address.wrapping_sub(WRAM_START) as usize] = data;
    }
}
