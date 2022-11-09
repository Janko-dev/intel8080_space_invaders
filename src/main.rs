use std::error::Error;

use intel8080::Emulator;

fn main() -> Result<(), Box<dyn Error>>{
    let mut emu = Emulator::new();
    emu.load_rom("test")?;
    emu.run();
    Ok(())
}
