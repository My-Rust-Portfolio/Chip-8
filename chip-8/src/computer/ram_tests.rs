#[cfg(test)]
mod tests {
    use crate::computer::ram::*;

    #[test]
    fn test_ram_init() {
        let ram = Ram::new();
        let mut m = [0; RAM_SIZE];
        m[CHIP8_RAM_FONTDATA_START..CHIP8_RAM_FONTDATA_START + FONTSET_SIZE]
            .copy_from_slice(&FONTSET);

        assert_eq!(ram.memory, m);
    }

    #[test]
    fn test_ram_read_write_byte() {
        let mut ram = Ram::new();
        ram.write_byte(100, 100);
        assert_eq!(ram.read_byte(100), 100);
    }

    #[test]
    fn test_ram_read_write_slice() {
        let mut ram = Ram::new();
        let data = [0, 1, 2];
        ram.write_slice(0, &data);
        assert_eq!(ram.read_slice(0, data.len()), data);
    }
}
