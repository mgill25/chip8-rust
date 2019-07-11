use std::env;
use std::fs::File;

mod core;

fn main() {
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let mut file = File::open(rom_file).expect("ROM not found");
    let vm = core::Machine::new("Chip8", &mut file);
    println!("{:#?}", vm);
}
