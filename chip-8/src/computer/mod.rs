mod cpu;
mod display;
mod keyboard;
mod ram;
use cpu::{CHIP8_ROM_START, Cpu};
use display::Display;
use keyboard::Keyboard;
use ram::Ram;
use std::fs;

#[derive(Debug)]
pub struct Chip8 {
    pub cpu: Cpu,
    pub ram: Ram,
    pub display: Display,
    pub keyboard: Keyboard,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn load_rom(&mut self, file_path: &str) {
        let rom_data = fs::read(file_path).expect("Failed to read ROM file !");
        let start: usize = CHIP8_ROM_START.into();
        let end = start + rom_data.len();
        self.ram.memory[start..end].copy_from_slice(&rom_data);
    }
}
