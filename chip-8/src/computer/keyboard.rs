#[derive(Debug)]
pub struct Keyboard {
    pub keypad: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keypad: [false; 16],
        }
    }
}
