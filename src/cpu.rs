use std::{fs::{File}, error::Error, io::Read};

#[derive(Debug)]
struct RegPair {
    left: u8,
    right: u8
}

impl RegPair {
    fn new(left: u8, right: u8) -> Self {
        Self { left, right }
    }

    fn as_u16(&self) -> u16 {
        ((self.left as u16) << 8) | self.right as u16
    }
}


#[derive(Debug)]
struct Registers {
    af: RegPair,
    bc: RegPair,
    de: RegPair,
    hl: RegPair,
}

impl Registers {
    fn new() -> Self {
        Self { 
            af: RegPair::new(0, 0), 
            bc: RegPair::new(0, 0), 
            de: RegPair::new(0, 0), 
            hl: RegPair::new(0, 0),
        }
    }
}

#[derive(Debug)]
pub struct Intel8080 {
    reg: Registers,
    pc: u16,
    sp: u16,
    mem: [u8; 65536]
}

impl Intel8080 {
    pub fn new() -> Self {
        Self { 
            reg: Registers::new(), 
            pc: 0x00, 
            sp: 0x00, 
            mem: [0; 65536]
        }
    }

    pub fn load_rom(&mut self, file_path: &str) -> Result<(), Box<dyn Error>>{
        let mut f = File::open(file_path)?;
        f.read(&mut self.mem)?;
        Ok(())
    }

    pub fn disassemble(&self) {
        
    }
}