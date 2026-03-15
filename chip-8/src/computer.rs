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
}

#[derive(Debug)]
pub struct Ram {
    pub memory: [u8; 4096], // 4kb (4096b) of RAM
}

impl Ram {
    fn new() -> Self {
        Self { memory: [0; 4096] }
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
            program_counter: 0x200, // CHIP-8 always starts from 0x200 address
            stack: [0; 16],
            stack_p: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
