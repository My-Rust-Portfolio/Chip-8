use super::Ram;

// First 512bytes (0x200) are for system, hence CHIP-8 games always start from 0x200 address
pub const CHIP8_ROM_START: u16 = 0x200;

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
    pub fn new() -> Self {
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

    pub fn fetch(&mut self, ram: &Ram) -> u16 {
        // single instruction in Chip8 is 16 bits (2 bytes) long. Our ram is array of 1 bytes
        let byte1 = ram.memory[self.program_counter as usize] as u16;
        let byte2 = ram.memory[(self.program_counter + 1) as usize] as u16;

        self.program_counter += 2;

        // merge bytes to produce final instruction
        (byte1 << 8) | byte2
    }
}
