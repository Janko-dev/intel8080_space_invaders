use std::error::Error;

use space_invaders::Emulator;

fn main() -> Result<(), Box<dyn Error>>{
    let mut emu = Emulator::new();

    emu.load_rom("test")?;
    emu.run();
    
    Ok(())
}
