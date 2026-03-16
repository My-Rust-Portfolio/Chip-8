#[cfg(test)]
mod tests {
    use crate::computer::display::*;

    #[test]
    fn test_display_init() {
        let display = Display::new();
        assert_eq!(display.display, [false; DISPLAY_SIZE]);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::new();
        display.display[100..200].fill(true);
        assert_ne!(display.display, [false; DISPLAY_SIZE]);
        display.clear();
        assert_eq!(display.display, [false; DISPLAY_SIZE]);
    }

    #[test]
    fn test_display_set_get_pixel() {
        let mut display = Display::new();
        display.set_pixel(100, true);
        assert_eq!(display.get_pixel(100), true);
    }
}
