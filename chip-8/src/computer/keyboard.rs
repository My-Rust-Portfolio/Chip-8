// historically 16 keys
const KEYBOARD_SIZE: usize = 16;

#[derive(Debug)]
pub struct Keyboard {
    keypad: [bool; KEYBOARD_SIZE],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keypad: [false; KEYBOARD_SIZE],
        }
    }

    pub fn is_pressed(&self, key_to_check: usize) -> bool {
        self.keypad[key_to_check]
    }
}
