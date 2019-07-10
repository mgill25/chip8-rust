mod core;

fn main() {
    let mut vm = core::Machine::new("Chip8");
    vm.copy_rom();
    vm.dump_memory();
}
