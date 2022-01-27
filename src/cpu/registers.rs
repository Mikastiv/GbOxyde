use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b1000_0000;
        const N = 0b0100_0000;
        const H = 0b0010_0000;
        const C = 0b0001_0000;
    }
}

pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub const fn new() -> Self {
        Self {
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0x0100,
            sp: 0,
        }
    }

    pub const fn z(&self) -> bool {
        self.f.contains(Flags::Z)
    }

    pub const fn n(&self) -> bool {
        self.f.contains(Flags::N)
    }

    pub const fn h(&self) -> bool {
        self.f.contains(Flags::H)
    }

    pub const fn c(&self) -> bool {
        self.f.contains(Flags::C)
    }

    pub fn set_flags(&mut self, flags: Flags, value: bool) {
        self.f.set(flags, value);
    }

    pub fn bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn inc_hl(&mut self) {
        let hl = self.hl();
        self.set_hl(hl.wrapping_add(1));
    }

    pub fn dec_hl(&mut self) {
        let hl = self.hl();
        self.set_hl(hl.wrapping_sub(1));
    }

    pub fn set_hl(&mut self, value: u16) {
        let (hi, lo) = get_reverse_bytes(value);
        self.h = hi;
        self.l = lo;
    }
}

fn get_reverse_bytes(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    (hi, lo)
}
