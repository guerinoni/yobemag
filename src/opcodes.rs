use crate::opcodes;

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum RegisterWord {
    BC,
    DE,
    HL,
    SP,
}

pub enum ConditionOperand {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    /// LD r, n
    /// Byte n is read as an integer and loaded into register r.
    /// Clock cycles: 8
    LdBNext,
    LdCNext,
    LdDNext,
    LdENext,
    LdHNext,
    LdLNext,
    LdANext,

    /// LD r, r'
    /// The contents of the register r' are loaded to the register r.
    /// Clock cycles: 4
    LdBB,
    LdBC,
    LdBD,
    LdBE,
    LdBH,
    LdBL,
    LdBA,

    LdCB,
    LdCC,
    LdCD,
    LdCE,
    LdCH,
    LdCL,
    LdCA,

    LdDB,
    LdDC,
    LdDD,
    LdDE,
    LdDH,
    LdDL,
    LdDA,

    LdEB,
    LdEC,
    LdED,
    LdEE,
    LdEH,
    LdEL,
    LdEA,

    LdHB,
    LdHC,
    LdHD,
    LdHE,
    LdHH,
    LdHL,
    LdHA,

    LdLB,
    LdLC,
    LdLD,
    LdLE,
    LdLH,
    LdLL,
    LdLA,

    LdAB,
    LdAC,
    LdAD,
    LdAE,
    LdAH,
    LdAL,
    LdAA,

    /// LD r, (HL)
    /// The byte at the memory address stored in the register pair HL is loaded into register r.
    /// Clock cycles: 8
    LdBHL,
    LdCHL,
    LdDHL,
    LdEHL,
    LdHHL,
    LdLHL,
    LdAHL,

    /// LD (HL), r
    /// The contents of register r are loaded into the byte at the memory address specified in the register pair HL.
    /// Clock cycles: 8
    LdHlB,
    LdHlC,
    LdHlD,
    LdHlE,
    LdHlH,
    LdHlL,
    LdHlA,

    /// LD A, (BC)
    /// The byte at the memory address specified in the register pair BC is loaded in to the register A.
    /// Clock cycles: 8
    LdABc,

    /// LD A, (DE)
    /// The byte at the memory address specified in the register pair DE is loaded in to the register A.
    /// Clock cycles: 8
    LdADe,

    /// LD A, (nn)
    /// The byte at the memory address specified by the 16-bit short nn is loaded into the register A.
    /// Clock cycles: 16
    LdANn,

    /// LD (HL), n
    /// Byte n is read as an integer and loaded into the memory address specified in the register pair HL.
    /// Clock cycles: 12
    LdHlN,

    /// LD (BC), A
    /// The contents of the register A are loaded into the byte at the
    /// memory address specified in the register pair BC.
    /// Clock cycles: 8
    LdBcA,

    /// LD (DE), A
    /// The contents of the register A are loaded into the byte at the memory address specified in the register pair BC.
    /// Clock cycles: 8
    LdDeA,

    /// LD (nn), A
    /// The contents of the register A are loaded into the byte at the memory address specified by the 16-bit short nn.
    /// Clock cycles: 16
    LDNnA,

    /// LD dd, nn
    /// The 16-bit short nn is read as an integer and loaded into the register pair dd.
    /// Clock cycles: 12
    LdBcNn,
    LdDeNn,
    LdHlNn,
    LdSpNn,

    /// LDD (HL), A
    /// The contents of the register A are loaded into the byte at the memory address specified by the register pair HL.
    /// HL is then decremented by 1.
    /// Clock cycles: 8
    LddHlA,

    /// LD A, (FF00+n)
    /// The byte at the memory address (FF00+n) - the nth I/O port - is
    /// loaded into the register A.
    /// Clock cycles: 12
    LdAFF00n,

    /// LD (FF00+n), A
    /// The contents of the register A are loaded into the byte at the  memory address (FF00+n) - the nth I/O port.
    /// Clock cycles: 12
    LdFF00nA,

    /// LD A, (FF00+C)
    /// The byte at the memory address (FF00 + register C) - the Cth I/O port - is loaded into the register A.
    /// Clock cycles: 8
    LdAFF00C,

    /// LD (FF00+C), A
    /// The contents of the register A are loaded into the byte at the memory address (FF00+C) - the Cth I/O port.
    /// Clock cycles: 8
    LdFF00CA,

    /// LDD A, (HL)
    /// The byte at the memory address specified by the register pair HL is loaded into the register A. HL is then decremented by 1.
    /// Clock cycles: 8
    LddAHl,

    /// LD (nn), SP
    /// The contents of the 16-bit SP register are loaded to the word at the memory address specified in the 16-bit operand nn.
    /// Clock cycles: 20
    LdNnSP,

