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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Address {
    HL,
    BC,
    DE,
    HLI,
    HLD,
    Absolute,
    ZeroPage,
    ZeroPageC,
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

impl Cpu {
    fn get_address(&mut self, bus: &mut impl Interface, addr: Address) -> u16 {
        match addr {
            Address::HL => self.regs.hl(),
            Address::BC => self.regs.bc(),
            Address::DE => self.regs.de(),
            Address::HLI => {
                let hl = self.regs.hl();
                self.regs.inc_hl();
                hl
            }
            Address::HLD => {
                let hl = self.regs.hl();
                self.regs.dec_hl();
                hl
            }
            Address::Absolute => self.imm_word(bus),
            Address::ZeroPage => {
                let data = self.imm(bus);
                0xFF00 | data as u16
            }
            Address::ZeroPageC => 0xFF00 | self.regs.c as u16,
        }
    }
}

impl Dst<Address> for Cpu {
    fn write(&mut self, bus: &mut impl Interface, dst: Address, data: u8) {
        let address = self.get_address(bus, dst);
        bus.write(address, data);
    }
}

impl Src<Address> for Cpu {
    fn read(&mut self, bus: &mut impl Interface, src: Address) -> u8 {
        let address = self.get_address(bus, src);
        bus.read(address)
    }
}

impl Src<Imm> for Cpu {
    fn read(&mut self, bus: &mut impl Interface, _src: Imm) -> u8 {
        self.imm(bus)
    }
}

impl Cpu {
    pub fn execute(&mut self, bus: &mut impl Interface) {
        match self.cur_opcode {
            0x00 => self.nop(),
            0x10 => self.stop(),
            0x37 => self.scf(),
            0x3F => self.ccf(),
            0x76 => self.halt(),
            0xF3 => self.di(),
            0xFB => self.ei(),
            0x02 => self.load(bus, Address::BC, Reg::A),
            0x12 => self.load(bus, Address::DE, Reg::A),
            0x22 => self.load(bus, Address::HLI, Reg::A),
            0x32 => self.load(bus, Address::HLD, Reg::A),
            0x0A => self.load(bus, Reg::A, Address::BC),
            0x1A => self.load(bus, Reg::A, Address::DE),
            0x2A => self.load(bus, Reg::A, Address::HLI),
            0x3A => self.load(bus, Reg::A, Address::HLD),
            0x06 => self.load(bus, Reg::B, Imm),
            0x0E => self.load(bus, Reg::C, Imm),
            0x16 => self.load(bus, Reg::D, Imm),
            0x1E => self.load(bus, Reg::E, Imm),
            0x26 => self.load(bus, Reg::H, Imm),
            0x2E => self.load(bus, Reg::L, Imm),
            0x36 => self.load(bus, Address::HL, Imm),
            0x3E => self.load(bus, Reg::A, Imm),
            0x40 => self.load(bus, Reg::B, Reg::B),
            0x41 => self.load(bus, Reg::B, Reg::C),
            0x42 => self.load(bus, Reg::B, Reg::D),
            0x43 => self.load(bus, Reg::B, Reg::E),
            0x44 => self.load(bus, Reg::B, Reg::H),
            0x45 => self.load(bus, Reg::B, Reg::L),
            0x46 => self.load(bus, Reg::B, Address::HL),
            0x47 => self.load(bus, Reg::B, Reg::A),
            0x48 => self.load(bus, Reg::C, Reg::B),
            0x49 => self.load(bus, Reg::C, Reg::C),
            0x4A => self.load(bus, Reg::C, Reg::D),
            0x4B => self.load(bus, Reg::C, Reg::E),
            0x4C => self.load(bus, Reg::C, Reg::H),
            0x4D => self.load(bus, Reg::C, Reg::L),
            0x4E => self.load(bus, Reg::C, Address::HL),
            0x4F => self.load(bus, Reg::C, Reg::A),
            0x50 => self.load(bus, Reg::D, Reg::B),
            0x51 => self.load(bus, Reg::D, Reg::C),
            0x52 => self.load(bus, Reg::D, Reg::D),
            0x53 => self.load(bus, Reg::D, Reg::E),
            0x54 => self.load(bus, Reg::D, Reg::H),
            0x55 => self.load(bus, Reg::D, Reg::L),
            0x56 => self.load(bus, Reg::D, Address::HL),
            0x57 => self.load(bus, Reg::D, Reg::A),
            0x58 => self.load(bus, Reg::E, Reg::B),
            0x59 => self.load(bus, Reg::E, Reg::C),
            0x5A => self.load(bus, Reg::E, Reg::D),
            0x5B => self.load(bus, Reg::E, Reg::E),
            0x5C => self.load(bus, Reg::E, Reg::H),
            0x5D => self.load(bus, Reg::E, Reg::L),
            0x5E => self.load(bus, Reg::E, Address::HL),
            0x5F => self.load(bus, Reg::E, Reg::A),
            0x60 => self.load(bus, Reg::H, Reg::B),
            0x61 => self.load(bus, Reg::H, Reg::C),
            0x62 => self.load(bus, Reg::H, Reg::D),
            0x63 => self.load(bus, Reg::H, Reg::E),
            0x64 => self.load(bus, Reg::H, Reg::H),
            0x65 => self.load(bus, Reg::H, Reg::L),
            0x66 => self.load(bus, Reg::H, Address::HL),
            0x67 => self.load(bus, Reg::H, Reg::A),
            0x68 => self.load(bus, Reg::L, Reg::B),
            0x69 => self.load(bus, Reg::L, Reg::C),
            0x6A => self.load(bus, Reg::L, Reg::D),
            0x6B => self.load(bus, Reg::L, Reg::E),
            0x6C => self.load(bus, Reg::L, Reg::H),
            0x6D => self.load(bus, Reg::L, Reg::L),
            0x6E => self.load(bus, Reg::L, Address::HL),
            0x6F => self.load(bus, Reg::L, Reg::A),
            0x70 => self.load(bus, Address::HL, Reg::B),
            0x71 => self.load(bus, Address::HL, Reg::C),
            0x72 => self.load(bus, Address::HL, Reg::D),
            0x73 => self.load(bus, Address::HL, Reg::E),
            0x74 => self.load(bus, Address::HL, Reg::H),
            0x75 => self.load(bus, Address::HL, Reg::L),
            0x77 => self.load(bus, Address::HL, Reg::A),
            0x78 => self.load(bus, Reg::A, Reg::B),
            0x79 => self.load(bus, Reg::A, Reg::C),
            0x7A => self.load(bus, Reg::A, Reg::D),
            0x7B => self.load(bus, Reg::A, Reg::E),
            0x7C => self.load(bus, Reg::A, Reg::H),
            0x7D => self.load(bus, Reg::A, Reg::L),
            0x7E => self.load(bus, Reg::A, Address::HL),
            0x7F => self.load(bus, Reg::A, Reg::A),
            0xE0 => self.load(bus, Address::ZeroPage, Reg::A),
            0xF0 => self.load(bus, Reg::A, Address::ZeroPage),
            0xE2 => self.load(bus, Address::ZeroPageC, Reg::A),
            0xF2 => self.load(bus, Reg::A, Address::ZeroPageC),
            0xEA => self.load(bus, Address::Absolute, Reg::A),
            0xFA => self.load(bus, Reg::A, Address::Absolute),
            _ => {}
        }
    }
}
