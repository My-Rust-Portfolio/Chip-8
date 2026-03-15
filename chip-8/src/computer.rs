use std::fs;

// First 512bytes (0x200) are for system, hence CHIP-8 games always start from 0x200 address
const CHIP8_ROM_START: u16 = 0x200;

// Font data starts at byte 80 (0x50)
const CHIP8_RAM_FONTDATA_START: usize = 0x50;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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

#[derive(Debug)]
pub struct Ram {
    pub memory: [u8; 4096], // 4kb (4096b) of RAM
}

impl Ram {
    fn new() -> Self {
        let mut m = [0; 4096];
        m[CHIP8_RAM_FONTDATA_START..CHIP8_RAM_FONTDATA_START + FONTSET_SIZE]
            .copy_from_slice(&FONTSET);

        Self { memory: m }
    }
}

#[derive(Debug)]
pub struct Keyboard {
    pub keypad: [bool; 16],
}

impl Keyboard {
    fn new() -> Self {
        Self {
            keypad: [false; 16],
        }
    }
}

#[derive(Debug)]
pub struct Display {
    pub display: [bool; 64 * 32],
}

impl Display {
    fn new() -> Self {
        Self {
            display: [false; 64 * 32],
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub registeres: [u8; 16], // V0 to VF
    pub index_register: u16,  // points to memory/RAM location
    pub program_counter: u16, // pointer to current instruction
    pub stack: [u16; 16],
    pub stack_p: u8, // pointer to current stack
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    fn new() -> Self {
        Self {
            registeres: [0; 16],
            index_register: 0,
            program_counter: CHIP8_ROM_START,
            stack: [0; 16],
            stack_p: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
