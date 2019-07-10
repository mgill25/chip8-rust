use std::env;

mod core;

fn main() {
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let vm = core::Machine::new("Chip8", &rom_file);
}
