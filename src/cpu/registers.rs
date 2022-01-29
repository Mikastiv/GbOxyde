use bitflags::bitflags;

use super::{
    instructions::{Dst, Src},
    Cpu, Interface,
};

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b1000_0000;
        const N = 0b0100_0000;
        const H = 0b0010_0000;
        const C = 0b0001_0000;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Dst<Reg> for Cpu {
    fn write(&mut self, _bus: &mut impl Interface, dst: Reg, data: u8) {
        match dst {
            Reg::A => self.regs.a = data,
            Reg::B => self.regs.b = data,
            Reg::C => self.regs.c = data,
            Reg::D => self.regs.d = data,
            Reg::E => self.regs.e = data,
            Reg::H => self.regs.h = data,
            Reg::L => self.regs.l = data,
        }
    }
}

impl Src<Reg> for Cpu {
    fn read(&mut self, _bus: &mut impl Interface, src: Reg) -> u8 {
        match src {
            Reg::A => self.regs.a,
            Reg::B => self.regs.b,
            Reg::C => self.regs.c,
            Reg::D => self.regs.d,
            Reg::E => self.regs.e,
            Reg::H => self.regs.h,
            Reg::L => self.regs.l,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg16 {
    BC,
    DE,
    HL,
    SP,
    AF,
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

    pub const fn zf(&self) -> bool {
        self.f.contains(Flags::Z)
    }

    pub const fn nf(&self) -> bool {
        self.f.contains(Flags::N)
    }

    pub const fn hf(&self) -> bool {
        self.f.contains(Flags::H)
    }

    pub const fn cf(&self) -> bool {
        self.f.contains(Flags::C)
    }

    pub fn set_flags(&mut self, flags: Flags, value: bool) {
        self.f.set(flags, value);
    }

    pub fn bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        let (hi, lo) = get_reverse_bytes(value);
        self.b = hi;
        self.c = lo;
    }

    pub fn de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        let (hi, lo) = get_reverse_bytes(value);
        self.d = hi;
        self.e = lo;
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

    pub fn inc_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn dec_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f.bits)
    }

    pub fn set_af(&mut self, value: u16) {
        let (hi, lo) = get_reverse_bytes(value);
        self.a = hi;
        self.f = Flags::from_bits_truncate(lo);
    }

    pub fn read16(&self, src: Reg16) -> u16 {
        match src {
            Reg16::BC => self.bc(),
            Reg16::DE => self.de(),
            Reg16::HL => self.hl(),
            Reg16::SP => self.sp,
            Reg16::AF => self.af(),
        }
    }

    pub fn write16(&mut self, dst: Reg16, data: u16) {
        match dst {
            Reg16::BC => self.set_bc(data),
            Reg16::DE => self.set_de(data),
            Reg16::HL => self.set_hl(data),
            Reg16::SP => self.sp = data,
            Reg16::AF => self.set_af(data),
        }
    }
}

fn get_reverse_bytes(value: u16) -> (u8, u8) {
    let hi = (value >> 8) as u8;
    let lo = value as u8;
    (hi, lo)
}
