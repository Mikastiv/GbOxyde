use crate::bus::interrupts::InterruptFlag;

use self::{
    dbg::Dbg,
    instructions::{Condition, Dst, Rotate, Src},
    registers::{Flags, Reg16, Registers},
};

mod dbg;
mod instructions;
mod registers;
mod rw;
mod trace;

pub struct Cpu {
    cur_opcode: u8,
    regs: Registers,
    halted: bool,
    ime: bool,
    enabling_ime: bool,
    dbg: Dbg,
}

pub trait Interface {
    fn peek(&self, address: u16) -> u8;
    fn read(&mut self, address: u16) -> u8 {
        let data = self.peek(address);
        self.tick(1);
        data
    }
    fn set(&mut self, address: u16, data: u8);
    fn write(&mut self, address: u16, data: u8) {
        self.set(address, data);
        self.tick(1);
    }
    fn tick(&mut self, count: usize);
    fn cycles(&self) -> u64;
    fn check_interrupts(&self) -> InterruptFlag;
    fn interrupt_handled(&mut self, intr: InterruptFlag);
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            cur_opcode: 0x00,
            regs: Registers::new(),
            halted: false,
            ime: false,
            enabling_ime: false,
            dbg: Dbg::new(),
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
        // Last instruction was DI
        if self.cur_opcode == 0xF3 {
            self.fetch_instruction(bus);
            self.execute(bus);
            return;
        }

        match self.halted {
            true => {
                bus.tick(1);
                if !bus.check_interrupts().is_empty() {
                    self.halted = false;
                }
            }
            false => {
                // self.dbg.update(bus);
                // self.dbg.print();
                self.fetch_instruction(bus);
                self.execute(bus);
            }
        }

