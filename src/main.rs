#[macro_use] extern crate log;
extern crate env_logger;

use std::env;

mod core;


fn main() -> Result<(), String> {
    env_logger::init();
    let rom_file = env::args().nth(1).expect("Please input a ROM file");
    let mut vm = core::Machine::new("Chip8");
    vm.load_rom(&rom_file)?;
    debug!("{:#?}", vm);
    Ok(())
}
