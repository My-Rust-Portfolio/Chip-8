use eframe::egui;

// Font data starts at byte 80 (0x50)
pub const CHIP8_RAM_FONTDATA_START: usize = 0x50;

const FONTSET_SIZE: usize = 80;
const RAM_SIZE: usize = 4096;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Ram {
    memory: [u8; RAM_SIZE], // 4kb (4096 byte) of RAM
}

impl Ram {
    pub fn new() -> Self {
        let mut m = [0; RAM_SIZE];
        m[CHIP8_RAM_FONTDATA_START..CHIP8_RAM_FONTDATA_START + FONTSET_SIZE]
            .copy_from_slice(&FONTSET);

        Self { memory: m }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_slice(&mut self, start_address: u16, data: &[u8]) {
        let start = start_address as usize;
        let end = start + data.len();
        self.memory[start..end].copy_from_slice(data);
    }

    pub fn read_slice(&self, start_address: u16, length: usize) -> &[u8] {
        &self.memory[start_address as usize..length]
    }

    pub fn draw_ui(&self, ui: &mut egui::Ui, current_program_counter: u16) {
        ui.heading("RAM");
        ui.separator();

        const TOTAL_ROWS: usize = 4096 / 8;

        // 2. Calculate the exact height of one row of text so egui knows how big the scrollbar should be
        let text_height = ui.text_style_height(&egui::TextStyle::Monospace);
        let row_height = text_height + ui.spacing().item_spacing.y;

        egui::ScrollArea::vertical()
            .id_salt("ram_scroll")
            .max_height(300.0)
            .auto_shrink([false; 2])
            // only iterate over the visible rows
            .show_rows(ui, row_height, TOTAL_ROWS, |ui, row_range| {
                for row_index in row_range {
                    let base_address = row_index * 8;
                    let chunk = &self.memory[base_address..(base_address + 8)];

                    ui.horizontal(|ui| {
                        // address on the left
                        ui.label(
                            egui::RichText::new(format!("0x{:04X}:", base_address))
                                .family(egui::FontFamily::Monospace)
                                .color(egui::Color32::DARK_GRAY),
                        );

                        // 8 bytes on the right
                        for (byte_index, byte) in chunk.iter().enumerate() {
                            let absolute_address = (base_address + byte_index) as u16;

                            let is_pc = absolute_address == current_program_counter
                                || absolute_address == current_program_counter + 1;

                            let text = egui::RichText::new(format!("{:02X}", byte))
                                .family(egui::FontFamily::Monospace);

                            if is_pc {
                                ui.label(
                                    text.color(egui::Color32::BLACK)
                                        .background_color(egui::Color32::YELLOW),
                                );
                            } else if *byte == 0 {
                                ui.label(text.color(egui::Color32::DARK_GRAY));
                            } else {
                                ui.label(text);
                            }
                        }
                    });
                }
            });
    }
}

#[cfg(test)]
#[path = "ram_tests.rs"]
mod tests;