        if self.ime {
            self.handle_interrupts(bus);
            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.ime = true;
        }
    }

    fn handle_interrupts(&mut self, bus: &mut impl Interface) {
        let flags = bus.check_interrupts();
        let intr = flags.bits() & (!flags.bits()).wrapping_add(1);
        let intr = InterruptFlag::from_bits_truncate(intr);

        if intr.is_empty() {
            return;
        }

        let address = match intr {
            InterruptFlag::VBLANK => 0x40,
            InterruptFlag::STAT => 0x48,
            InterruptFlag::TIMER => 0x50,
            InterruptFlag::SERIAL => 0x58,
            InterruptFlag::JOYPAD => 0x60,
            _ => panic!("No interrupts"),
        };

        self.ime = false;
        self.halted = false;

        bus.tick(1);

        self.stack_push(bus, self.regs.pc);
        self.regs.pc = address;
        bus.interrupt_handled(intr);
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

    fn stack_push(&mut self, bus: &mut impl Interface, value: u16) {
        bus.tick(1);
        self.regs.dec_sp();
        bus.write(self.regs.sp, (value >> 8) as u8);
        self.regs.dec_sp();
        bus.write(self.regs.sp, value as u8);
    }

    fn stack_pop(&mut self, bus: &mut impl Interface) -> u16 {
        let lo = bus.read(self.regs.sp);
        self.regs.inc_sp();
        let hi = bus.read(self.regs.sp);
        self.regs.inc_sp();
        u16::from_le_bytes([lo, hi])
    }

    fn inc_pc(&mut self) {
        self.regs.pc = self.regs.pc.wrapping_add(1);
    }

    fn check_cond(&self, cond: Condition) -> bool {
        match cond {
            Condition::NZ => !self.regs.zf(),
            Condition::Z => self.regs.zf(),
            Condition::NC => !self.regs.cf(),
            Condition::C => self.regs.cf(),
        }
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

    fn ld16(&mut self, bus: &mut impl Interface, dst: Reg16) {
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

    fn push(&mut self, bus: &mut impl Interface, src: Reg16) {
        let value = self.regs.read16(src);
        self.stack_push(bus, value);
    }

    fn pop(&mut self, bus: &mut impl Interface, dst: Reg16) {
        let value = self.stack_pop(bus);
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
        self.regs.a = self.alu_add(value, self.regs.cf());
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
        self.regs.a = self.alu_sub(value, self.regs.cf());
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

    fn nop(&self) {}

    fn stop(&self) {
        panic!("Stop instruction");
    }

    fn halt(&mut self) {
        self.halted = true;
    }

    fn ccf(&mut self) {
        self.regs.set_flags(Flags::N | Flags::H, false);
        self.regs.set_flags(Flags::C, !self.regs.cf());
    }

    fn scf(&mut self) {
        self.regs.set_flags(Flags::N | Flags::H, false);
        self.regs.set_flags(Flags::C, true);
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn ei(&mut self) {
        self.enabling_ime = true;
    }

    fn daa(&mut self) {
        let a = self.regs.a;
        let n = self.regs.nf();
        let c = self.regs.cf();
        let h = self.regs.hf();

        let mut adjust = 0x00;
        let mut carry = false;

        if h || (!n && (a & 0x0F) > 0x09) {
            adjust |= 0x06;
        }

        if c || (!n && a > 0x99) {
            adjust |= 0x60;
            carry = true;
        }

        match n {
            true => self.regs.a = a.wrapping_sub(adjust),
            false => self.regs.a = a.wrapping_add(adjust),
        }

        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H, false);
        self.regs.set_flags(Flags::Z, self.regs.a == 0);
    }

    fn cpl(&mut self) {
        self.regs.a = !self.regs.a;
        self.regs.set_flags(Flags::N | Flags::H, true);
    }

    fn alu_rl(&mut self, value: u8, cy: bool) -> u8 {
        let carry = (value & 0x80) != 0x00;
        let result = (value << 1) | u8::from(cy);

        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::N | Flags::H, false);
        self.regs.set_flags(Flags::Z, result == 0);

        result
    }

    fn alu_rr(&mut self, value: u8, cy: bool) -> u8 {
        let carry = (value & 0x01) != 0x00;
        let result = (u8::from(cy) << 7) | (value >> 1);

        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::N | Flags::H, false);
        self.regs.set_flags(Flags::Z, result == 0);

        result
    }

    fn rotate_a(&mut self, r: Rotate) {
        let value = self.regs.a;
        self.regs.a = match r {
            Rotate::Rlc => self.alu_rl(value, (value & 0x80) != 0x00),
            Rotate::Rl => self.alu_rl(value, self.regs.cf()),
            Rotate::Rrc => self.alu_rr(value, (value & 0x01) != 0x00),
            Rotate::Rr => self.alu_rr(value, self.regs.cf()),
        };
        self.regs.set_flags(Flags::Z, false);
    }

    fn rotate<I, D>(&mut self, bus: &mut I, dst: D, r: Rotate)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let result = match r {
            Rotate::Rlc => self.alu_rl(value, (value & 0x80) != 0x00),
            Rotate::Rl => self.alu_rl(value, self.regs.cf()),
            Rotate::Rrc => self.alu_rr(value, (value & 0x01) != 0x00),
            Rotate::Rr => self.alu_rr(value, self.regs.cf()),
        };
        self.write(bus, dst, result);
    }

    fn jump(&mut self, bus: &mut impl Interface, address: u16) {
        self.regs.pc = address;
        bus.tick(1);
    }

    fn jp(&mut self, bus: &mut impl Interface) {
        let address = self.imm_word(bus);
        self.jump(bus, address);
    }

    fn jp_hl(&mut self) {
        self.regs.pc = self.regs.hl();
    }

    fn jp_cond(&mut self, bus: &mut impl Interface, cond: Condition) {
        let address = self.imm_word(bus);
        if self.check_cond(cond) {
            self.jump(bus, address);
        }
    }

    fn jump_relative(&mut self, bus: &mut impl Interface, offset: i8) {
        let address = self.regs.pc.wrapping_add(offset as i16 as u16);
        self.jump(bus, address);
    }

    fn jr(&mut self, bus: &mut impl Interface) {
        let offset = self.imm(bus) as i8;
        self.jump_relative(bus, offset);
    }

    fn jr_cond(&mut self, bus: &mut impl Interface, cond: Condition) {
        let offset = self.imm(bus) as i8;
        if self.check_cond(cond) {
            self.jump_relative(bus, offset);
        }
    }

    fn call_func(&mut self, bus: &mut impl Interface, address: u16) {
        self.stack_push(bus, self.regs.pc);
        self.regs.pc = address;
    }

    fn call(&mut self, bus: &mut impl Interface) {
        let address = self.imm_word(bus);
        self.call_func(bus, address);
    }

    fn call_cond(&mut self, bus: &mut impl Interface, cond: Condition) {
        let address = self.imm_word(bus);
        if self.check_cond(cond) {
            self.call_func(bus, address);
        }
    }

    fn ret(&mut self, bus: &mut impl Interface) {
        let address = self.stack_pop(bus);
        self.jump(bus, address);
    }

    fn ret_cond(&mut self, bus: &mut impl Interface, cond: Condition) {
        bus.tick(1);
        if self.check_cond(cond) {
            self.ret(bus);
        }
    }

    fn reti(&mut self, bus: &mut impl Interface) {
        self.ret(bus);
        self.ime = true;
    }

    fn rst(&mut self, bus: &mut impl Interface) {
        let address = self.cur_opcode & 0x38;
        self.stack_push(bus, self.regs.pc);
        self.regs.pc = address as u16;
    }

    fn undefined(&self) {
        panic!("Undefined opcode: {:02X}", self.cur_opcode);
    }

    fn sla<I, D>(&mut self, bus: &mut I, dst: D)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let carry = (value & 0x80) != 0x00;
        let result = value << 1;

        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H | Flags::N, false);
        self.regs.set_flags(Flags::Z, result == 0);

        self.write(bus, dst, result);
    }

    fn sra<I, D>(&mut self, bus: &mut I, dst: D)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let carry = (value & 0x01) != 0x00;
        let hi = value & 0x80;
        let result = hi | (value >> 1);

        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H | Flags::N, false);
        self.regs.set_flags(Flags::Z, result == 0);

        self.write(bus, dst, result);
    }

    fn srl<I, D>(&mut self, bus: &mut I, dst: D)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let carry = (value & 0x01) != 0x00;
        let result = value >> 1;

        self.regs.set_flags(Flags::C, carry);
        self.regs.set_flags(Flags::H | Flags::N, false);
        self.regs.set_flags(Flags::Z, result == 0);

        self.write(bus, dst, result);
    }

    fn swap<I, D>(&mut self, bus: &mut I, dst: D)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let result = (value << 4) | (value >> 4);

        self.regs.set_flags(Flags::H | Flags::N | Flags::C, false);
        self.regs.set_flags(Flags::Z, result == 0);

        self.write(bus, dst, result);
    }

    fn bit<I, D>(&mut self, bus: &mut I, dst: D, bit: u8)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        let result = value & (1 << bit);

        self.regs.set_flags(Flags::Z, result == 0);
        self.regs.set_flags(Flags::N, false);
        self.regs.set_flags(Flags::H, true);
    }

    fn set<I, D>(&mut self, bus: &mut I, dst: D, bit: u8)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        self.write(bus, dst, value | (1 << bit));
    }

    fn res<I, D>(&mut self, bus: &mut I, dst: D, bit: u8)
    where
        I: Interface,
        D: Copy,
        Self: Dst<D> + Src<D>,
    {
        let value = self.read(bus, dst);
        self.write(bus, dst, value & !(1 << bit));
    }
}
