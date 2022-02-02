use bitflags::bitflags;

use super::interrupts::{InterruptFlag, Interrupts};

bitflags! {
    pub struct Buttons: u8 {
        const A      = 1 << 0;
        const B      = 1 << 1;
        const SELECT = 1 << 2;
        const START  = 1 << 3;
    }
}

bitflags! {
    pub struct DPad: u8 {
        const RIGHT  = 1 << 0;
        const LEFT   = 1 << 1;
        const UP     = 1 << 2;
        const DOWN   = 1 << 3;
    }
}

pub enum GbButton {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

const SELECT_DPAD: u8 = 1 << 4;
const SELECT_BUTTONS: u8 = 1 << 5;

pub struct Joypad {
    buttons: Buttons,
    dpad: DPad,
    register: u8,
}

impl Joypad {
    pub const fn new() -> Self {
        Self {
            buttons: Buttons::all(),
            dpad: DPad::all(),
            register: 0xCF,
        }
    }

    pub const fn read(&self) -> u8 {
        let mut value = self.register & 0xF0;
        if self.register & SELECT_BUTTONS == 0 {
            value |= self.buttons.bits;
        }
        if self.register & SELECT_DPAD == 0 {
            value |= self.dpad.bits;
        }
        value
    }

    pub fn write(&mut self, data: u8) {
        self.register = data & 0x3F;
        self.register |= 0xC0;
    }

    pub fn keyup(&mut self, button: GbButton) {
        match button {
            GbButton::A => self.buttons.insert(Buttons::A),
            GbButton::B => self.buttons.insert(Buttons::B),
            GbButton::Select => self.buttons.insert(Buttons::SELECT),
            GbButton::Start => self.buttons.insert(Buttons::START),
            GbButton::Right => self.dpad.insert(DPad::RIGHT),
            GbButton::Left => self.dpad.insert(DPad::LEFT),
            GbButton::Up => self.dpad.insert(DPad::UP),
            GbButton::Down => self.dpad.insert(DPad::DOWN),
        }
    }

    pub fn keydown(&mut self, button: GbButton, intr: &mut Interrupts) {
        match button {
            GbButton::A => self.buttons.remove(Buttons::A),
            GbButton::B => self.buttons.remove(Buttons::B),
            GbButton::Select => self.buttons.remove(Buttons::SELECT),
            GbButton::Start => self.buttons.remove(Buttons::START),
            GbButton::Right => self.dpad.remove(DPad::RIGHT),
            GbButton::Left => self.dpad.remove(DPad::LEFT),
            GbButton::Up => self.dpad.remove(DPad::UP),
            GbButton::Down => self.dpad.remove(DPad::DOWN),
        }
        intr.request(InterruptFlag::JOYPAD);
    }
}
