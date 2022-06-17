use bus::joypad::GbButton;
use gameboy::Gameboy;

use anyhow::Result;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod gameboy;

mod bus;
mod cpu;

fn map_key(key: VirtualKeyCode) -> Option<GbButton> {
    match key {
        VirtualKeyCode::A => Some(GbButton::A),
        VirtualKeyCode::S => Some(GbButton::B),
        VirtualKeyCode::Z => Some(GbButton::Start),
        VirtualKeyCode::X => Some(GbButton::Select),
        VirtualKeyCode::Right => Some(GbButton::Right),
        VirtualKeyCode::Left => Some(GbButton::Left),
        VirtualKeyCode::Up => Some(GbButton::Up),
        VirtualKeyCode::Down => Some(GbButton::Down),
        _ => None,
    }
}

pub fn run_emulation(mut gb: Gameboy) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("GbOxyde")
        .with_inner_size(PhysicalSize::new(800i32, 600i32))
        .with_resizable(false)
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                let button = map_key(key);
                if let Some(button) = button {
                    match state {
                        ElementState::Pressed => gb.bus.keydown(button),
                        ElementState::Released => gb.bus.keyup(button),
                    }
                }
            }
            Event::MainEventsCleared => {
                gb.cpu.step(&mut gb.bus);
                window.request_redraw();
            }
            _ => (),
        }
    });
}
