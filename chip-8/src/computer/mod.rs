mod cpu;
mod display;
mod keyboard;
mod ram;
use cpu::{CHIP8_ROM_START, Cpu, SkipCondition};
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

    pub fn fetch(&mut self) -> u16 {
        // single instruction in Chip8 is 16 bits (2 bytes) long. Our ram is array of 1 bytes
        let byte1 = self.ram.memory[self.cpu.program_counter as usize] as u16;
        let byte2 = self.ram.memory[(self.cpu.program_counter + 1) as usize] as u16;

        self.cpu.skip_instruction();

        // merge bytes to produce final instruction
        (byte1 << 8) | byte2
    }

    pub fn tick(&mut self) {
        // 16bit long, represented as 4 hex digits
        let instruction = self.fetch();

        let (op1, op2, op3, op4, x, y, nn, nnn) = unwrap_instruction(instruction);

        match (op1, op2, op3, op4) {
            (0, 0, 0, 0) => {
                // empty instrunction
            }

            (0, 0, 0xE, 0) => {
                self.display.clear();
            }

            (0, 0, 0xE, 0xE) => {
                self.cpu.return_from_subroutine();
            }

            (1, _, _, _) => {
                self.cpu.jump(nnn);
            }

            (2, _, _, _) => {
                self.cpu.call_subroutine(nnn);
            }

            (3, _, _, _) => {
                self.cpu
                    .skip_instruction_if(SkipCondition::RegisterXEqualsNn(x, nn));
            }

            (4, _, _, _) => {
                self.cpu
                    .skip_instruction_if(SkipCondition::RegisterXNotEqualsNn(x, nn));
            }

            (5, _, _, _) => {
                self.cpu
                    .skip_instruction_if(SkipCondition::RegisterXEqualsRegisterY(x, y));
            }

            (6, _, _, _) => {
                self.cpu.set_register_x_to_nn(x, nn);
            }

            (7, _, _, _) => {
                self.cpu.add_nn_to_register_x(x, nn);
            }

            (9, _, _, _) => {
                self.cpu
                    .skip_instruction_if(SkipCondition::RegisterXNotEqualsRegisterY(x, y));
            }

            (0xA, _, _, _) => {
                self.cpu.set_index_register(nnn);
            }

            (_, _, _, _) => {
                eprintln!("Unknown instruction: {instruction:#06X}");
            }
        }
    }
}

// ================= private helpers =================
// 16bit instructions need to be unwrapped to be executed
fn unwrap_instruction(instruction: u16) -> (u16, u16, u16, u16, usize, usize, u8, u16) {
    // extract the 4 hex digits
    let op1 = (instruction & 0xF000) >> 12; // category digit
    let op2 = (instruction & 0x0F00) >> 8; // register x
    let op3 = (instruction & 0x00F0) >> 4; // register y
    let op4 = instruction & 0x000F; // 4 bit number

    // depending on the category we will decide which values are necessary
    // to complete the instrucction
    let y = op3 as usize;
    let x = op2 as usize;
    let nn = (instruction & 0x00FF) as u8;
    let nnn = instruction & 0x0FFF;

    (op1, op2, op3, op4, x, y, nn, nnn)
}
