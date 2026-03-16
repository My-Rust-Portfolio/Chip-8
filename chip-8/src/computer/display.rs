const DISPLAY_SIZE: usize = 64 * 32;

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
}

#[cfg(test)]
#[path = "display_tests.rs"]
mod tests;
