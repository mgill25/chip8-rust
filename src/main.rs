use std::env;
use std::fs::File;

mod core;

fn main() {
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let mut vm = core::Machine::new("Chip8");
    vm.load_rom(&rom_file);
}
