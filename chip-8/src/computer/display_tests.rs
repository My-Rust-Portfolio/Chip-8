#[cfg(test)]
mod tests {
    use crate::computer::display::{self, *};

    #[test]
    fn test_display_init() {
        let display = Display::new();
        assert_eq!(display.display, [false; DISPLAY_SIZE]);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::new();
        display.display[100] = true;
        display.display[101] = true;
        display.display[102] = true;
        display.display[103] = true;
        assert_ne!(display.display, [false; DISPLAY_SIZE]);
        display.clear();
        assert_eq!(display.display, [false; DISPLAY_SIZE]);
    }
}
