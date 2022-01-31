use bitflags::bitflags;

bitflags! {
    pub struct InterruptFlag: u8 {
        const VBLANK = 1 << 0;
        const STAT = 1 << 1;
        const TIMER = 1 << 2;
        const SERIAL = 1 << 3;
        const JOYPAD = 1 << 4;
    }
}

pub struct Interrupts {
    flags: InterruptFlag,
    enable: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            flags: InterruptFlag::empty(),
            enable: 0x00,
        }
    }

    pub fn get_enable(&self) -> u8 {
        self.enable
    }

    pub fn set_enable(&mut self, data: u8) {
        self.enable = data;
    }

    pub fn check(&self) -> InterruptFlag {
        self.flags
            .intersection(InterruptFlag::from_bits_truncate(self.enable))
    }

    pub fn handled(&mut self, intr: InterruptFlag) {
        self.flags.remove(intr);
    }
}
