pub mod computer;
use crate::computer::Chip8;

fn main() {
    println!("Hello, world!");

    let a = Chip8::new();
    println!("{:?}", a.ram);
}
