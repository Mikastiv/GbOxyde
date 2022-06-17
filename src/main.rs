use gboxyde::{gameboy::Gameboy, run_emulation};

use anyhow::Result;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let romfile = std::env::args().nth(1).expect("Missing argument");
    let mut file = File::open(romfile)?;
    let mut rom = vec![];
    file.read_to_end(&mut rom)?;

    run_emulation(Gameboy::new(rom))?;

    Ok(())
}
