use std::fs::File;
use std::io::Read;
use std::fmt;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTER_COUNT: usize = 16;
const PROGRAM_OFFSET: usize = 512;

struct Memory {
    mem: [u8; MEMORY_SIZE],
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const ZERO: u8 = 0;
        write!(f, "{{ ");
        for (index, byte) in self.mem.iter().enumerate() {
            if *byte != ZERO {
                write!(f, "{}: {}, ", index, byte);
            }
        };
        write!(f, "}}")
    }
}

pub struct Machine {
    name: String,
    counter: u16,
    stack_ptr: u8,
    mem: Memory,
    stack: [u16; STACK_SIZE],
    v: [u8; REGISTER_COUNT],        // registers: v0 to vf
    i: u16,                         // "There is also a 16-bit register called I."
    delay_register: u8,
    sound_register: u8,
}

impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {{ \n\tPC: {}, \n\tSP: {}, \n\tStack: {:?}, \n\tRegisters: {:?}, \n\ti: {}, \n\tDR: {}, \n\tSR: {}, \n\tMem: {:?} }}", self.name, self.counter, self.stack_ptr, self.stack, self.v, self.i, self.delay_register, self.sound_register, self.mem)
    }
}

impl Machine {
    pub fn new(name: &str, rom_file: &mut File) -> Self {
        let mut m = Machine {
            name: name.to_string(),
            counter: 0,
            stack_ptr: 0,
            mem: Memory {mem: [0; MEMORY_SIZE]},
            stack: [0; STACK_SIZE],
            v: [0; REGISTER_COUNT],
            i: 0,
            delay_register: 0,
            sound_register: 0,
        };
        m.load_rom(rom_file).expect("Error loading ROM file");
        m
    }

    fn load_rom(&mut self, file: &mut File) -> Result<(), String> {
        const BUFSIZE: usize = MEMORY_SIZE - PROGRAM_OFFSET;
        let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];

        // load the ROM into the buffer
        let _ = file.read(&mut buffer).expect("Error reading from File");

        // Copy the buffer into the VM memory
        // TODO: Why not copy directly without the intermediate buffer
        self.mem.mem[PROGRAM_OFFSET..].clone_from_slice(&buffer);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_into_mem() {
        let mut tmpfile = tempfile::tempfile().unwrap();
        let vm = Machine::new("TestVM", &mut tmpfile);
        assert_eq!(1 + 1, 2);
    }
}
