use std::convert::{Into, TryFrom};

type Address = u16;
type Register = u8;
type Data = u8;

#[derive(Debug)]
pub enum InstructionV2 {
    ClearScreen,                              // 00E0 - CLS
    Return,                                   // 00EE - RET
    SYS,                                      // 0nnn - SYS addr
    Jump(Address),                            // 1nnn - JP addr
    Call(Address),                            // 2nnn - CALL addr
    SkipEqualsByte(Register, Data),           // 3xkk - SE vx, byte
    SkipNotEqualsByte(Register, Data),        // 4xkk - SNE Vx, byte
    SkipEqualsRegister(Register, Register),   // 5xy0 - SE Vx, Vy
    LoadByte(Register, Data),                 // 6xkk - LD Vx, byte
    AddByte(Register, Data),                  // 7xkk - ADD Vx, byte
    LoadRegister(Register, Register),         // 8xy0 - LD Vx, Vy
    Or(Register, Register),                   // 8xy1 - OR Vx, Vy
    And(Register, Register),                  // 8xy2 - AND Vx, Vy
    Xor(Register, Register),                  // 8xy3 - XOR Vx, Vy
    AddRegister(Register, Register),          // 8xy4 - ADD Vx, Vy
    SubRegister(Register, Register),          // 8xy5 - SUB Vx, Vy
    ShiftRight(Register),                     // 8xy6 - SHR Vx
    SubNRegister(Register, Register),         // 8xy7 - SUBN Vx, Vy
    ShiftLeft(Register),                      // 8xyE - SHL Vx
    SkipNotEqualRegister(Register, Register), // 9xy0 - SNE Vx, Vy
    LoadImmediate(Address),                   // Annn - LD I, addr
    JumpBase(Address),                        // Bnnn - JP V0, address
    Random(Register, Data),                   // Cxkk - RND Vx, byte
    DisplaySprite(Register, Register, u8),    // Dxyn - DRW Vx, Vy, nibble
    SkipKeyPress(Register),                   // Ex9E - SKP Vx
    SkipNotKeyPress(Register),                // ExA1 - SKNP Vx
    LoadFromDelay(Register),                  // Fx07 - LD Vx, DT
    LoadKeyPress(Register),                   // Fx0A - LD Vx, K
    LoadDelay(Register),                      // Fx15 - LD DT, Vx
    LoadSound(Register),                      // Fx18 - LD ST, Vx
    AddI(Register),                           // Fx1E - ADD I, Vx
    LoadFontSprite(Register),                 // Fx29 - LD F, Vx
    LoadIBCD(Register),                       // Fx33 - LD B, Vx
    StoreRegisters(Register),                 // Fx55 - LD [I], Vx
    LoadRegisters(Register),                  // Fx65 - LD Vx, [I]
}

pub struct OpcodeTableEntry {
    opcode: u16,
    mask: u16,
}

