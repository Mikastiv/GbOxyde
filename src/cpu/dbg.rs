use std::ffi::CString;

use super::Interface;

pub struct Dbg {
    msg: [u8; 1024],
    size: usize,
}

impl Dbg {
    pub const fn new() -> Self {
        Self {
            msg: [0; 1024],
            size: 0,
        }
    }

    pub fn update(&mut self, bus: &mut impl Interface) {
        if bus.peek(0xFF02) == 0x81 {
            let c = bus.peek(0xFF01);
            self.msg[self.size] = c;
            self.size += 1;
            bus.set(0xFF02, 0);
        }
    }

    pub fn print(&self) {
        if self.msg[0] != 0 {
            let s = CString::new(&self.msg[..self.size]);
            if let Ok(msg) = s {
                if let Ok(s) = msg.to_str() {
                    if s.contains("Failed") || s.contains("Passed") {
                        println!("DBG: {s:}");
                        panic!();
                    }
                }
            }
        }
    }
}
