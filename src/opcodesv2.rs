use crate::ophandlers;
use std::convert::{Into, TryFrom};

type Address = u16;
type Register = u8;
type Data = u8;

#[derive(Debug, PartialEq)]
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
    handler: fn(u16) -> InstructionV2,
}

const OPCODE_TABLE: [OpcodeTableEntry; 35] = [
    OpcodeTableEntry {
        opcode: 0x00E0,
        mask: 0xFFFF,
        handler: ophandlers::handle0x00E0,
    }, // 0x00E0
    OpcodeTableEntry {
        opcode: 0x00EE,
        mask: 0xFFFF,
        handler: ophandlers::handle0x00EE,
    }, // 0x00EE
    OpcodeTableEntry {
        opcode: 0x0000,
        mask: 0xF000,
        handler: ophandlers::handle0x0NNN,
    }, // 0x0NNN
    OpcodeTableEntry {
        opcode: 0x1000,
        mask: 0xF000,
        handler: ophandlers::handle0x1NNN,
    }, // 0x1NNN
    OpcodeTableEntry {
        opcode: 0x2000,
        mask: 0xF000,
        handler: ophandlers::handle0x2NNN,
    }, // 0x2NNN
    OpcodeTableEntry {
        opcode: 0x3000,
        mask: 0xF000,
        handler: ophandlers::handle0x3XNN,
    }, // 0x3XNN
    OpcodeTableEntry {
        opcode: 0x4000,
        mask: 0xF000,
        handler: ophandlers::handle0x4XNN,
    }, // 0x4XNN
    OpcodeTableEntry {
        opcode: 0x5000,
        mask: 0xF00F,
        handler: ophandlers::handle0x5XY0,
    }, // 0x5XY0
    OpcodeTableEntry {
        opcode: 0x6000,
        mask: 0xF000,
        handler: ophandlers::handle0x6XNN,
    }, // 0x6XNN
    OpcodeTableEntry {
        opcode: 0x7000,
        mask: 0xF000,
        handler: ophandlers::handle0x7XNN,
    }, // 0x7XNN
    OpcodeTableEntry {
        opcode: 0x8000,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY0,
    }, // 0x8XY0
    OpcodeTableEntry {
        opcode: 0x8001,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY1,
    }, // 0x8XY1
    OpcodeTableEntry {
        opcode: 0x8002,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY2,
    }, // 0x8XY2
    OpcodeTableEntry {
        opcode: 0x8003,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY3,
    }, // 0x8XY3
    OpcodeTableEntry {
        opcode: 0x8004,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY4,
    }, // 0x8XY4
    OpcodeTableEntry {
        opcode: 0x8005,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY5,
    }, // 0x8XY5
    OpcodeTableEntry {
        opcode: 0x8006,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY6,
    }, // 0x8XY6
    OpcodeTableEntry {
        opcode: 0x8007,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XY7,
    }, // 0x8XY7
    OpcodeTableEntry {
        opcode: 0x800E,
        mask: 0xF00F,
        handler: ophandlers::handle0x8XYE,
    }, // 0x8XYE
    OpcodeTableEntry {
        opcode: 0x9000,
        mask: 0xF00F,
        handler: ophandlers::handle0x9XY0,
    }, // 0x9XY0
    OpcodeTableEntry {
        opcode: 0xA000,
        mask: 0xF000,
        handler: ophandlers::handle0xANNN,
    }, // 0xANNN
    OpcodeTableEntry {
        opcode: 0xB000,
        mask: 0xF000,
        handler: ophandlers::handle0xBNNN,
    }, // 0xBNNN
    OpcodeTableEntry {
        opcode: 0xC000,
        mask: 0xF000,
        handler: ophandlers::handle0xCXNN,
    }, // 0xCXNN
    OpcodeTableEntry {
        opcode: 0xD000,
        mask: 0xF000,
        handler: ophandlers::handle0xDXYN,
    }, // 0xDXYN
    OpcodeTableEntry {
        opcode: 0xE09E,
        mask: 0xF0FF,
        handler: ophandlers::handle0xEX9E,
    }, // 0xEX9E
    OpcodeTableEntry {
        opcode: 0xE001,
        mask: 0xF00F,
        handler: ophandlers::handle0xEXA1,
    }, // 0xEXA1
    OpcodeTableEntry {
        opcode: 0xF007,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX07,
    }, // 0xFX07
    OpcodeTableEntry {
        opcode: 0xF00A,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX0A,
    }, // 0xFX0A
    OpcodeTableEntry {
        opcode: 0xF015,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX15,
    }, // 0xFX15
    OpcodeTableEntry {
        opcode: 0xF018,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX18,
    }, // 0xFX18
    OpcodeTableEntry {
        opcode: 0xF01E,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX1E,
    }, // 0xFX1E
    OpcodeTableEntry {
        opcode: 0xF029,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX29,
    }, // 0xFX29
    OpcodeTableEntry {
        opcode: 0xF033,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX33,
    }, // 0xFX33
    OpcodeTableEntry {
        opcode: 0xF055,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX55,
    }, // 0xFX55
    OpcodeTableEntry {
        opcode: 0xF065,
        mask: 0xF0FF,
        handler: ophandlers::handle0xFX65,
    }, // 0xFX65 */
];

