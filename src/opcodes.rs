type Address = u16;
type Register = u8;
type Data = u8;

pub enum Instruction {
    ClearScreen,                              // CLS
    Return,                                   // RET
    SYS(Address),                             // SYS addr
    Jump(Address),                            //  JP addr
    Call(Address),                            // CALL addr
    SkipEqualsByte(Register, Data),           // SE vx, byte
    SkipNotEqualsByte(Register, Data),        // SNE Vx, byte
    SkipEqualsRegister(Register, Register),   // SE Vx, Vy
    LoadByte(Register, Data),                 // LD Vx, byte
    AddByte(Register, Data),                  // ADD Vx, byte
    LoadRegister(Register, Register),         // LD Vx, Vy
    Or(Register, Register),                   // OR Vx, Vy
    And(Register, Register),                  // AND Vx, Vy
    Xor(Register, Register),                  // XOR Vx, Vy
    AddRegister(Register, Register),          // ADD Vx, Vy
    SubRegister(Register, Register),          // SUB Vx, Vy
    ShiftRight(Register),                     // SHR Vx
    SubNRegister(Register, Register),         // SUBN Vx, Vy
    ShiftLeft(Register),                      // SHL Vx
    SkipNotEqualRegister(Register, Register), // SNE Vx, Vy
    LoadImmediate(Address),                   // LD I, addr
    JumpBase(Address),                        // JP V0, address
    Random(Register, Data),                   // RND Vx, byte
    DisplaySprite(Register, Register, u8),    // DRW Vx, Vy, nibble
    SkipKeyPress(Register),                   // SKP Vx
    SkipNotKeyPress(Register),                // SKNP Vx
    LoadFromDelay(Register),                  // LD Vx, DT
    LoadKeyPress(Register),                   // LD Vx, K
    LoadDelay(Register),                      // LD DT, Vx
    LoadSound(Register),                      // LD ST, Vx
    AddI(Register),                           // ADD I, Vx
    LoadFontSprite(Register),                 // LD F, Vx
    LoadIBCD(Register),                       // LD B, Vx
    StoreRegisters(Register),                 // LD [I], Vx
    LoadRegisters(Register),                  // LD Vx, [I]
}
