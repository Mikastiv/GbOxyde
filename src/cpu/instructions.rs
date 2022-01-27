use super::{Cpu, Interface};

pub trait Dst<T: Copy> {
    fn write(&mut self, bus: &mut impl Interface, dst: T, data: u8);
}

pub trait Src<T: Copy> {
    fn read(&mut self, bus: &mut impl Interface, src: T) -> u8;
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

#[derive(Debug, Clone, Copy)]
pub enum Absolute {
    HL,
}

#[derive(Debug, Clone, Copy)]
pub struct Imm;

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

impl Dst<Absolute> for Cpu {
    fn write(&mut self, bus: &mut impl Interface, dst: Absolute, data: u8) {
        let address = match dst {
            Absolute::HL => self.regs.hl(),
        };

        bus.write(address, data);
    }
}

impl Src<Absolute> for Cpu {
    fn read(&mut self, bus: &mut impl Interface, src: Absolute) -> u8 {
        let address = match src {
            Absolute::HL => self.regs.hl(),
        };

        bus.read(address)
    }
}

impl Cpu {
    pub fn execute(&mut self, bus: &mut impl Interface) {
        match self.cur_opcode {
            0x00 => self.nop(),
            0x10 => self.stop(),
            0x37 => self.scf(),
            0x3F => self.ccf(),
            0xF3 => self.di(),
            0xFB => self.ei(),
            _ => {}
        }
    }
}
