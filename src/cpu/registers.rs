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
}
