#[derive(Debug)]
pub struct Keyboard {
    keypad: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keypad: [false; 16],
        }
    }
}
