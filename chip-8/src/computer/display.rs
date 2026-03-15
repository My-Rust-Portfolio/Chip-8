#[derive(Debug)]
pub struct Display {
    pub display: [bool; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Self {
            display: [false; 64 * 32],
        }
    }
}
