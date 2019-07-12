#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
mod core;

fn main() {
    env_logger::init();
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let vm = core::Machine::new("Chip8", &rom_file);
    debug!("{:#?}", vm);
}
