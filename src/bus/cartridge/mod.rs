use self::header::{Header, HEADER_LOC, HEADER_SIZE};

mod header;

pub struct Cartridge {
    header: Header,
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            header: Header::new(&rom[HEADER_LOC..HEADER_LOC + HEADER_SIZE]),
            rom,
        }
    }

    pub fn print_header(&self) {
        let line = "-".repeat(30);
        println!("{}", &line);
        self.header.print(12);
        println!("{}", &line);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.rom[address as usize] = data;
    }
}
