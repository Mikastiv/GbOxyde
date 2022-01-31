use super::{
    registers::{Reg, Reg16},
    rw::{Address, Imm},
    Cpu, Interface,
};

pub trait Dst<T: Copy> {
    fn write(&mut self, bus: &mut impl Interface, dst: T, data: u8);
}

pub trait Src<T: Copy> {
    fn read(&mut self, bus: &mut impl Interface, src: T) -> u8;
}

#[derive(Debug, Clone, Copy)]
pub enum Rotate {
    Rlc,
    Rl,
    Rrc,
    Rr,
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
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
            0x27 => self.daa(),
            0x2F => self.cpl(),
            0x07 => self.rotate_a(Rotate::Rlc),
            0x0F => self.rotate_a(Rotate::Rrc),
            0x17 => self.rotate_a(Rotate::Rl),
            0x1F => self.rotate_a(Rotate::Rr),
            0x02 => self.ld(bus, Address::BC, Reg::A),
            0x12 => self.ld(bus, Address::DE, Reg::A),
            0x22 => self.ld(bus, Address::HLI, Reg::A),
            0x32 => self.ld(bus, Address::HLD, Reg::A),
            0x0A => self.ld(bus, Reg::A, Address::BC),
            0x1A => self.ld(bus, Reg::A, Address::DE),
            0x2A => self.ld(bus, Reg::A, Address::HLI),
            0x3A => self.ld(bus, Reg::A, Address::HLD),
            0x06 => self.ld(bus, Reg::B, Imm),
            0x0E => self.ld(bus, Reg::C, Imm),
            0x16 => self.ld(bus, Reg::D, Imm),
            0x1E => self.ld(bus, Reg::E, Imm),
            0x26 => self.ld(bus, Reg::H, Imm),
            0x2E => self.ld(bus, Reg::L, Imm),
            0x36 => self.ld(bus, Address::HL, Imm),
            0x3E => self.ld(bus, Reg::A, Imm),
            0x40 => self.ld(bus, Reg::B, Reg::B),
            0x41 => self.ld(bus, Reg::B, Reg::C),
            0x42 => self.ld(bus, Reg::B, Reg::D),
            0x43 => self.ld(bus, Reg::B, Reg::E),
            0x44 => self.ld(bus, Reg::B, Reg::H),
            0x45 => self.ld(bus, Reg::B, Reg::L),
            0x46 => self.ld(bus, Reg::B, Address::HL),
            0x47 => self.ld(bus, Reg::B, Reg::A),
            0x48 => self.ld(bus, Reg::C, Reg::B),
            0x49 => self.ld(bus, Reg::C, Reg::C),
            0x4A => self.ld(bus, Reg::C, Reg::D),
            0x4B => self.ld(bus, Reg::C, Reg::E),
            0x4C => self.ld(bus, Reg::C, Reg::H),
            0x4D => self.ld(bus, Reg::C, Reg::L),
            0x4E => self.ld(bus, Reg::C, Address::HL),
            0x4F => self.ld(bus, Reg::C, Reg::A),
            0x50 => self.ld(bus, Reg::D, Reg::B),
            0x51 => self.ld(bus, Reg::D, Reg::C),
            0x52 => self.ld(bus, Reg::D, Reg::D),
            0x53 => self.ld(bus, Reg::D, Reg::E),
            0x54 => self.ld(bus, Reg::D, Reg::H),
            0x55 => self.ld(bus, Reg::D, Reg::L),
            0x56 => self.ld(bus, Reg::D, Address::HL),
            0x57 => self.ld(bus, Reg::D, Reg::A),
            0x58 => self.ld(bus, Reg::E, Reg::B),
            0x59 => self.ld(bus, Reg::E, Reg::C),
            0x5A => self.ld(bus, Reg::E, Reg::D),
            0x5B => self.ld(bus, Reg::E, Reg::E),
            0x5C => self.ld(bus, Reg::E, Reg::H),
            0x5D => self.ld(bus, Reg::E, Reg::L),
            0x5E => self.ld(bus, Reg::E, Address::HL),
            0x5F => self.ld(bus, Reg::E, Reg::A),
            0x60 => self.ld(bus, Reg::H, Reg::B),
            0x61 => self.ld(bus, Reg::H, Reg::C),
            0x62 => self.ld(bus, Reg::H, Reg::D),
            0x63 => self.ld(bus, Reg::H, Reg::E),
            0x64 => self.ld(bus, Reg::H, Reg::H),
            0x65 => self.ld(bus, Reg::H, Reg::L),
            0x66 => self.ld(bus, Reg::H, Address::HL),
            0x67 => self.ld(bus, Reg::H, Reg::A),
            0x68 => self.ld(bus, Reg::L, Reg::B),
            0x69 => self.ld(bus, Reg::L, Reg::C),
            0x6A => self.ld(bus, Reg::L, Reg::D),
            0x6B => self.ld(bus, Reg::L, Reg::E),
            0x6C => self.ld(bus, Reg::L, Reg::H),
            0x6D => self.ld(bus, Reg::L, Reg::L),
            0x6E => self.ld(bus, Reg::L, Address::HL),
            0x6F => self.ld(bus, Reg::L, Reg::A),
            0x70 => self.ld(bus, Address::HL, Reg::B),
            0x71 => self.ld(bus, Address::HL, Reg::C),
            0x72 => self.ld(bus, Address::HL, Reg::D),
            0x73 => self.ld(bus, Address::HL, Reg::E),
            0x74 => self.ld(bus, Address::HL, Reg::H),
            0x75 => self.ld(bus, Address::HL, Reg::L),
            0x77 => self.ld(bus, Address::HL, Reg::A),
            0x78 => self.ld(bus, Reg::A, Reg::B),
            0x79 => self.ld(bus, Reg::A, Reg::C),
            0x7A => self.ld(bus, Reg::A, Reg::D),
            0x7B => self.ld(bus, Reg::A, Reg::E),
            0x7C => self.ld(bus, Reg::A, Reg::H),
            0x7D => self.ld(bus, Reg::A, Reg::L),
            0x7E => self.ld(bus, Reg::A, Address::HL),
            0x7F => self.ld(bus, Reg::A, Reg::A),
            0xE0 => self.ld(bus, Address::ZeroPage, Reg::A),
            0xF0 => self.ld(bus, Reg::A, Address::ZeroPage),
            0xE2 => self.ld(bus, Address::ZeroPageC, Reg::A),
            0xF2 => self.ld(bus, Reg::A, Address::ZeroPageC),
            0xEA => self.ld(bus, Address::Absolute, Reg::A),
            0xFA => self.ld(bus, Reg::A, Address::Absolute),
            0x01 => self.ld16(bus, Reg16::BC),
            0x11 => self.ld16(bus, Reg16::DE),
            0x21 => self.ld16(bus, Reg16::HL),
            0x31 => self.ld16(bus, Reg16::SP),
            0x08 => self.ld_mem_d16_sp(bus),
            0xF8 => self.ld_hl_sp_d8(bus),
            0xF9 => self.ld_sp_hl(bus),
            0xC1 => self.pop(bus, Reg16::BC),
            0xD1 => self.pop(bus, Reg16::DE),
            0xE1 => self.pop(bus, Reg16::HL),
            0xF1 => self.pop(bus, Reg16::AF),
            0xC5 => self.push(bus, Reg16::BC),
            0xD5 => self.push(bus, Reg16::DE),
            0xE5 => self.push(bus, Reg16::HL),
            0xF5 => self.push(bus, Reg16::AF),
            0x80 => self.add(bus, Reg::B),
            0x81 => self.add(bus, Reg::C),
            0x82 => self.add(bus, Reg::D),
            0x83 => self.add(bus, Reg::E),
            0x84 => self.add(bus, Reg::H),
            0x85 => self.add(bus, Reg::L),
            0x86 => self.add(bus, Address::HL),
            0x87 => self.add(bus, Reg::A),
            0xC6 => self.add(bus, Imm),
            0x09 => self.add16(bus, Reg16::BC),
            0x19 => self.add16(bus, Reg16::DE),
            0x29 => self.add16(bus, Reg16::HL),
            0x39 => self.add16(bus, Reg16::SP),
            0xE8 => self.add_sp_d8(bus),
            0x88 => self.adc(bus, Reg::B),
            0x89 => self.adc(bus, Reg::C),
            0x8A => self.adc(bus, Reg::D),
            0x8B => self.adc(bus, Reg::E),
            0x8C => self.adc(bus, Reg::H),
            0x8D => self.adc(bus, Reg::L),
            0x8E => self.adc(bus, Address::HL),
            0x8F => self.adc(bus, Reg::A),
            0xCE => self.adc(bus, Imm),
            0x90 => self.sub(bus, Reg::B),
            0x91 => self.sub(bus, Reg::C),
            0x92 => self.sub(bus, Reg::D),
            0x93 => self.sub(bus, Reg::E),
            0x94 => self.sub(bus, Reg::H),
            0x95 => self.sub(bus, Reg::L),
            0x96 => self.sub(bus, Address::HL),
            0x97 => self.sub(bus, Reg::A),
            0xD6 => self.sub(bus, Imm),
            0x98 => self.sbc(bus, Reg::B),
            0x99 => self.sbc(bus, Reg::C),
            0x9A => self.sbc(bus, Reg::D),
            0x9B => self.sbc(bus, Reg::E),
            0x9C => self.sbc(bus, Reg::H),
            0x9D => self.sbc(bus, Reg::L),
            0x9E => self.sbc(bus, Address::HL),
            0x9F => self.sbc(bus, Reg::A),
            0xDE => self.sbc(bus, Imm),
            0xA0 => self.and(bus, Reg::B),
            0xA1 => self.and(bus, Reg::C),
            0xA2 => self.and(bus, Reg::D),
            0xA3 => self.and(bus, Reg::E),
            0xA4 => self.and(bus, Reg::H),
            0xA5 => self.and(bus, Reg::L),
            0xA6 => self.and(bus, Address::HL),
            0xA7 => self.and(bus, Reg::A),
            0xE6 => self.and(bus, Imm),
            0xA8 => self.xor(bus, Reg::B),
            0xA9 => self.xor(bus, Reg::C),
            0xAA => self.xor(bus, Reg::D),
            0xAB => self.xor(bus, Reg::E),
            0xAC => self.xor(bus, Reg::H),
            0xAD => self.xor(bus, Reg::L),
            0xAE => self.xor(bus, Address::HL),
            0xAF => self.xor(bus, Reg::A),
            0xEE => self.xor(bus, Imm),
            0xB0 => self.or(bus, Reg::B),
            0xB1 => self.or(bus, Reg::C),
            0xB2 => self.or(bus, Reg::D),
            0xB3 => self.or(bus, Reg::E),
            0xB4 => self.or(bus, Reg::H),
            0xB5 => self.or(bus, Reg::L),
            0xB6 => self.or(bus, Address::HL),
            0xB7 => self.or(bus, Reg::A),
            0xF6 => self.or(bus, Imm),
            0xB8 => self.cp(bus, Reg::B),
            0xB9 => self.cp(bus, Reg::C),
            0xBA => self.cp(bus, Reg::D),
            0xBB => self.cp(bus, Reg::E),
            0xBC => self.cp(bus, Reg::H),
            0xBD => self.cp(bus, Reg::L),
            0xBE => self.cp(bus, Address::HL),
            0xBF => self.cp(bus, Reg::A),
            0xFE => self.cp(bus, Imm),
            0x04 => self.inc(bus, Reg::B),
            0x14 => self.inc(bus, Reg::D),
            0x24 => self.inc(bus, Reg::H),
            0x34 => self.inc(bus, Address::HL),
            0x0C => self.inc(bus, Reg::C),
            0x1C => self.inc(bus, Reg::E),
            0x2C => self.inc(bus, Reg::L),
            0x3C => self.inc(bus, Reg::A),
            0x03 => self.inc16(bus, Reg16::BC),
            0x13 => self.inc16(bus, Reg16::DE),
            0x23 => self.inc16(bus, Reg16::HL),
            0x33 => self.inc16(bus, Reg16::SP),
            0x05 => self.dec(bus, Reg::B),
            0x15 => self.dec(bus, Reg::D),
            0x25 => self.dec(bus, Reg::H),
            0x35 => self.dec(bus, Address::HL),
            0x0D => self.dec(bus, Reg::C),
            0x1D => self.dec(bus, Reg::E),
            0x2D => self.dec(bus, Reg::L),
            0x3D => self.dec(bus, Reg::A),
            0x0B => self.dec16(bus, Reg16::BC),
            0x1B => self.dec16(bus, Reg16::DE),
            0x2B => self.dec16(bus, Reg16::HL),
            0x3B => self.dec16(bus, Reg16::SP),
            0xC3 => self.jp(bus),
            0xE9 => self.jp_hl(),
            0xC2 => self.jp_cond(bus, Condition::NZ),
            0xCA => self.jp_cond(bus, Condition::Z),
            0xD2 => self.jp_cond(bus, Condition::NC),
            0xDA => self.jp_cond(bus, Condition::C),
            0x18 => self.jr(bus),
            0x20 => self.jr_cond(bus, Condition::NZ),
            0x28 => self.jr_cond(bus, Condition::Z),
            0x30 => self.jr_cond(bus, Condition::NC),
            0x38 => self.jr_cond(bus, Condition::C),
            0xCD => self.call(bus),
            0xC4 => self.call_cond(bus, Condition::NZ),
            0xCC => self.call_cond(bus, Condition::Z),
            0xD4 => self.call_cond(bus, Condition::NC),
            0xDC => self.call_cond(bus, Condition::C),
            0xC9 => self.ret(bus),
            0xC0 => self.ret_cond(bus, Condition::NZ),
            0xC8 => self.ret_cond(bus, Condition::Z),
            0xD0 => self.ret_cond(bus, Condition::NC),
            0xD8 => self.ret_cond(bus, Condition::C),
            0xD9 => self.reti(bus),
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst(bus),
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                self.undefined()
            }
            0xCB => self.prefix_cb(bus),
        }
    }

    fn prefix_cb(&mut self, bus: &mut impl Interface) {
        let opcode = self.imm(bus);
        match opcode {
            0x00 => self.rotate(bus, Reg::B, Rotate::Rlc),
            0x01 => self.rotate(bus, Reg::C, Rotate::Rlc),
            0x02 => self.rotate(bus, Reg::D, Rotate::Rlc),
            0x03 => self.rotate(bus, Reg::E, Rotate::Rlc),
            0x04 => self.rotate(bus, Reg::H, Rotate::Rlc),
            0x05 => self.rotate(bus, Reg::L, Rotate::Rlc),
            0x06 => self.rotate(bus, Address::HL, Rotate::Rlc),
            0x07 => self.rotate(bus, Reg::A, Rotate::Rrc),
            0x08 => self.rotate(bus, Reg::B, Rotate::Rrc),
            0x09 => self.rotate(bus, Reg::C, Rotate::Rrc),
            0x0A => self.rotate(bus, Reg::D, Rotate::Rrc),
            0x0B => self.rotate(bus, Reg::E, Rotate::Rrc),
            0x0C => self.rotate(bus, Reg::H, Rotate::Rrc),
            0x0D => self.rotate(bus, Reg::L, Rotate::Rrc),
            0x0E => self.rotate(bus, Address::HL, Rotate::Rrc),
            0x0F => self.rotate(bus, Reg::A, Rotate::Rrc),
            0x10 => self.rotate(bus, Reg::B, Rotate::Rl),
            0x11 => self.rotate(bus, Reg::C, Rotate::Rl),
            0x12 => self.rotate(bus, Reg::D, Rotate::Rl),
            0x13 => self.rotate(bus, Reg::E, Rotate::Rl),
            0x14 => self.rotate(bus, Reg::H, Rotate::Rl),
            0x15 => self.rotate(bus, Reg::L, Rotate::Rl),
            0x16 => self.rotate(bus, Address::HL, Rotate::Rl),
            0x17 => self.rotate(bus, Reg::A, Rotate::Rr),
            0x18 => self.rotate(bus, Reg::B, Rotate::Rr),
            0x19 => self.rotate(bus, Reg::C, Rotate::Rr),
            0x1A => self.rotate(bus, Reg::D, Rotate::Rr),
            0x1B => self.rotate(bus, Reg::E, Rotate::Rr),
            0x1C => self.rotate(bus, Reg::H, Rotate::Rr),
            0x1D => self.rotate(bus, Reg::L, Rotate::Rr),
            0x1E => self.rotate(bus, Address::HL, Rotate::Rr),
            0x1F => self.rotate(bus, Reg::A, Rotate::Rr),
            0x20 => self.sla(bus, Reg::B),
            0x21 => self.sla(bus, Reg::C),
            0x22 => self.sla(bus, Reg::D),
            0x23 => self.sla(bus, Reg::E),
            0x24 => self.sla(bus, Reg::H),
            0x25 => self.sla(bus, Reg::L),
            0x26 => self.sla(bus, Address::HL),
            0x27 => self.sla(bus, Reg::A),
            0x28 => self.sra(bus, Reg::B),
            0x29 => self.sra(bus, Reg::C),
            0x2A => self.sra(bus, Reg::D),
            0x2B => self.sra(bus, Reg::E),
            0x2C => self.sra(bus, Reg::H),
            0x2D => self.sra(bus, Reg::L),
            0x2E => self.sra(bus, Address::HL),
            0x2F => self.sra(bus, Reg::A),
            0x30 => self.swap(bus, Reg::B),
            0x31 => self.swap(bus, Reg::C),
            0x32 => self.swap(bus, Reg::D),
            0x33 => self.swap(bus, Reg::E),
            0x34 => self.swap(bus, Reg::H),
            0x35 => self.swap(bus, Reg::L),
            0x36 => self.swap(bus, Address::HL),
            0x37 => self.swap(bus, Reg::A),
            0x38 => self.srl(bus, Reg::B),
            0x39 => self.srl(bus, Reg::C),
            0x3A => self.srl(bus, Reg::D),
            0x3B => self.srl(bus, Reg::E),
            0x3C => self.srl(bus, Reg::H),
            0x3D => self.srl(bus, Reg::L),
            0x3E => self.srl(bus, Address::HL),
            0x3F => self.srl(bus, Reg::A),
            0x40 => self.bit(bus, Reg::B, 0),
            0x41 => self.bit(bus, Reg::C, 0),
            0x42 => self.bit(bus, Reg::D, 0),
            0x43 => self.bit(bus, Reg::E, 0),
            0x44 => self.bit(bus, Reg::H, 0),
            0x45 => self.bit(bus, Reg::L, 0),
            0x46 => self.bit(bus, Address::HL, 0),
            0x47 => self.bit(bus, Reg::A, 0),
            0x48 => self.bit(bus, Reg::B, 1),
            0x49 => self.bit(bus, Reg::C, 1),
            0x4A => self.bit(bus, Reg::D, 1),
            0x4B => self.bit(bus, Reg::E, 1),
            0x4C => self.bit(bus, Reg::H, 1),
            0x4D => self.bit(bus, Reg::L, 1),
            0x4E => self.bit(bus, Address::HL, 1),
            0x4F => self.bit(bus, Reg::A, 1),
            0x50 => self.bit(bus, Reg::B, 2),
            0x51 => self.bit(bus, Reg::C, 2),
            0x52 => self.bit(bus, Reg::D, 2),
            0x53 => self.bit(bus, Reg::E, 2),
            0x54 => self.bit(bus, Reg::H, 2),
            0x55 => self.bit(bus, Reg::L, 2),
            0x56 => self.bit(bus, Address::HL, 2),
            0x57 => self.bit(bus, Reg::A, 2),
            0x58 => self.bit(bus, Reg::B, 3),
            0x59 => self.bit(bus, Reg::C, 3),
            0x5A => self.bit(bus, Reg::D, 3),
            0x5B => self.bit(bus, Reg::E, 3),
            0x5C => self.bit(bus, Reg::H, 3),
            0x5D => self.bit(bus, Reg::L, 3),
            0x5E => self.bit(bus, Address::HL, 3),
            0x5F => self.bit(bus, Reg::A, 3),
            0x60 => self.bit(bus, Reg::B, 4),
            0x61 => self.bit(bus, Reg::C, 4),
            0x62 => self.bit(bus, Reg::D, 4),
            0x63 => self.bit(bus, Reg::E, 4),
            0x64 => self.bit(bus, Reg::H, 4),
            0x65 => self.bit(bus, Reg::L, 4),
            0x66 => self.bit(bus, Address::HL, 4),
            0x67 => self.bit(bus, Reg::A, 4),
            0x68 => self.bit(bus, Reg::B, 5),
            0x69 => self.bit(bus, Reg::C, 5),
            0x6A => self.bit(bus, Reg::D, 5),
            0x6B => self.bit(bus, Reg::E, 5),
            0x6C => self.bit(bus, Reg::H, 5),
            0x6D => self.bit(bus, Reg::L, 5),
            0x6E => self.bit(bus, Address::HL, 5),
            0x6F => self.bit(bus, Reg::A, 5),
            0x70 => self.bit(bus, Reg::B, 6),
            0x71 => self.bit(bus, Reg::C, 6),
            0x72 => self.bit(bus, Reg::D, 6),
            0x73 => self.bit(bus, Reg::E, 6),
            0x74 => self.bit(bus, Reg::H, 6),
            0x75 => self.bit(bus, Reg::L, 6),
            0x76 => self.bit(bus, Address::HL, 6),
            0x77 => self.bit(bus, Reg::A, 6),
            0x78 => self.bit(bus, Reg::B, 7),
            0x79 => self.bit(bus, Reg::C, 7),
            0x7A => self.bit(bus, Reg::D, 7),
            0x7B => self.bit(bus, Reg::E, 7),
            0x7C => self.bit(bus, Reg::H, 7),
            0x7D => self.bit(bus, Reg::L, 7),
            0x7E => self.bit(bus, Address::HL, 7),
            0x7F => self.bit(bus, Reg::A, 7),
            0x80 => self.res(bus, Reg::B, 0),
            0x81 => self.res(bus, Reg::C, 0),
            0x82 => self.res(bus, Reg::D, 0),
            0x83 => self.res(bus, Reg::E, 0),
            0x84 => self.res(bus, Reg::H, 0),
            0x85 => self.res(bus, Reg::L, 0),
            0x86 => self.res(bus, Address::HL, 0),
            0x87 => self.res(bus, Reg::A, 0),
            0x88 => self.res(bus, Reg::B, 1),
            0x89 => self.res(bus, Reg::C, 1),
            0x8A => self.res(bus, Reg::D, 1),
            0x8B => self.res(bus, Reg::E, 1),
            0x8C => self.res(bus, Reg::H, 1),
            0x8D => self.res(bus, Reg::L, 1),
            0x8E => self.res(bus, Address::HL, 1),
            0x8F => self.res(bus, Reg::A, 1),
            0x90 => self.res(bus, Reg::B, 2),
            0x91 => self.res(bus, Reg::C, 2),
            0x92 => self.res(bus, Reg::D, 2),
            0x93 => self.res(bus, Reg::E, 2),
            0x94 => self.res(bus, Reg::H, 2),
            0x95 => self.res(bus, Reg::L, 2),
            0x96 => self.res(bus, Address::HL, 2),
            0x97 => self.res(bus, Reg::A, 2),
            0x98 => self.res(bus, Reg::B, 3),
            0x99 => self.res(bus, Reg::C, 3),
            0x9A => self.res(bus, Reg::D, 3),
            0x9B => self.res(bus, Reg::E, 3),
            0x9C => self.res(bus, Reg::H, 3),
            0x9D => self.res(bus, Reg::L, 3),
            0x9E => self.res(bus, Address::HL, 3),
            0x9F => self.res(bus, Reg::A, 3),
            0xA0 => self.res(bus, Reg::B, 4),
            0xA1 => self.res(bus, Reg::C, 4),
            0xA2 => self.res(bus, Reg::D, 4),
            0xA3 => self.res(bus, Reg::E, 4),
            0xA4 => self.res(bus, Reg::H, 4),
            0xA5 => self.res(bus, Reg::L, 4),
            0xA6 => self.res(bus, Address::HL, 4),
            0xA7 => self.res(bus, Reg::A, 4),
            0xA8 => self.res(bus, Reg::B, 5),
            0xA9 => self.res(bus, Reg::C, 5),
            0xAA => self.res(bus, Reg::D, 5),
            0xAB => self.res(bus, Reg::E, 5),
            0xAC => self.res(bus, Reg::H, 5),
            0xAD => self.res(bus, Reg::L, 5),
            0xAE => self.res(bus, Address::HL, 5),
            0xAF => self.res(bus, Reg::A, 5),
            0xB0 => self.res(bus, Reg::B, 6),
            0xB1 => self.res(bus, Reg::C, 6),
            0xB2 => self.res(bus, Reg::D, 6),
            0xB3 => self.res(bus, Reg::E, 6),
            0xB4 => self.res(bus, Reg::H, 6),
            0xB5 => self.res(bus, Reg::L, 6),
            0xB6 => self.res(bus, Address::HL, 6),
            0xB7 => self.res(bus, Reg::A, 6),
            0xB8 => self.res(bus, Reg::B, 7),
            0xB9 => self.res(bus, Reg::C, 7),
            0xBA => self.res(bus, Reg::D, 7),
            0xBB => self.res(bus, Reg::E, 7),
            0xBC => self.res(bus, Reg::H, 7),
            0xBD => self.res(bus, Reg::L, 7),
            0xBE => self.res(bus, Address::HL, 7),
            0xBF => self.res(bus, Reg::A, 7),
            0xC0 => self.set(bus, Reg::B, 0),
            0xC1 => self.set(bus, Reg::C, 0),
            0xC2 => self.set(bus, Reg::D, 0),
            0xC3 => self.set(bus, Reg::E, 0),
            0xC4 => self.set(bus, Reg::H, 0),
            0xC5 => self.set(bus, Reg::L, 0),
            0xC6 => self.set(bus, Address::HL, 0),
            0xC7 => self.set(bus, Reg::A, 0),
            0xC8 => self.set(bus, Reg::B, 1),
            0xC9 => self.set(bus, Reg::C, 1),
            0xCA => self.set(bus, Reg::D, 1),
            0xCB => self.set(bus, Reg::E, 1),
            0xCC => self.set(bus, Reg::H, 1),
            0xCD => self.set(bus, Reg::L, 1),
            0xCE => self.set(bus, Address::HL, 1),
            0xCF => self.set(bus, Reg::A, 1),
            0xD0 => self.set(bus, Reg::B, 2),
            0xD1 => self.set(bus, Reg::C, 2),
            0xD2 => self.set(bus, Reg::D, 2),
            0xD3 => self.set(bus, Reg::E, 2),
            0xD4 => self.set(bus, Reg::H, 2),
            0xD5 => self.set(bus, Reg::L, 2),
            0xD6 => self.set(bus, Address::HL, 2),
            0xD7 => self.set(bus, Reg::A, 2),
            0xD8 => self.set(bus, Reg::B, 3),
            0xD9 => self.set(bus, Reg::C, 3),
            0xDA => self.set(bus, Reg::D, 3),
            0xDB => self.set(bus, Reg::E, 3),
            0xDC => self.set(bus, Reg::H, 3),
            0xDD => self.set(bus, Reg::L, 3),
            0xDE => self.set(bus, Address::HL, 3),
            0xDF => self.set(bus, Reg::A, 3),
            0xE0 => self.set(bus, Reg::B, 4),
            0xE1 => self.set(bus, Reg::C, 4),
            0xE2 => self.set(bus, Reg::D, 4),
            0xE3 => self.set(bus, Reg::E, 4),
            0xE4 => self.set(bus, Reg::H, 4),
            0xE5 => self.set(bus, Reg::L, 4),
            0xE6 => self.set(bus, Address::HL, 4),
            0xE7 => self.set(bus, Reg::A, 4),
            0xE8 => self.set(bus, Reg::B, 5),
            0xE9 => self.set(bus, Reg::C, 5),
            0xEA => self.set(bus, Reg::D, 5),
            0xEB => self.set(bus, Reg::E, 5),
            0xEC => self.set(bus, Reg::H, 5),
            0xED => self.set(bus, Reg::L, 5),
            0xEE => self.set(bus, Address::HL, 5),
            0xEF => self.set(bus, Reg::A, 5),
            0xF0 => self.set(bus, Reg::B, 6),
            0xF1 => self.set(bus, Reg::C, 6),
            0xF2 => self.set(bus, Reg::D, 6),
            0xF3 => self.set(bus, Reg::E, 6),
            0xF4 => self.set(bus, Reg::H, 6),
            0xF5 => self.set(bus, Reg::L, 6),
            0xF6 => self.set(bus, Address::HL, 6),
            0xF7 => self.set(bus, Reg::A, 6),
            0xF8 => self.set(bus, Reg::B, 7),
            0xF9 => self.set(bus, Reg::C, 7),
            0xFA => self.set(bus, Reg::D, 7),
            0xFB => self.set(bus, Reg::E, 7),
            0xFC => self.set(bus, Reg::H, 7),
            0xFD => self.set(bus, Reg::L, 7),
            0xFE => self.set(bus, Address::HL, 7),
            0xFF => self.set(bus, Reg::A, 7),
        }
    }
}
