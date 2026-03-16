// First 512bytes (0x200) are for system, hence CHIP-8 games always start from 0x200 address
pub const CHIP8_ROM_START: u16 = 0x200;

const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

pub enum SkipCondition {
    RegisterXEqualsNn(usize, u8),
    RegisterXNotEqualsNn(usize, u8),
    RegisterXEqualsRegisterY(usize, usize),
    RegisterXNotEqualsRegisterY(usize, usize),
}

#[derive(Debug)]
pub struct Cpu {
    registers: [u8; REGISTER_SIZE], // V0 to VF
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

    pub fn add_nn_to_register_x(&mut self, x: usize, nn: u8) {
        self.registers[x] = self.registers[x].wrapping_add(nn);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = Cpu::new();
        assert_eq!(cpu.registers, [0; REGISTER_SIZE]);
        assert_eq!(cpu.index_register, 0);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START);
        assert_eq!(cpu.stack, [0; STACK_SIZE]);
        assert_eq!(cpu.stack_p, 0);
        assert_eq!(cpu.delay_timer, 0);
        assert_eq!(cpu.sound_timer, 0);
    }

    #[test]
    fn test_cpu_skip_instruction() {
        let mut cpu = Cpu::new();
        cpu.skip_instruction();
        assert_eq!(cpu.get_program_counter(), CHIP8_ROM_START + 2);
        cpu.skip_instruction();
        assert_eq!(cpu.get_program_counter(), CHIP8_ROM_START + 4);
    }

    #[test]
    fn test_cpu_jump() {
        let mut cpu = Cpu::new();
        const JUMP_TO: u16 = CHIP8_ROM_START + 50;
        cpu.jump(JUMP_TO);
        assert_eq!(cpu.get_program_counter(), JUMP_TO);
    }

    #[test]
    fn test_cpu_subroutine() {
        let mut cpu = Cpu::new();
        const CURRENT_ROUTINE: u16 = CHIP8_ROM_START + 32;
        const SUBROUTINE: u16 = CHIP8_ROM_START + 64;
        cpu.jump(CURRENT_ROUTINE);
        cpu.call_subroutine(SUBROUTINE);
        assert_eq!(cpu.stack[0], CURRENT_ROUTINE);
        assert_eq!(cpu.get_program_counter(), SUBROUTINE);
        cpu.return_from_subroutine();
        assert_eq!(cpu.get_program_counter(), CURRENT_ROUTINE);
    }

    #[test]
    fn test_cpu_skip_if_() {
        let mut cpu = Cpu::new();
        let cond = SkipCondition::RegisterXEqualsNn(5, 6);
        cpu.registers[5] = 6;
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);
        cpu.registers[5] = 7;
        let cond = SkipCondition::RegisterXEqualsNn(5, 6);
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);

        let mut cpu = Cpu::new();
        let cond = SkipCondition::RegisterXNotEqualsNn(5, 6);
        cpu.registers[5] = 6;
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START);
        cpu.registers[5] = 7;
        let cond = SkipCondition::RegisterXNotEqualsNn(5, 6);
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);

        let mut cpu = Cpu::new();
        let cond = SkipCondition::RegisterXEqualsRegisterY(5, 6);
        cpu.registers[5] = 6;
        cpu.registers[6] = 6;
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);
        cpu.registers[5] = 7;
        let cond = SkipCondition::RegisterXEqualsRegisterY(5, 6);
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);

        let mut cpu = Cpu::new();
        let cond = SkipCondition::RegisterXNotEqualsRegisterY(5, 6);
        cpu.registers[5] = 6;
        cpu.registers[6] = 6;
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START);
        cpu.registers[5] = 7;
        let cond = SkipCondition::RegisterXNotEqualsRegisterY(5, 6);
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);
    }

    #[test]
    fn test_cpu_set_register() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(5, 6);
        assert_eq!(cpu.registers[5], 6);
    }

    #[test]
    fn test_cpu_add_to_register() {
        let mut cpu = Cpu::new();
        cpu.add_nn_to_register_x(0, 10);
        assert_eq!(cpu.registers[0], 10);
        cpu.add_nn_to_register_x(0, 245);
        assert_eq!(cpu.registers[0], 255);
        cpu.add_nn_to_register_x(0, 1);
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_cpu_set_index_to_register() {
        let mut cpu = Cpu::new();
        cpu.set_index_register(100);
        assert_eq!(cpu.index_register, 100);
    }
}
