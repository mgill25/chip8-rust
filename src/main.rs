struct Machine {
    name: String,
    counter: u16,
    stack_ptr: u8,
    mem: [u8; 4096],
    stack: [u16; 16],
    v: [u8; 16],        // registers: v0 to vf
    i: u16,             // "There is also a 16-bit register called I."
    delay_register: u8,
    sound_register: u8,
}

impl Machine {

    fn new(name: &str) -> Machine {
        let mem = [0; 4096];
        let stack = [0; 16];
        let registers = [0; 16];
        let chip8 = Machine {
            name: String::from("Chip8"),
            counter: 0,
            stack_ptr: 0,
            mem,
            stack,
            v: registers,
            i: 0,
            delay_register: 0,
            sound_register: 0
        };
        return chip8;
    }

}

fn main() {
    let mut vm = Machine::new("Chip8");
}
