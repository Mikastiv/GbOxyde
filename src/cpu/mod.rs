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
