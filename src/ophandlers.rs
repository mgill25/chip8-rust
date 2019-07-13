use crate::bitmasks::*;
use crate::opcodesv2::InstructionV2;

#[allow(non_snake_case)]
pub fn handle0x00E0(opcode: u16) -> InstructionV2 {
    InstructionV2::ClearScreen
}

#[allow(non_snake_case)]
pub fn handle0x00EE(opcode: u16) -> InstructionV2 {
    InstructionV2::Return
}

#[allow(non_snake_case)]
pub fn handle0x0NNN(opcode: u16) -> InstructionV2 {
    InstructionV2::SYS
}

#[allow(non_snake_case)]
pub fn handle0x1NNN(opcode: u16) -> InstructionV2 {
    InstructionV2::Jump(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x2NNN(opcode: u16) -> InstructionV2 {
    InstructionV2::Call(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x3XNN(opcode: u16) -> InstructionV2 {
    InstructionV2::SkipEqualsByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x4XNN(opcode: u16) -> InstructionV2 {
    InstructionV2::SkipNotEqualsByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x5XY0(opcode: u16) -> InstructionV2 {
    InstructionV2::SkipEqualsRegister(mask_0F00(opcode), mask_00F0(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x6XNN(opcode: u16) -> InstructionV2 {
    InstructionV2::LoadByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x7XNN(opcode: u16) -> InstructionV2 {
    InstructionV2::AddByte(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0x8XY0(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::LoadRegister(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XY1(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::Or(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XY2(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::And(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XY3(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::Xor(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XY4(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::AddRegister(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XY5(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::SubRegister(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XY6(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    InstructionV2::ShiftRight(r1)
}

#[allow(non_snake_case)]
pub fn handle0x8XY7(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    let r2 = mask_00F0(opcode);
    InstructionV2::SubNRegister(r1, r2)
}

#[allow(non_snake_case)]
pub fn handle0x8XYE(opcode: u16) -> InstructionV2 {
    let r1 = mask_0F00(opcode);
    InstructionV2::ShiftLeft(r1)
}

#[allow(non_snake_case)]
pub fn handle0x9XY0(opcode: u16) -> InstructionV2 {
    InstructionV2::SkipNotEqualRegister(mask_0F00(opcode), mask_00F0(opcode))
}

#[allow(non_snake_case)]
pub fn handle0xANNN(opcode: u16) -> InstructionV2 {
    InstructionV2::LoadImmediate(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0xBNNN(opcode: u16) -> InstructionV2 {
    InstructionV2::JumpBase(mask_0FFF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0xCXNN(opcode: u16) -> InstructionV2 {
    InstructionV2::Random(mask_0F00(opcode), mask_00FF(opcode))
}

#[allow(non_snake_case)]
pub fn handle0xDXYN(opcode: u16) -> InstructionV2 {
    InstructionV2::DisplaySprite(mask_0F00(opcode), mask_00F0(opcode), mask_000F(opcode))
}

#[allow(non_snake_case)]
pub fn handle0xEX9E(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::SkipKeyPress(register)
}

#[allow(non_snake_case)]
pub fn handle0xEXA1(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::SkipNotKeyPress(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX07(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadFromDelay(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX0A(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadKeyPress(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX15(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadDelay(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX18(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadSound(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX1E(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::AddI(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX29(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadFontSprite(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX33(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadIBCD(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX55(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::StoreRegisters(register)
}

#[allow(non_snake_case)]
pub fn handle0xFX65(opcode: u16) -> InstructionV2 {
    let register = mask_0F00(opcode);
    InstructionV2::LoadRegisters(register)
}
