#[cfg(test)]
mod tests {
    use crate::computer::cpu::*;

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