    /// OR r
    /// A bitwise OR operation is performed between the contents of the register r and the contents of the register A, and the result is  stored in register A.
    /// Clock cycles: 4
    OrB,
    OrC,
    OrD,
    OrE,
    OrH,
    OrL,
    OrA,

    /// XOR r
    /// A bitwise XOR operation is performed between the contents of the register r and the contents of the register A, and the result is
    /// stored in register A.
    /// Clock cycles: 4
    XorB,
    XorC,
    XorD,
    XorE,
    XorH,
    XorL,
    XorA,

    /// INC rr
    /// The register pair rr is incremented by 1.
    /// Clock cycles: 8
    IncBC,
    IncDE,
    IncHL,
    IncSP,

    /// INC r
    /// The register r is incremented by 1.
    /// Clock cycles: 4
    IncB,
    IncC,
    IncD,
    IncE,
    IncH,
    IncL,
    IncA,

    /// DEC r
    /// The register r is decremented by 1.
    /// Clock cycles: 4
    DecB,
    DecC,
    DecD,
    DecE,
    DecH,
    DecL,
    DecA,

    /// JP nn
    /// The 16-bit word nn is loaded into the program counter, from where execution continues.
    /// Clock cycles: 16
    JpNN,

    /// JR f, PC+dd
    /// The 8-bit signed integer dd is added to the program counter and the result is stored in the program counter only if the condition f is true.
    /// Execution will then continue from the program counter.
    /// Condition f may be any of nz, z, nc or c.
    /// Clock cycles: 12 if condition is met, otherwise 8
    JrNzPcDd,
    JrZPcDd,
    JrNcPcDd,
    JrCPcDd,

    /// JR PC+dd
    /// The 8-bit signed integer dd is added to the program counter and the
    /// result is stored in the program counter, from where execution continues.
    /// Clock cycles: 12
    JrPcDd,

    /// CP n
    /// The byte n is compared with (subtracted from) the register A, setting
    /// the appropriate flags but not storing the result.
    /// Clock cycles: 8
    CpN,

    /// RRA
    /// The contents of register A are rotated right by 1 bit position through the carry flag.
    /// Clock cycles: 4
    RrA,

    /// RLCA
    /// The contents of register A are rotated left by 1 bit position, after the sign bit (7) is copied into the carry flag.
    /// Clock cycles: 4
    RLCA,

    /// DI
    /// Interrupts are disabled by resetting the Interrupt Master Flag (IME).
    /// Clock cycles: 4
    DI,

    /// CALL nn
    /// The current program counter (return address) is pushed to the stack, high-order byte first.
    /// The 16-bit word nn is then loaded into the program counter, from where execution continues.
    /// Clock cycles: 24
    CallNn,

    /// The CPU performs no operation during this cycle.
    /// Clock cycles: 4
    Noop,

    /// STOP
    /// CPU operation is stopped.
    /// Clock cycles: N/A
    Stop,

    /// HALT
    /// CPU operation is suspended until an interrupt or reset is recieved. While in
    /// this halted state, NOPs are executed to maintain memory refresh logic.
    /// Clock cycles: 4 (+4 for every following NOP)
    Halt,

