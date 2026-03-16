#[derive(Debug)]
pub struct Keyboard {
    keypad: u16, // 16 bits, each representing a key, historically 16 keys
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keypad: 0 }
    }

    pub fn is_pressed(&self, key_to_check: u16) -> bool {
        self.keypad & (1 << key_to_check) != 0
    }

    pub fn press(&mut self, key_to_press: u16) {
        self.keypad |= 1 << key_to_press;
    }

    pub fn release(&mut self, key_to_release: u16) {
        self.keypad &= !(1 << key_to_release);
    }

    pub fn reset(&mut self) {
        self.keypad = 0;
    }
}

#[cfg(test)]
#[path = "keyboard_tests.rs"]
mod tests;
