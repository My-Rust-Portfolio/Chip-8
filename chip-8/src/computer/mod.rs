mod cpu;
mod display;
mod keyboard;
mod ram;
use cpu::{CHIP8_ROM_START, Cpu, SkipCondition};
use display::Display;
use keyboard::Keyboard;
use ram::{CHIP8_RAM_FONTDATA_START, Ram};
use std::fs;

use crate::computer::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    ram: Ram,
    display: Display,
    keyboard: Keyboard,
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
        self.ram.write_slice(CHIP8_ROM_START, &rom_data);
    }

    pub fn fetch(&mut self) -> u16 {
        // single instruction in Chip8 is 16 bits (2 bytes) long. Our ram is array of 1 bytes
        let program_counter = self.cpu.get_program_counter();
        let byte1 = self.ram.read_byte(program_counter) as u16;
        let byte2 = self.ram.read_byte(program_counter + 1) as u16;

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

            (8, _, _, _) => match op4 {
                0 => self.cpu.set_register_x_to_y(x, y),
                1 => self.cpu.bitwise_or(x, y),
                2 => self.cpu.bitwise_and(x, y),
                3 => self.cpu.bitwise_xor(x, y),
                4 => self.cpu.add_y_to_x_with_carry(x, y),
                5 => self.cpu.sub_y_from_x(x, y),
                6 => self.cpu.shift_right(x),
                7 => self.cpu.sub_x_from_y(x, y),
                0xE => self.cpu.shift_left(x),
                _ => eprintln!("Unknown 8-series instruction: {instruction:#06X}"),
            },

            (9, _, _, _) => {
                self.cpu
                    .skip_instruction_if(SkipCondition::RegisterXNotEqualsRegisterY(x, y));
            }

            (0xA, _, _, _) => {
                self.cpu.set_index_register(nnn);
            }

            (0xD, _, _, _) => {
                let sprite_start_x = self.cpu.get_register_x(x) as usize % DISPLAY_WIDTH;
                let sprite_start_y = self.cpu.get_register_x(y) as usize % DISPLAY_HEIGHT;

                // prepare for collision detection, 0xF used to detect collisions
                self.cpu.set_register_x_to_nn(0xF, 0);
                let sprite_height = op4 as usize;

                for row in 0..sprite_height {
                    let current_y = sprite_start_y + row;

                    // stop drawing if out of screen bottom
                    if current_y >= DISPLAY_HEIGHT {
                        break;
                    }

                    // read the sprite from RAM
                    let i = self.cpu.get_index_register();
                    let sprite_byte = self.ram.read_byte(i + row as u16);

                    // loop over all 8 bits in the sprite byte
                    for col in 0..8 {
                        // stop drawing if out of the screen right
                        let current_x = sprite_start_x + col;
                        if current_x >= DISPLAY_WIDTH {
                            break;
                        }

                        // 0x80 is 10000000, check if bit at col is set
                        let sprite_pixel_is_set = (sprite_byte & (0x80 >> col)) != 0;

                        if sprite_pixel_is_set {
                            let pixel_index = current_y * DISPLAY_WIDTH + current_x;

                            // if the screen pixel is already true, its collision
                            if self.display.get_pixel(pixel_index) {
                                self.cpu.set_register_x_to_nn(0xF, 1);
                            }

                            // XOR the pixel to the screen
                            let val = self.display.get_pixel(pixel_index) ^ true;
                            self.display.set_pixel(pixel_index, val);
                        }
                    }
                }
            }

            (0xE, _, _, _) => {
                let key_to_check = self.cpu.get_register_x(x) as u16;
                let is_pressed = self.keyboard.is_pressed(key_to_check);

                match nn {
                    0x9E => {
                        if is_pressed {
                            self.cpu.skip_instruction();
                        }
                    }
                    0xA1 => {
                        if !is_pressed {
                            self.cpu.skip_instruction();
                        }
                    }
                    _ => {
                        eprintln!("Unknown E-series instruction: {instruction:#06X}");
                    }
                }
            }

            (0xF, _, _, _) => match nn {
                0x0A => {
                    let mut key_pressed = false;
                    for i in 0..16 {
                        if self.keyboard.is_pressed(i) {
                            // We found a pressed key! Store its index in Vx.
                            self.cpu.set_register_x_to_nn(x, i as u8);
                            key_pressed = true;
                            break;
                        }
                    }

                    if !key_pressed {
                        let current_pc = self.cpu.get_program_counter();
                        self.cpu.jump(current_pc - 2); // loop the same instruction until a key is pressed  
                    }
                }

                0x07 => {
                    self.cpu.set_register_x_to_delay_timer(x);
                }

                0x15 => {
                    self.cpu.set_delay_timer_to_register_x(x);
                }

                0x18 => {
                    self.cpu.set_sound_timer_to_register_x(x);
                }

                0x1E => {
                    self.cpu.add_register_x_to_index_register(x);
                }

                0x29 => {
                    let char_val = self.cpu.get_register_x(x) as u16;
                    const FONT_SIZE: u16 = 5;
                    let font_address = CHIP8_RAM_FONTDATA_START as u16 + (char_val * FONT_SIZE);
                    self.cpu.set_index_register(font_address);
                }

                0x33 => {
                    let reg = self.cpu.get_register_x(x);
                    let i_reg = self.cpu.get_index_register();

                    self.ram.write_byte(i_reg, reg / 100); // hundreds
                    self.ram.write_byte(i_reg + 1, (reg / 10) % 10); // tens
                    self.ram.write_byte(i_reg + 2, reg % 10); // ones
                }

                0x55 => {
                    let i_reg = self.cpu.get_index_register();
                    for i in 0..=x {
                        let val = self.cpu.get_register_x(i);
                        self.ram.write_byte(i_reg + i as u16, val);
                    }
                }

                0x65 => {
                    let i_reg = self.cpu.get_index_register();
                    for i in 0..=x {
                        let val = self.ram.read_byte(i_reg + i as u16);
                        self.cpu.set_register_x_to_nn(i, val);
                    }
                }

                _ => {
                    eprintln!("Unknown F-series instruction: {instruction:#06X}");
                }
            },

            (_, _, _, _) => {
                eprintln!("Unknown instruction: {instruction:#06X}");
            }
        }
    }

    pub fn update_delay_timer(&mut self) {
        self.cpu.update_delay_timer();
    }

    pub fn update_sound_timer(&mut self) {
        self.cpu.update_sound_timer();
    }

    pub fn get_cpu_program_counter(&self) -> u16 {
        self.cpu.get_program_counter()
    }

    pub fn reset_keyboard(&mut self) {
        self.keyboard.reset();
    }

    pub fn key_pressed(&mut self, key: u16) {
        self.keyboard.press(key);
    }

    pub fn is_pixel_set(&self, index: usize) -> bool {
        self.display.get_pixel(index)
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
