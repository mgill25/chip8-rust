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

fn main() {
    let mut mem: [u8; 4096] = [0; 4096];
    let mut stack: [u16; 16] = [0; 16];
    let mut registers: [u8; 16] = [0; 16];
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
    println!("Hello, {}", chip8.name);
}
