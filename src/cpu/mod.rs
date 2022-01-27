use self::{
    instructions::{Dst, Src},
    registers::{Flags, Registers},
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

    fn inc_pc(&mut self) {
        self.regs.pc += 1;
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

    fn load<I, D, S>(&mut self, bus: &mut I, dst: D, src: S)
    where
        I: Interface,
        D: Copy,
        S: Copy,
        Self: Dst<D> + Src<S>,
    {
        let data = self.read(bus, src);
        self.write(bus, dst, data);
    }
}