impl TryFrom<u16> for InstructionV2 {
    type Error = String;
    fn try_from(opcode: u16) -> Result<Self, String> {
        let ins: InstructionV2;
        for opcode_entry in OPCODE_TABLE.iter() {
            if opcode != 0 && (opcode & opcode_entry.mask == opcode_entry.opcode) {
                // debug!("input opcode = {:X}, mask = {:X}, actual code: {:X}", opcode, opcode_entry.mask, opcode_entry.opcode);
                ins = (opcode_entry.handler)(opcode);
                return Ok(ins);
            }
        }
        Err(format!("Opcode not found: {:X}", opcode))
    }
}

use crate::bitmasks::*;
#[cfg(test)]
use std::collections::HashMap;
mod tests {
    use super::*;

    #[test]
    fn test_opcode_table_simple() {
        let mut opcode_hash: HashMap<u16, InstructionV2> = HashMap::new();

        opcode_hash.insert(0x00E0, InstructionV2::ClearScreen);
        opcode_hash.insert(0x00EE, InstructionV2::Return);
        opcode_hash.insert(0x06B5, InstructionV2::SYS);
        opcode_hash.insert(0x16B5, InstructionV2::Jump(mask_0FFF(0x16B5)));
        opcode_hash.insert(0x26B5, InstructionV2::Call(mask_0FFF(0x26B5)));
        opcode_hash.insert(
            0x3D6B,
            InstructionV2::SkipEqualsByte(mask_0F00(0x3D6B), mask_00FF(0x3D6B)),
        );
        opcode_hash.insert(
            0x4D6B,
            InstructionV2::SkipNotEqualsByte(mask_0F00(0x4D6B), mask_00FF(0x4D6B)),
        );
        opcode_hash.insert(
            0x5DB0,
            InstructionV2::SkipEqualsRegister(mask_0F00(0x5DB0), mask_00F0(0x5DB0)),
        );
        opcode_hash.insert(
            0x6D6B,
            InstructionV2::LoadByte(mask_0F00(0x6D6B), mask_00FF(0x6D6B)),
        );
        opcode_hash.insert(
            0x7D6B,
            InstructionV2::AddByte(mask_0F00(0x7D6B), mask_00FF(0x7D6B)),
        );
        opcode_hash.insert(
            0x8DB0,
            InstructionV2::LoadRegister(mask_0F00(0x8DB0), mask_00F0(0x8DB0)),
        );
        opcode_hash.insert(
            0x8DB1,
            InstructionV2::Or(mask_0F00(0x8DB1), mask_00F0(0x8DB1)),
        );
        opcode_hash.insert(
            0x8DB2,
            InstructionV2::And(mask_0F00(0x8DB2), mask_00F0(0x8DB2)),
        );
        opcode_hash.insert(
            0x8DB3,
            InstructionV2::Xor(mask_0F00(0x8DB3), mask_00F0(0x8DB3)),
        );
        opcode_hash.insert(
            0x8DB4,
            InstructionV2::AddRegister(mask_0F00(0x8DB4), mask_00F0(0x8DB4)),
        );
        opcode_hash.insert(
            0x8DB5,
            InstructionV2::SubRegister(mask_0F00(0x8DB5), mask_00F0(0x8DB5)),
        );
        opcode_hash.insert(0x8DB6, InstructionV2::ShiftRight(mask_0F00(0x8DB6)));
        opcode_hash.insert(
            0x8DB7,
            InstructionV2::SubNRegister(mask_0F00(0x8DB7), mask_00F0(0x8DB7)),
        );
        opcode_hash.insert(0x8DBE, InstructionV2::ShiftLeft(mask_0F00(0x8DBE)));
        opcode_hash.insert(
            0x9DB0,
            InstructionV2::SkipNotEqualRegister(mask_0F00(0x9DB0), mask_00F0(0x9DB0)),
        );
        opcode_hash.insert(0xA6B5, InstructionV2::LoadImmediate(mask_0FFF(0xA6B5)));
        opcode_hash.insert(0xB6B5, InstructionV2::JumpBase(mask_0FFF(0xB6B5)));
        opcode_hash.insert(
            0xCD6B,
            InstructionV2::Random(mask_0F00(0xCD6B), mask_00FF(0xCD6B)),
        );
        opcode_hash.insert(
            0xDDB5,
            InstructionV2::DisplaySprite(mask_0F00(0xDDB5), mask_00F0(0xDDB5), mask_000F(0xDDB5)),
        );
        opcode_hash.insert(0xED9E, InstructionV2::SkipKeyPress(mask_0F00(0xED9E)));
        opcode_hash.insert(0xEDA1, InstructionV2::SkipNotKeyPress(mask_0F00(0xED9E)));
        opcode_hash.insert(0xFD07, InstructionV2::LoadFromDelay(mask_0F00(0xFD07)));
        opcode_hash.insert(0xFD0A, InstructionV2::LoadKeyPress(mask_0F00(0xFD0A)));
        opcode_hash.insert(0xFD15, InstructionV2::LoadDelay(mask_0F00(0xFD0A)));
        opcode_hash.insert(0xFD18, InstructionV2::LoadSound(mask_0F00(0xFD18)));
        opcode_hash.insert(0xFD1E, InstructionV2::AddI(mask_0F00(0xFD1E)));
        opcode_hash.insert(0xFD29, InstructionV2::LoadFontSprite(mask_0F00(0xFD1E)));
        opcode_hash.insert(0xFD33, InstructionV2::LoadIBCD(mask_0F00(0xFD33)));
        opcode_hash.insert(0xFD55, InstructionV2::StoreRegisters(mask_0F00(0xFD55)));
        opcode_hash.insert(0xFD65, InstructionV2::LoadRegisters(mask_0F00(0xFD55)));
        for (k, v) in opcode_hash.iter() {
            assert_eq!(*v, InstructionV2::try_from(*k).unwrap());
        }
    }

    #[test]
    fn test_bad_opcodes() {
        // Some negative tests for opcode construction
        let opcode = 0xFC14;
        let instruction = InstructionV2::try_from(opcode);
        assert_eq!(instruction, Err(format!("Opcode not found: {:X}", opcode)));

        let opcode = 0xEB8E;
        let instruction = InstructionV2::try_from(opcode);
        assert_eq!(instruction, Err(format!("Opcode not found: {:X}", opcode)));
    }
}
