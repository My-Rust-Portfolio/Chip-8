#[cfg(test)]
mod tests {
    use crate::computer::cpu::*;

    #[test]
    fn test_cpu_init() {
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
        assert_eq!(cpu.stack[(cpu.stack_p - 1) as usize], CURRENT_ROUTINE);
        assert_eq!(cpu.get_program_counter(), SUBROUTINE);
        cpu.return_from_subroutine();
        assert_eq!(cpu.get_program_counter(), CURRENT_ROUTINE);
    }

    #[test]
    fn test_cpu_skip_if_register_x_equals_nn() {
        let mut cpu = Cpu::new();
        let cond = SkipCondition::RegisterXEqualsNn(5, 6);
        cpu.registers[5] = 6;
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);
        cpu.registers[5] = 7;
        let cond = SkipCondition::RegisterXEqualsNn(5, 6);
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);
    }

    #[test]
    fn test_cpu_skip_if_register_x_not_equals_nn() {
        let mut cpu = Cpu::new();
        let cond = SkipCondition::RegisterXNotEqualsNn(5, 6);
        cpu.registers[5] = 6;
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START);
        cpu.registers[5] = 7;
        let cond = SkipCondition::RegisterXNotEqualsNn(5, 6);
        cpu.skip_instruction_if(cond);
        assert_eq!(cpu.program_counter, CHIP8_ROM_START + 2);
    }

    #[test]
    fn test_cpu_skip_if_register_x_equals_register_y() {
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
    }

    #[test]
    fn test_cpu_skip_if_register_x_not_equals_register_y() {
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
    fn test_cpu_set_get_register() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(5, 6);
        assert_eq!(cpu.get_register_x(5), 6);
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
    fn test_cpu_set_get_index_register() {
        let mut cpu = Cpu::new();
        cpu.set_index_register(100);
        assert_eq!(cpu.get_index_register(), 100);
    }

    #[test]
    fn test_cpu_bitwise_or() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 1); // 001
        cpu.set_register_x_to_nn(1, 2); // 010
        cpu.bitwise_or(0, 1); // 011 = 3
        assert_eq!(cpu.registers[0], 3);
    }

    #[test]
    fn test_cpu_bitwise_and() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 1); // 001
        cpu.set_register_x_to_nn(1, 2); // 010
        cpu.bitwise_and(0, 1); // 000 = 0
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_cpu_bitwise_xor() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 10); // 1010
        cpu.set_register_x_to_nn(1, 6); // 0110
        cpu.bitwise_xor(0, 1); // 1100 = 12
        assert_eq!(cpu.registers[0], 12);
    }

    #[test]
    fn test_cpu_add_y_to_x_carry() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 250);
        cpu.set_register_x_to_nn(1, 5);
        cpu.add_y_to_x_with_carry(0, 1);
        assert_eq!(cpu.registers[0], 255);
        assert_eq!(cpu.registers[0x0F], 0);
        cpu.set_register_x_to_nn(1, 1);
        cpu.add_y_to_x_with_carry(0, 1);
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.registers[0x0F], 1);
    }

    #[test]
    fn test_cpu_sub_y_from_x() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 250);
        cpu.set_register_x_to_nn(1, 5);
        cpu.sub_y_from_x(0, 1);
        assert_eq!(cpu.registers[0], 245);
        assert_eq!(cpu.registers[0x0F], 1);
        cpu.set_register_x_to_nn(1, 246);
        cpu.sub_y_from_x(0, 1);
        assert_eq!(cpu.registers[0], 255);
        assert_eq!(cpu.registers[0x0F], 0);
    }

    #[test]
    fn test_cpu_shift_right() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 6);
        cpu.shift_right(0);
        assert_eq!(cpu.registers[0], 3);
        assert_eq!(cpu.registers[0xF], 0);
        cpu.shift_right(0);
        assert_eq!(cpu.registers[0], 1);
        assert_eq!(cpu.registers[0xF], 1);
        cpu.shift_right(0);
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.registers[0xF], 1);
        cpu.shift_right(0);
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn test_cpu_sub_x_from_y() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 5);
        cpu.set_register_x_to_nn(1, 250);
        cpu.sub_x_from_y(0, 1);
        assert_eq!(cpu.registers[0], 245);
        assert_eq!(cpu.registers[0xF], 1);
        cpu.set_register_x_to_nn(1, 244);
        cpu.sub_x_from_y(0, 1);
        assert_eq!(cpu.registers[0], 255);
        assert_eq!(cpu.registers[0xF], 0);
    }

    #[test]
    fn test_cpu_shift_left() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 100);
        cpu.shift_left(0);
        assert_eq!(cpu.registers[0], 200);
        assert_eq!(cpu.registers[0xF], 0);
        cpu.shift_left(0);
        assert_eq!(cpu.registers[0], 144);
        assert_eq!(cpu.registers[0xF], 1);
    }

    #[test]
    fn test_cpu_set_register_x_to_delay_timer() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 10;
        cpu.set_register_x_to_delay_timer(0);
        assert_eq!(cpu.registers[0], 10);
    }

    #[test]
    fn test_cpu_set_delay_timer_to_register() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 10);
        cpu.set_delay_timer_to_register_x(0);
        assert_eq!(cpu.delay_timer, 10);
    }

    #[test]
    fn test_cpu_set_sound_timer_to_register_x() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 10);
        cpu.set_sound_timer_to_register_x(0);
        assert_eq!(cpu.sound_timer, 10);
    }

    #[test]
    fn test_cpu_add_register_x_to_index_register() {
        let mut cpu = Cpu::new();
        cpu.set_register_x_to_nn(0, 10);
        cpu.set_index_register(10);
        cpu.add_register_x_to_index_register(0);
        assert_eq!(cpu.index_register, 20);
    }

    #[test]
    fn test_cpu_update_delay_timer() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 10;
        cpu.update_delay_timer();
        assert_eq!(cpu.delay_timer, 9);
    }

    #[test]
    fn test_cpu_update_sound_timer() {
        let mut cpu = Cpu::new();
        cpu.sound_timer = 10;
        cpu.update_sound_timer();
        assert_eq!(cpu.sound_timer, 9);
    }

    #[test]
    fn test_cpu_store_rpl_flags() {
        let mut cpu = Cpu::new();
        for i in 0..8 {
            cpu.registers[i] = i as u8;
        }
        cpu.store_rpl_flags(100);
        let vals: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(cpu.rpl_flags, vals);
    }

    #[test]
    fn test_cpu_load_rpl_flags() {
        let mut cpu = Cpu::new();
        for i in 0..8 {
            cpu.rpl_flags[i] = i as u8;
        }
        cpu.load_rpl_flags(100);
        let vals: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(cpu.registers[0..8], vals);
    }
}
