#[cfg(test)]
mod tests {
    use crate::computer::keyboard::*;

    #[test]
    fn test_keyboard_init() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.keypad, 0);
    }

    #[test]
    fn test_keyboard_press_key() {
        let mut keyboard = Keyboard::new();
        keyboard.press(5);
        assert_eq!(keyboard.is_pressed(5), true);
    }

    #[test]
    fn test_keyboard_release_key() {
        let mut keyboard = Keyboard::new();
        keyboard.press(5);
        assert_eq!(keyboard.is_pressed(5), true);
        keyboard.release(5);
        assert_eq!(keyboard.is_pressed(5), false);
    }
}
