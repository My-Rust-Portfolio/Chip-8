pub mod computer;

use computer::Chip8;
use eframe::egui;
use std::time::{Duration, Instant};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Chip8 Emulator",
        options,
        Box::new(|cc| Ok(Box::new(EmulatorApp::new(cc)))),
    )
}

struct EmulatorApp {
    chip8: Chip8,
    last_timer_tick: Instant,
}

impl EmulatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut chip8 = Chip8::new();

        chip8.load_rom("roms/snake.ch8");

        Self {
            chip8,
            last_timer_tick: Instant::now(),
        }
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            self.chip8.reset_keyboard();

            // 1 2 3 4  ->  1 2 3 C
            if i.key_down(egui::Key::Num1) {
                self.chip8.key_pressed(0x1);
            }
            if i.key_down(egui::Key::Num2) {
                self.chip8.key_pressed(0x2);
            }
            if i.key_down(egui::Key::Num3) {
                self.chip8.key_pressed(0x3);
            }
            if i.key_down(egui::Key::Num4) {
                self.chip8.key_pressed(0xC);
            }

            // Q W E R  ->  4 5 6 D
            if i.key_down(egui::Key::Q) {
                self.chip8.key_pressed(0x4);
            }
            if i.key_down(egui::Key::W) {
                self.chip8.key_pressed(0x5);
            }
            if i.key_down(egui::Key::E) {
                self.chip8.key_pressed(0x6);
            }
            if i.key_down(egui::Key::R) {
                self.chip8.key_pressed(0xD);
            }

            // A S D F  ->  7 8 9 E
            if i.key_down(egui::Key::A) {
                self.chip8.key_pressed(0x7);
            }
            if i.key_down(egui::Key::S) {
                self.chip8.key_pressed(0x8);
            }
            if i.key_down(egui::Key::D) {
                self.chip8.key_pressed(0x9);
            }
            if i.key_down(egui::Key::F) {
                self.chip8.key_pressed(0xE);
            }

            // Z X C V  ->  A 0 B F
            if i.key_down(egui::Key::Z) {
                self.chip8.key_pressed(0xA);
            }
            if i.key_down(egui::Key::X) {
                self.chip8.key_pressed(0x0);
            }
            if i.key_down(egui::Key::C) {
                self.chip8.key_pressed(0xB);
            }
            if i.key_down(egui::Key::V) {
                self.chip8.key_pressed(0xF);
            }
        });

        // game speed
        for _ in 0..1 {
            self.chip8.tick();
        }

        // timers need to run 60 times per second
        // check if 16.6 milliseconds (1/60th of a second) have passed
        if self.last_timer_tick.elapsed() >= Duration::from_micros(16666) {
            self.chip8.update_delay_timer();
            self.chip8.update_sound_timer();

            // reset the clock for the next timer tick
            self.last_timer_tick = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chip-8 Emulator");
            // width * height * color channels
            let mut pixels = vec![0_u8; 64 * 32 * 4];

            for y in 0..32 {
                for x in 0..64 {
                    let chip8_index = y * 64 + x;

                    let color = if self.chip8.is_pixel_set(chip8_index) {
                        255
                    } else {
                        0
                    };

                    let rgba_index = chip8_index * 4;
                    pixels[rgba_index] = color;
                    pixels[rgba_index + 1] = color;
                    pixels[rgba_index + 2] = color;
                    pixels[rgba_index + 3] = 255;
                }
            }

            let image = egui::ColorImage::from_rgba_unmultiplied([64, 32], &pixels);

            // send the image to the GPU
            let texture = ctx.load_texture(
                "chip8_screen",
                image,
                egui::TextureOptions::NEAREST, // NEAREST keeps the pixels sharp and blocky
            );

            // scale it up to see on a modern monitor
            let image_widget =
                egui::Image::new(&texture).fit_to_exact_size(egui::vec2(640.0, 320.0));
            ui.add(image_widget);
        });

        ctx.request_repaint();
    }
}
