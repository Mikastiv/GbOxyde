use std::{fs::File, io::Read};

use anyhow::Result;
use gboxyde::gameboy::Gameboy;

fn main() -> Result<()> {
    let romfile = std::env::args().nth(1).expect("Missing argument");
    let mut file = File::open(romfile)?;
    let mut rom = vec![];
    file.read_to_end(&mut rom)?;

    let mut gb = Gameboy::new(rom);
    if let Err(e) = gb.run() {
        eprintln!("{e}");
    }

    Ok(())
}
