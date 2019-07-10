use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTER_COUNT: usize = 16;
const PROGRAM_OFFSET: usize = 512;

pub struct Machine {
    name: String,
    counter: u16,
    stack_ptr: u8,
    mem: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    v: [u8; REGISTER_COUNT],        // registers: v0 to vf
    i: u16,                         // "There is also a 16-bit register called I."
    delay_register: u8,
    sound_register: u8,
}

impl Machine {
    pub fn new(name: &str, rom_file: &str) -> Self {
        let mut m = Machine {
            name: name.to_string(),
            counter: 0,
            stack_ptr: 0,
            mem: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            v: [0; REGISTER_COUNT],
            i: 0,
            delay_register: 0,
            sound_register: 0,
        };
        m.load_rom(rom_file).expect("Error loading ROM file");
        m
    }

    fn load_rom(&mut self, filename: &str) -> Result<(), String> {
        let mut file = File::open(filename).expect("ROM not found");

        const BUFSIZE: usize = MEMORY_SIZE - PROGRAM_OFFSET;
        let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];

        // load the ROM into the buffer
        let _ = file.read(&mut buffer).expect("Error reading from File");

        // Copy the buffer into the VM memory
        // TODO: Why not copy directly without the intermediate buffer
        self.mem[PROGRAM_OFFSET..].clone_from_slice(&buffer);
        Ok(())
    }
}
