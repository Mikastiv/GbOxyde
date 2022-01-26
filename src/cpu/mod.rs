use self::registers::Registers;

mod registers;

pub struct Cpu {
    cur_inst: u8,
    regs: Registers,
    halted: bool,
    ime: bool,
    cycles: u64,
}

pub trait Interface {}

impl Cpu {
    pub fn new() -> Self {
        Self {
            cur_inst: 0x00,
            regs: Registers::new(),
            halted: false,
            ime: false,
            cycles: 0,
        }
    }

    pub fn execute(&mut self, bus: &mut impl Interface) {}
}
