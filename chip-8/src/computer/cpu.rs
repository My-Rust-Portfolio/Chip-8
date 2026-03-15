// First 512bytes (0x200) are for system, hence CHIP-8 games always start from 0x200 address
pub const CHIP8_ROM_START: u16 = 0x200;

#[derive(Debug)]
pub struct Cpu {
    pub registeres: [u8; 16], // V0 to VF
    pub index_register: u16,  // points to memory/RAM location
    pub program_counter: u16, // pointer to current instruction
    pub stack: [u16; 16],
    pub stack_p: u8, // pointer to current stack + 1
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

    pub fn skip_instruction(&mut self) {
        // 1 instruction is 16 bit, our ram is 8bit per cell
        self.program_counter += 2;
    }

    pub fn jump(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    pub fn return_from_subroutine(&mut self) {
        self.program_counter = self.pop_from_stack();
    }

    pub fn call_subroutine(&mut self, nnn: u16) {
        self.push_into_stack(self.program_counter);
        self.program_counter = nnn;
    }

    pub fn skip_if_register_x_equals_nn(&mut self, x: usize, nn: u8) {
        if self.registeres[x] == nn {
            self.skip_instruction();
        }
    }

    pub fn skip_if_register_x_not_equals_nn(&mut self, x: usize, nn: u8) {
        if self.registeres[x] != nn {
            self.skip_instruction();
        }
    }

    pub fn skip_if_register_x_equals_register_y(&mut self, x: usize, y: usize) {
        if self.registeres[x] == self.registeres[y] {
            self.skip_instruction();
        }
    }

    pub fn skip_if_register_x_not_equals_register_y(&mut self, x: usize, y: usize) {
        if self.registeres[x] != self.registeres[y] {
            self.skip_instruction();
        }
    }

    pub fn set_register_x_to_nn(&mut self, x: usize, nn: u8) {
        self.registeres[x] = nn;
    }

    pub fn add_nn_to_register_x(&mut self, x: usize, nn: u8) {
        self.registeres[x] = self.registeres[x].wrapping_add(nn);
    }

    pub fn set_index_register(&mut self, nnn: u16) {
        self.index_register = nnn;
    }
}

// ============ private helpers ============
impl Cpu {
    fn push_into_stack(&mut self, val: u16) {
        self.stack[self.stack_p as usize] = val;
        self.stack_p += 1;
    }

    fn pop_from_stack(&mut self) -> u16 {
        self.stack_p -= 1;
        self.stack[self.stack_p as usize]
    }
}
