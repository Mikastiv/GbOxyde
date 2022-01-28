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
        let value = bus.read(self.regs.pc);
        self.inc_pc();
        value
    }

    fn imm_word(&mut self, bus: &mut impl Interface) -> u16 {
        let lo = self.imm(bus);
        let hi = self.imm(bus);
        u16::from_le_bytes([lo, hi])
    }

    fn push(&mut self, bus: &mut impl Interface, value: u16) {
        self.regs.dec_sp();
        bus.write(self.regs.sp, (value >> 8) as u8);
        self.regs.dec_sp();
        bus.write(self.regs.sp, value as u8);
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
        let value = self.read(bus, src);
        self.write(bus, dst, value);
    }

    fn ld_d16(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let value = self.imm_word(bus);
        self.regs.write16(dst, value);
    }

    fn ld_sp_hl(&mut self, bus: &mut impl Interface) {
        self.regs.sp = self.regs.hl();
        bus.tick(1);
    }

    fn ld_mem_d16_sp(&mut self, bus: &mut impl Interface) {
        let address = self.imm_word(bus);
        let value = self.regs.sp;
        bus.write(address, value as u8);
        bus.write(address.wrapping_add(1), (value >> 8) as u8);
    }

    fn ld_hl_sp_d8(&mut self, bus: &mut impl Interface) {
        let value = self.imm(bus) as i8 as i16 as u16;
        let sp = self.regs.sp;

        let carry = (sp & 0x00FF) + (value & 0x00FF) > 0x00FF;
        let half_carry = (sp & 0x000F) + (value & 0x000F) > 0x000F;
        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H, half_carry);
        self.regs.set_flags(Flags::Z | Flags::N, false);

        self.regs.set_hl(sp.wrapping_add(value));
        bus.tick(1);
    }

    fn push16(&mut self, bus: &mut impl Interface, src: Reg16) {
        bus.tick(1);
        let value = self.regs.read16(src);
        self.push(bus, value);
    }

    fn pop16(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let value = self.pop(bus);
        self.regs.write16(dst, value);
    }

    fn alu_add(&mut self, value: u8, cy: bool) -> u8 {
        let a = self.regs.a;
        let cy = cy as u8;
        let result = u16::from(a) + u16::from(value) + u16::from(cy);

        let carry = result > 0xFF;
        let half_carry = (a & 0x0F) + (value & 0x0F) + cy > 0x0F;
        self.regs.set_flags(Flags::Z, result as u8 == 0);
        self.regs.set_flags(Flags::N, false);
        self.regs.set_flags(Flags::H, half_carry);
        self.regs.set_flags(Flags::C, carry);

        result as u8
    }

    fn add<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        self.regs.a = self.alu_add(value, false);
    }

    fn add16(&mut self, bus: &mut impl Interface, src: Reg16) {
        let value = self.regs.read16(src);
        let hl = self.regs.hl();

        let (result, carry) = hl.overflowing_add(value);
        let half_carry = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::N, false);
        self.regs.set_flags(Flags::H, half_carry);

        self.regs.set_hl(result);
        bus.tick(1);
    }

    fn add_sp_d8(&mut self, bus: &mut impl Interface) {
        let value = self.imm(bus) as i8 as i16 as u16;
        let sp = self.regs.sp;

        let carry = (sp & 0x00FF) + (value & 0x00FF) > 0x00FF;
        let half_carry = (sp & 0x000F) + (value & 0x000F) > 0x000F;
        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H, half_carry);
        self.regs.set_flags(Flags::Z | Flags::N, false);

        self.regs.sp = sp.wrapping_add(value);
        bus.tick(2);
    }

    fn adc<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        self.regs.a = self.alu_add(value, self.regs.c());
    }

    fn alu_sub(&mut self, value: u8, cy: bool) -> u8 {
        let a = self.regs.a;
        let cy = cy as u8;
        let result = a.wrapping_sub(value).wrapping_sub(cy);

        let carry = u16::from(a) < u16::from(value) + u16::from(cy);
        let half_carry = (a & 0x0F) < (value & 0x0F) + (cy & 0x0F);
        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N, true);
        self.regs.set_flags(Flags::H, half_carry);
        self.regs.set_flags(Flags::C, carry);

        result
    }

    fn sub<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        self.regs.a = self.alu_sub(value, false);
    }

    fn sbc<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        self.regs.a = self.alu_sub(value, self.regs.c());
    }

    fn and<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        let result = self.regs.a & value;

        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N | Flags::C, false);
        self.regs.set_flags(Flags::H, true);

        self.regs.a = result;
    }

    fn or<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        let result = self.regs.a | value;

        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N | Flags::C | Flags::H, false);

        self.regs.a = result;
    }

    fn xor<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        let result = self.regs.a ^ value;

        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N | Flags::C | Flags::H, false);

        self.regs.a = result;
    }

    fn cp<I, S>(&mut self, bus: &mut I, src: S)
    where
        I: Interface,
        S: Copy,
        Self: Src<S>,
    {
        let value = self.read(bus, src);
        self.alu_sub(value, false);
    }

    fn inc<I, D>(&mut self, bus: &mut I, dst: D)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let result = value.wrapping_add(1);

        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N, false);
        self.regs.set_flags(Flags::H, (value & 0x0F) + 0x01 > 0x0F);

        self.write(bus, dst, result);
    }

    fn inc16(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let value = self.regs.read16(dst);
        self.regs.write16(dst, value.wrapping_add(1));
        bus.tick(1);
    }

    fn dec<I, D>(&mut self, bus: &mut I, dst: D)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let result = value.wrapping_sub(1);

        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N, false);
        self.regs.set_flags(Flags::H, (value & 0x0F) + 0x01 > 0x0F);

        self.write(bus, dst, result);
    }

    fn dec16(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let value = self.regs.read16(dst);
        self.regs.write16(dst, value.wrapping_sub(1));
        bus.tick(1);
    }
}
