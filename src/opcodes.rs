pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    /// LD r, r'
    /// The contents of the register r' are loaded to the register r.
    /// Registers r and r' may be any combination of B, C, D, E, H, L or A.
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

    /// LD r, n
    /// Byte n is read as an integer and loaded into register r. Register
    /// r may be any of B, C, D, E, H, L or A.
    /// Clock cycles: 8
    LdBNext,
    LdCNext,
    LdDNext,
    LdENext,
    LdHNext,
    LdLNext,
    LdANext,

    /// LD r, (HL)
    /// The byte at the memory address stored in the register pair HL is
    /// loaded into register r. Register r may be any of B, C, D, E, H, L
    /// or A.
    /// Clock cycles: 8
    LdBHL,
    LdCHL,
    LdDHL,
    LdEHL,
    LdHHL,
    LdLHL,
    LdAHL,

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
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => OpCode::Noop,
            0x76 => OpCode::Halt,
            0x10 => OpCode::Stop,

            0x40 => OpCode::LdBB,
            0x41 => OpCode::LdBC,
            0x42 => OpCode::LdBD,
            0x43 => OpCode::LdBE,
            0x44 => OpCode::LdBH,
            0x45 => OpCode::LdBL,
            0x47 => OpCode::LdBA,
            0x48 => OpCode::LdCB,
            0x49 => OpCode::LdCC,
            0x4A => OpCode::LdCD,
            0x4B => OpCode::LdCE,
            0x4C => OpCode::LdCH,
            0x4D => OpCode::LdCL,
            0x4F => OpCode::LdCA,
            0x50 => OpCode::LdDB,
            0x51 => OpCode::LdDC,
            0x52 => OpCode::LdDD,
            0x53 => OpCode::LdDE,
            0x54 => OpCode::LdDH,
            0x55 => OpCode::LdDL,
            0x57 => OpCode::LdDA,
            0x58 => OpCode::LdEB,
            0x59 => OpCode::LdEC,
            0x5A => OpCode::LdED,
            0x5B => OpCode::LdEE,
            0x5C => OpCode::LdEH,
            0x5D => OpCode::LdEL,
            0x5F => OpCode::LdEA,
            0x60 => OpCode::LdHB,
            0x61 => OpCode::LdHC,
            0x62 => OpCode::LdHD,
            0x63 => OpCode::LdHE,
            0x64 => OpCode::LdHH,
            0x65 => OpCode::LdHL,
            0x67 => OpCode::LdHA,
            0x68 => OpCode::LdLB,
            0x69 => OpCode::LdLC,
            0x6A => OpCode::LdLD,
            0x6B => OpCode::LdLE,
            0x6C => OpCode::LdLH,
            0x6D => OpCode::LdLL,
            0x6F => OpCode::LdLA,
            0x78 => OpCode::LdAB,
            0x79 => OpCode::LdAC,
            0x7A => OpCode::LdAD,
            0x7B => OpCode::LdAE,
            0x7C => OpCode::LdAH,
            0x7D => OpCode::LdAL,
            0x7F => OpCode::LdAA,

            0x6 => OpCode::LdBNext,
            0xE => OpCode::LdCNext,
            0x16 => OpCode::LdDNext,
            0x1E => OpCode::LdENext,
            0x26 => OpCode::LdHNext,
            0x2E => OpCode::LdLNext,
            0x3E => OpCode::LdANext,

            0x46 => OpCode::LdBHL,
            0x4E => OpCode::LdCHL,
            0x56 => OpCode::LdDHL,
            0x5E => OpCode::LdEHL,
            0x66 => OpCode::LdHHL,
            0x6E => OpCode::LdLHL,
            0x7E => OpCode::LdAHL,

            _ => panic!("unknown ram size"),
        }
    }
}