fn initialize_opcode_table() -> [OpcodeTableEntry; 35] {
    const OPCODE_TABLE: [OpcodeTableEntry; 35] = [
        OpcodeTableEntry {
            opcode: 0x0000,
            mask: 0xF000,
        }, // 0x0NNN
        OpcodeTableEntry {
            opcode: 0x00E0,
            mask: 0xFFFF,
        }, // 0x00E0
        OpcodeTableEntry {
            opcode: 0x00EE,
            mask: 0xFFFF,
        }, // 0x00EE
        OpcodeTableEntry {
            opcode: 0x1000,
            mask: 0xF000,
        }, // 0x1NNN
        OpcodeTableEntry {
            opcode: 0x2000,
            mask: 0xF000,
        }, // 0x2NNN
        OpcodeTableEntry {
            opcode: 0x3000,
            mask: 0xF000,
        }, // 0x3XNN
        OpcodeTableEntry {
            opcode: 0x4000,
            mask: 0xF000,
        }, // 0x4XNN
        OpcodeTableEntry {
            opcode: 0x5000,
            mask: 0xF00F,
        }, // 0x5XY0
        OpcodeTableEntry {
            opcode: 0x6000,
            mask: 0xF000,
        }, // 0x6XNN
        OpcodeTableEntry {
            opcode: 0x7000,
            mask: 0xF000,
        }, // 0x7XNN
        OpcodeTableEntry {
            opcode: 0x8000,
            mask: 0xF00F,
        }, // 0x8XY0
        OpcodeTableEntry {
            opcode: 0x8001,
            mask: 0xF00F,
        }, // 0x8XY1
        OpcodeTableEntry {
            opcode: 0x8002,
            mask: 0xF00F,
        }, // 0x8XY2
        OpcodeTableEntry {
            opcode: 0x8003,
            mask: 0xF00F,
        }, // 0x8XY3
        OpcodeTableEntry {
            opcode: 0x8004,
            mask: 0xF00F,
        }, // 0x8XY4
        OpcodeTableEntry {
            opcode: 0x8005,
            mask: 0xF00F,
        }, // 0x8XY5
        OpcodeTableEntry {
            opcode: 0x8006,
            mask: 0xF00F,
        }, // 0x8XY6
        OpcodeTableEntry {
            opcode: 0x8007,
            mask: 0xF00F,
        }, // 0x8XY7
        OpcodeTableEntry {
            opcode: 0x800E,
            mask: 0xF00F,
        }, // 0x8XYE
        OpcodeTableEntry {
            opcode: 0x9000,
            mask: 0xF00F,
        }, // 0x9XY0
        OpcodeTableEntry {
            opcode: 0xA000,
            mask: 0xF000,
        }, // 0xANNN
        OpcodeTableEntry {
            opcode: 0xB000,
            mask: 0xF000,
        }, // 0xBNNN
        OpcodeTableEntry {
            opcode: 0xC000,
            mask: 0xF000,
        }, // 0xCXNN
        OpcodeTableEntry {
            opcode: 0xD000,
            mask: 0xF000,
        }, // 0xDXYN
        OpcodeTableEntry {
            opcode: 0xE09E,
            mask: 0xF0FF,
        }, // 0xEX9E
        OpcodeTableEntry {
            opcode: 0xE001,
            mask: 0xF0FF,
        }, // 0xEXA1
        OpcodeTableEntry {
            opcode: 0xF007,
            mask: 0xF0FF,
        }, // 0xFX07
        OpcodeTableEntry {
            opcode: 0xF00A,
            mask: 0xF0FF,
        }, // 0xFX0A
        OpcodeTableEntry {
            opcode: 0xF015,
            mask: 0xF0FF,
        }, // 0xFX15
        OpcodeTableEntry {
            opcode: 0xF018,
            mask: 0xF0FF,
        }, // 0xFX18
        OpcodeTableEntry {
            opcode: 0xF01E,
            mask: 0xF0FF,
        }, // 0xFX1E
        OpcodeTableEntry {
            opcode: 0xF029,
            mask: 0xF0FF,
        }, // 0xFX29
        OpcodeTableEntry {
            opcode: 0xF033,
            mask: 0xF0FF,
        }, // 0xFX33
        OpcodeTableEntry {
            opcode: 0xF055,
            mask: 0xF0FF,
        }, // 0xFX55
        OpcodeTableEntry {
            opcode: 0xF065,
            mask: 0xF0FF,
        }, // 0xFX65
    ];
    OPCODE_TABLE
}

impl TryFrom<u16> for InstructionV2 {
    type Error = String;
    fn try_from(opcode: u16) -> Result<Self, String> {
        let opcode_table = initialize_opcode_table();
        for opcode_entry in opcode_table.iter() {
            if opcode != 0 && (opcode & opcode_entry.mask == opcode_entry.opcode) {
                trace!("Got a valid opcode: {:X}", opcode);
            }
        }
        Ok(InstructionV2::SYS) // just to satisfy the type checker for now
    }
}
