extern crate minifb;

use std::error::Error;
use minifb::{Key, Window, WindowOptions};

mod cpu;
mod opcodes;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Emulator {
    cpu: cpu::Intel8080,
    window: Window,
}

impl Emulator {
    pub fn new() -> Self {
        let mut window = Window::new(
            "Intel 8080 - Space invaders emulator", 
            WIDTH, 
            HEIGHT, 
            WindowOptions::default()
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            cpu: cpu::Intel8080::new(),
            window
        }
    }

    pub fn load_rom(&mut self, file_name: &str) -> Result<(), Box<dyn Error>>{
        self.cpu.load_rom(file_name)
    }

    pub fn run(&self){

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.cpu.disassemble();    
        }

    }
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}