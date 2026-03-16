pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

#[derive(Debug)]
pub struct Display {
    display: [bool; DISPLAY_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Self {
            display: [false; DISPLAY_SIZE],
        }
    }

    pub fn clear(&mut self) {
        self.display = [false; DISPLAY_SIZE];
    }

    pub fn get_pixel(&self, index: usize) -> bool {
        self.display[index]
    }

    pub fn set_pixel(&mut self, index: usize, val: bool) {
        self.display[index] = val;
    }
}

#[cfg(test)]
#[path = "display_tests.rs"]
mod tests;
