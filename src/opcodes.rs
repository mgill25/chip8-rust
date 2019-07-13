use std::convert::TryFrom;

type Address = u16;
type Register = u8;
type Data = u8;

#[derive(Debug)]
pub enum Instruction {
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

impl TryFrom<u16> for Instruction {
    type Error = String;
    fn try_from(opcode: u16) -> Result<Self, String> {
        match mask_F000(opcode) {
            0x0 => match mask_00FF(opcode) {
                0xE0 => Ok(Instruction::ClearScreen),
                0xEE => Ok(Instruction::Return),
                _ => Ok(Instruction::SYS),
            },
            0x1 => Ok(Instruction::Jump(mask_0FFF(opcode))),
            0x2 => Ok(Instruction::Call(mask_0FFF(opcode))),
            0x3 => Ok(Instruction::SkipEqualsByte(
                mask_0F00(opcode),
                mask_00FF(opcode),
            )),
            0x4 => Ok(Instruction::SkipNotEqualsByte(
                mask_0F00(opcode),
                mask_00FF(opcode),
            )),
            0x5 => Ok(Instruction::SkipEqualsRegister(
                mask_0F00(opcode),
                mask_00F0(opcode),
            )),
            0x6 => Ok(Instruction::LoadByte(mask_0F00(opcode), mask_00FF(opcode))),
            0x7 => Ok(Instruction::AddByte(mask_0F00(opcode), mask_00FF(opcode))),
            0x8 => {
                let r1 = mask_0F00(opcode);
                let r2 = mask_00F0(opcode);
                match mask_000F(opcode) {
                    0x0 => Ok(Instruction::LoadRegister(r1, r2)),
                    0x1 => Ok(Instruction::Or(r1, r2)),
                    0x2 => Ok(Instruction::And(r1, r2)),
                    0x3 => Ok(Instruction::Xor(r1, r2)),
                    0x4 => Ok(Instruction::AddRegister(r1, r2)),
                    0x5 => Ok(Instruction::SubRegister(r1, r2)),
                    0x6 => Ok(Instruction::ShiftRight(r1)),
                    0x7 => Ok(Instruction::SubNRegister(r1, r2)),
                    0xE => Ok(Instruction::ShiftLeft(r1)),
                    _ => Err(format!("Invalid opcode: {:X}", opcode)),
                }
            }
            0x9 => Ok(Instruction::SkipNotEqualRegister(
                mask_0F00(opcode),
                mask_00F0(opcode),
            )),
            0xA => Ok(Instruction::LoadImmediate(mask_0FFF(opcode))),
            0xB => Ok(Instruction::JumpBase(mask_0FFF(opcode))),
            0xC => Ok(Instruction::Random(mask_0F00(opcode), mask_00FF(opcode))),
            0xD => Ok(Instruction::DisplaySprite(
                mask_0F00(opcode),
                mask_00F0(opcode),
                mask_000F(opcode),
            )),
            0xE => {
                let register = mask_0F00(opcode);
                match mask_00FF(opcode) {
                    0x9E => Ok(Instruction::SkipKeyPress(register)),
                    0xA1 => Ok(Instruction::SkipNotKeyPress(register)),
                    _ => Err(format!("Invalid opcode: {:X}", opcode)),
                }
            }
            0xF => {
                let register = mask_0F00(opcode);
                match mask_00FF(opcode) {
                    0x07 => Ok(Instruction::LoadFromDelay(register)),
                    0x0A => Ok(Instruction::LoadKeyPress(register)),
                    0x15 => Ok(Instruction::LoadDelay(register)),
                    0x18 => Ok(Instruction::LoadSound(register)),
                    0x1E => Ok(Instruction::AddI(register)),
                    0x29 => Ok(Instruction::LoadFontSprite(register)),
                    0x33 => Ok(Instruction::LoadIBCD(register)),
                    0x55 => Ok(Instruction::StoreRegisters(register)),
                    0x65 => Ok(Instruction::LoadRegisters(register)),
                    _ => Err(format!("Invalid opcode: {:X}", opcode)),
                }
            }
            _ => Err(format!("opcode {:X} is invalid", opcode)),
        }
    }
}

#[allow(non_snake_case)]
fn mask_F000(opcode: u16) -> u8 {
    ((opcode >> 12) & 0xF) as u8
}

#[allow(non_snake_case)]
fn mask_00FF(opcode: u16) -> u8 {
    (opcode & 0xFF) as u8
}

#[allow(non_snake_case)]
fn mask_0F00(opcode: u16) -> u8 {
    ((opcode >> 8) & 0xF) as u8
}

#[allow(non_snake_case)]
fn mask_0FFF(opcode: u16) -> u16 {
    opcode & 0xFFF
}

#[allow(non_snake_case)]
fn mask_00F0(opcode: u16) -> u8 {
    ((opcode >> 4) & 0xF) as u8
}

#[allow(non_snake_case)]
fn mask_000F(opcode: u16) -> u8 {
    (opcode & 0xF) as u8
}
