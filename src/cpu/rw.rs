use super::{
    instructions::{Dst, Src},
    registers::Reg,
    Cpu, Interface,
};

#[derive(Debug, Clone, Copy)]
pub struct Imm;

impl Src<Imm> for Cpu {
    fn read(&mut self, bus: &mut impl Interface, _src: Imm) -> u8 {
        self.imm(bus)
    }
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

impl Cpu {
    fn get_address(&mut self, bus: &mut impl Interface, address: Address) -> u16 {
        match address {
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
            Address::ZeroPage => 0xFF00 | self.imm(bus) as u16,
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
