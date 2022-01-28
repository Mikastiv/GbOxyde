use self::{
    instructions::{Dst, Src},
    registers::{Flags, Reg16, Registers},
};

mod instructions;
mod registers;
mod trace;

pub struct Cpu {
    cur_opcode: u8,
    regs: Registers,
    halted: bool,
    ime: bool,
}

pub trait Interface {
    fn peek(&self, address: u16) -> u8;
    fn read(&mut self, address: u16) -> u8 {
        let data = self.peek(address);
        self.tick(1);

        data
    }
    fn write(&mut self, address: u16, data: u8);
    fn tick(&mut self, count: usize);
    fn cycles(&self) -> u64;
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            cur_opcode: 0x00,
            regs: Registers::new(),
            halted: false,
            ime: false,
        }
    }

    pub fn step_callback<I, C>(&mut self, bus: &mut I, mut callback: C)
    where
        I: Interface,
        C: FnMut(&mut Self, &mut I),
    {
        callback(self, bus);
        self.step(bus);
    }

    pub fn step(&mut self, bus: &mut impl Interface) {
        self.fetch_instruction(bus);
        self.execute(bus);
    }

    fn fetch_instruction(&mut self, bus: &mut impl Interface) {
        self.cur_opcode = bus.read(self.regs.pc);
        self.inc_pc();
    }

    fn imm(&mut self, bus: &mut impl Interface) -> u8 {
        let data = bus.read(self.regs.pc);
        self.inc_pc();
        data
    }

    fn imm_word(&mut self, bus: &mut impl Interface) -> u16 {
        let lo = self.imm(bus);
        let hi = self.imm(bus);
        u16::from_le_bytes([lo, hi])
    }

    fn push(&mut self, bus: &mut impl Interface, data: u16) {
        self.regs.dec_sp();
        bus.write(self.regs.sp, (data >> 8) as u8);
        self.regs.dec_sp();
        bus.write(self.regs.sp, data as u8);
    }

    fn pop(&mut self, bus: &mut impl Interface) -> u16 {
        let lo = bus.read(self.regs.sp);
        self.regs.inc_sp();
        let hi = bus.read(self.regs.sp);
        self.regs.inc_sp();
        u16::from_le_bytes([lo, hi])
    }

    fn inc_pc(&mut self) {
        self.regs.pc = self.regs.pc.wrapping_add(1);
    }

    fn nop(&self) {}

    fn stop(&self) {
        panic!("Stop instruction");
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    fn ccf(&mut self) {
        self.regs.set_flags(Flags::N | Flags::H, false);
        self.regs.set_flags(Flags::C, !self.regs.c());
    }

    fn scf(&mut self) {
        self.regs.set_flags(Flags::N | Flags::H, false);
        self.regs.set_flags(Flags::C, true);
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn ei(&mut self) {
        self.ime = true;
    }

    fn ld<I, D, S>(&mut self, bus: &mut I, dst: D, src: S)
    where
        I: Interface,
        D: Copy,
        S: Copy,
        Self: Dst<D> + Src<S>,
    {
        let data = self.read(bus, src);
        self.write(bus, dst, data);
    }

    fn ld_d16(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let data = self.imm_word(bus);
        self.regs.write16(dst, data);
    }

    fn ld_sp_hl(&mut self, bus: &mut impl Interface) {
        self.regs.sp = self.regs.hl();
        bus.tick(1);
    }

    fn ld_mem_d16_sp(&mut self, bus: &mut impl Interface) {
        let address = self.imm_word(bus);
        let data = self.regs.sp;
        bus.write(address, data as u8);
        bus.write(address.wrapping_add(1), (data >> 8) as u8);
    }

    fn ld_hl_sp_d8(&mut self, bus: &mut impl Interface) {
        let data = self.imm(bus) as i8 as i16 as u16;
        let sp = self.regs.sp;

        let carry = (sp & 0x00FF) + (data & 0x00FF) > 0x00FF;
        let half_carry = (sp & 0x000F) + (data & 0x000F) > 0x000F;
        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H, half_carry);
        self.regs.set_flags(Flags::Z | Flags::N, false);

        self.regs.set_hl(sp.wrapping_add(data));
        bus.tick(1);
    }

    fn push16(&mut self, bus: &mut impl Interface, src: Reg16) {
        bus.tick(1);
        let data = self.regs.read16(src);
        self.push(bus, data);
    }

    fn pop16(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let data = self.pop(bus);
        self.regs.write16(dst, data);
    }
}
