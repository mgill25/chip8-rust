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
        let chip8 = Machine {
            name: name.to_string(),
            counter: 0,
            stack_ptr: 0,
            mem: [0; 4096],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            delay_register: 0,
            sound_register: 0,
        };
        return chip8;
    }

}

fn main() {
    let vm = Machine::new("Chip8");
}