    /// CB
    /// Interpret the next byte as a prefix instruction (PrefixOpCode) rather than a normal instruction (OpCode)
    CB,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => OpCode::Noop,
            0x01 => OpCode::LdBcNn,
            0x02 => OpCode::LdBcA,
            0x03 => OpCode::IncBC,
            0x04 => OpCode::IncB,
            0x05 => OpCode::DecB,
            0x06 => OpCode::LdBNext,
            0x07 => OpCode::RLCA,
            0x08 => OpCode::LdNnSP,
            0x0A => OpCode::LdABc,
            0x0C => OpCode::IncC,
            0x0D => OpCode::DecC,
            0x0E => OpCode::LdCNext,
            0x10 => OpCode::Stop,
            0x11 => OpCode::LdDeNn,
            0x13 => OpCode::IncDE,
            0x12 => OpCode::LdDeA,
            0x14 => OpCode::IncD,
            0x15 => OpCode::DecD,
            0x16 => OpCode::LdDNext,
            0x18 => OpCode::JrPcDd,
            0x1A => OpCode::LdADe,
            0x1C => OpCode::IncE,
            0x1D => OpCode::DecE,
            0x1E => OpCode::LdENext,
            0x1F => OpCode::RrA,
            0x20 => OpCode::JrNzPcDd,
            0x21 => OpCode::LdHlNn,
            0x23 => OpCode::IncHL,
            0x24 => OpCode::IncH,
            0x25 => OpCode::DecH,
            0x26 => OpCode::LdHNext,
            0x28 => OpCode::JrZPcDd,
            0x2C => OpCode::IncL,
            0x2D => OpCode::DecL,
            0x2E => OpCode::LdLNext,
            0x30 => OpCode::JrNcPcDd,
            0x31 => OpCode::LdSpNn,
            0x32 => OpCode::LddHlA,
            0x33 => OpCode::IncSP,
            0x36 => OpCode::LdHlN,
            0x38 => OpCode::JrCPcDd,
            0x3A => OpCode::LddAHl,
            0x3C => OpCode::IncA,
            0x3D => OpCode::DecA,
            0x3E => OpCode::LdANext,
            0x40 => OpCode::LdBB,
            0x41 => OpCode::LdBC,
            0x42 => OpCode::LdBD,
            0x43 => OpCode::LdBE,
            0x44 => OpCode::LdBH,
            0x45 => OpCode::LdBL,
            0x46 => OpCode::LdBHL,
            0x47 => OpCode::LdBA,
            0x48 => OpCode::LdCB,
            0x49 => OpCode::LdCC,
            0x4A => OpCode::LdCD,
            0x4B => OpCode::LdCE,
            0x4C => OpCode::LdCH,
            0x4D => OpCode::LdCL,
            0x4E => OpCode::LdCHL,
            0x4F => OpCode::LdCA,
            0x50 => OpCode::LdDB,
            0x51 => OpCode::LdDC,
            0x52 => OpCode::LdDD,
            0x53 => OpCode::LdDE,
            0x54 => OpCode::LdDH,
            0x55 => OpCode::LdDL,
            0x56 => OpCode::LdDHL,
            0x57 => OpCode::LdDA,
            0x58 => OpCode::LdEB,
            0x59 => OpCode::LdEC,
            0x5A => OpCode::LdED,
            0x5B => OpCode::LdEE,
            0x5C => OpCode::LdEH,
            0x5D => OpCode::LdEL,
            0x5E => OpCode::LdEHL,
            0x5F => OpCode::LdEA,
            0x60 => OpCode::LdHB,
            0x61 => OpCode::LdHC,
            0x62 => OpCode::LdHD,
            0x63 => OpCode::LdHE,
            0x64 => OpCode::LdHH,
            0x65 => OpCode::LdHL,
            0x66 => OpCode::LdHHL,
            0x67 => OpCode::LdHA,
            0x68 => OpCode::LdLB,
            0x69 => OpCode::LdLC,
            0x6A => OpCode::LdLD,
            0x6B => OpCode::LdLE,
            0x6C => OpCode::LdLH,
            0x6D => OpCode::LdLL,
            0x6E => OpCode::LdLHL,
            0x6F => OpCode::LdLA,
            0x70 => OpCode::LdHlB,
            0x71 => OpCode::LdHlC,
            0x72 => OpCode::LdHlD,
            0x73 => OpCode::LdHlE,
            0x74 => OpCode::LdHlH,
            0x75 => OpCode::LdHlL,
            0x76 => OpCode::Halt,
            0x77 => OpCode::LdHlA,
            0x78 => OpCode::LdAB,
            0x79 => OpCode::LdAC,
            0x7A => OpCode::LdAD,
            0x7B => OpCode::LdAE,
            0x7C => OpCode::LdAH,
            0x7D => OpCode::LdAL,
            0x7E => OpCode::LdAHL,
            0x7F => OpCode::LdAA,
            0xA8 => OpCode::XorB,
            0xA9 => OpCode::XorC,
            0xAA => OpCode::XorD,
            0xAB => OpCode::XorE,
            0xAC => OpCode::XorH,
            0xAD => OpCode::XorL,
            0xAF => OpCode::XorA,
            0xB0 => OpCode::OrB,
            0xB1 => OpCode::OrC,
            0xB2 => OpCode::OrD,
            0xB3 => OpCode::OrE,
            0xB4 => OpCode::OrH,
            0xB5 => OpCode::OrL,
            0xB7 => OpCode::OrA,
            0xC3 => OpCode::JpNN,
            0xCB => OpCode::CB,
            0xCD => OpCode::CallNn,
            0xE0 => OpCode::LdFF00nA,
            0xE2 => OpCode::LdFF00CA,
            0xEA => OpCode::LDNnA,
            0xF0 => OpCode::LdAFF00n,
            0xF2 => OpCode::LdAFF00C,
            0xF3 => OpCode::DI,
            0xFA => OpCode::LdANn,
            0xFE => OpCode::CpN,

            _ => panic!("unknown opcode"),
        }
    }
}
