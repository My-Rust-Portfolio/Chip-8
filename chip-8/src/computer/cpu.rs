// First 512bytes (0x200) are for system, hence CHIP-8 games always start from 0x200 address
pub const CHIP8_ROM_START: u16 = 0x200;
pub const REGISTER_SIZE: usize = 16;
pub const STACK_SIZE: usize = 16;

pub enum SkipCondition {
    RegisterXEqualsNn(usize, u8),
    RegisterXNotEqualsNn(usize, u8),
    RegisterXEqualsRegisterY(usize, usize),
    RegisterXNotEqualsRegisterY(usize, usize),
}

#[derive(Debug)]
pub struct Cpu {
    registers: [u8; REGISTER_SIZE], // V0 to VF, V0 to VE general use, VF for overflown bits from the last math operation
    index_register: u16,            // points to memory/RAM location
    program_counter: u16,           // pointer to current instruction
    stack: [u16; STACK_SIZE],
    stack_p: u8, // pointer to current stack + 1
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_SIZE],
            index_register: 0,
            program_counter: CHIP8_ROM_START,
            stack: [0; STACK_SIZE],
            stack_p: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
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

    pub fn skip_instruction_if(&mut self, condition: SkipCondition) {
        let skip = match condition {
            SkipCondition::RegisterXEqualsNn(x, nn) => self.registers[x] == nn,
            SkipCondition::RegisterXNotEqualsNn(x, nn) => self.registers[x] != nn,
            SkipCondition::RegisterXEqualsRegisterY(x, y) => self.registers[x] == self.registers[y],
            SkipCondition::RegisterXNotEqualsRegisterY(x, y) => {
                self.registers[x] != self.registers[y]
            }
        };

        if skip {
            self.skip_instruction();
        }
    }

    pub fn set_register_x_to_nn(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
    }

    pub fn get_register_x(&self, x: usize) -> u8 {
        self.registers[x]
    }

    pub fn add_nn_to_register_x(&mut self, x: usize, nn: u8) {
        self.registers[x] = self.registers[x].wrapping_add(nn);
    }

    pub fn set_index_register(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    pub fn get_index_register(&self) -> u16 {
        self.index_register
    }

    pub fn set_register_x_to_y(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }

    pub fn bitwise_or(&mut self, x: usize, y: usize) {
        self.registers[x] |= self.registers[y];
    }

    pub fn bitwise_and(&mut self, x: usize, y: usize) {
        self.registers[x] &= self.registers[y];
    }

    pub fn bitwise_xor(&mut self, x: usize, y: usize) {
        self.registers[x] ^= self.registers[y];
    }

    pub fn add_y_to_x_with_carry(&mut self, x: usize, y: usize) {
        let (result, carry) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = if carry { 1 } else { 0 };
    }

    pub fn sub_y_from_x(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = if borrow { 0 } else { 1 };
    }

    pub fn shift_right(&mut self, x: usize) {
        // div by 2
        let dropped_bit = self.registers[x] & 1;
        self.registers[x] >>= 1;
        self.registers[0xF] = dropped_bit;
    }

    pub fn sub_x_from_y(&mut self, x: usize, y: usize) {
        let (result, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
        self.registers[x] = result;
        self.registers[0xF] = if borrow { 0 } else { 1 };
    }

    pub fn shift_left(&mut self, x: usize) {
        // mult by 2
        // 8th bit 1000_0000 in binary, 0x80 in hex, move by 7 to get it
        let dropped_bit = (self.registers[x] & 0x80) >> 7;
        self.registers[x] <<= 1;
        self.registers[0xF] = dropped_bit;
    }

    pub fn set_register_x_to_delay_timer(&mut self, x: usize) {
        self.set_register_x_to_nn(x, self.delay_timer);
    }

    pub fn set_delay_timer_to_register_x(&mut self, x: usize) {
        self.delay_timer = self.registers[x];
    }

    pub fn set_sound_timer_to_register_x(&mut self, x: usize) {
        self.sound_timer = self.registers[x];
    }

    pub fn add_register_x_to_index_register(&mut self, x: usize) {
        self.set_index_register(self.registers[x] as u16 + self.index_register);
    }

    pub fn update_delay_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn update_sound_timer(&mut self) {
        if self.sound_timer > 0 {
            self.delay_timer -= 1;
        }
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

#[cfg(test)]
#[path = "cpu_tests.rs"]
mod tests;
