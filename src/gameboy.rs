use sdl2::{event::Event, keyboard::Keycode, EventPump};

use crate::{
    bus::{cartridge::Cartridge, joypad::GbButton, Bus},
    cpu::Cpu,
};

pub struct Gameboy {
    cpu: Cpu,
    bus: Bus,
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Self {
        let cartridge = Cartridge::new(rom);
        cartridge.print_header();

        Self {
            cpu: Cpu::new(),
            bus: Bus::new(cartridge),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let window = video
            .window("GbOxyde", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let mut event_pump = sdl_context.event_pump()?;
        loop {
            if !self.process_events(&mut event_pump) {
                return Ok(());
            }
            self.cpu.step(&mut self.bus);
        }
    }

    fn process_events(&mut self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(button) = map_key(key) {
                        self.bus.keydown(button);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(button) = map_key(key) {
                        self.bus.keyup(button);
                    }
                }
                _ => {}
            }
        }

        true
    }
}

fn map_key(key: Keycode) -> Option<GbButton> {
    match key {
        Keycode::A => Some(GbButton::A),
        Keycode::S => Some(GbButton::B),
        Keycode::Z => Some(GbButton::Start),
        Keycode::X => Some(GbButton::Select),
        Keycode::Right => Some(GbButton::Right),
        Keycode::Left => Some(GbButton::Left),
        Keycode::Up => Some(GbButton::Up),
        Keycode::Down => Some(GbButton::Down),
        _ => None,
    }
}
