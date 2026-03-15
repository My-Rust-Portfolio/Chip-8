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
}
