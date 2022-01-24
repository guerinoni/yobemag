#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    // LD r, n
    // Load to the 8-bit register r, the immediate data n.
    // Clock cycles: 8
    LdBNext,
    LdCNext,
    LdDNext,
    LdENext,
    LdHNext,
    LdLNext,
    LdANext,

    // LD r, r’
    // Load to the 8-bit register r, data from the 8-bit register r’.
    // Clock cycles: 4
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

    // LD r, (HL)
    // Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL.
    // Clock cycles: 8
    LdBHL,
    LdCHL,
    LdDHL,
    LdEHL,
    LdHHL,
    LdLHL,
    LdAHL,

    // LD (HL), r
    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r.
    // Clock cycles: 8
    LdHlB,
    LdHlC,
    LdHlD,
    LdHlE,
    LdHlH,
    LdHlL,
    LdHlA,

    // LD A, (BC)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register BC.
    // Clock cycles: 8
    LdABc,

    // LD A, (DE)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register DE.
    // Clock cycles: 8
    LdADe,

    // LD A, (nn)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit operand nn.
    // Clock cycles: 16
    LdANn,

    // LD r, (HL)
    // Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL.
    // Duration 2 machine cycles
    LdHlN,

    // LD (BC), a
    // Load to the absolute address specified by the 16-bit register BC, data from the 8-bit A register.
    // Clock cycles: 8
    LdBcA,

    // LD (DE), a
    // Load to the absolute address specified by the 16-bit register DE, data from the 8-bit A register.
    // Clock cycles: 8
    LdDeA,

    // LD (nn), A
    // Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit A register.
    // Clock cycles: 16
    LdNnA,

    // LD rr, nn
    // Load to the 16-bit register rr, the immediate 16-bit data nn.
    // Clock cycles: 12
    LdBcNn,
    LdDeNn,
    LdHlNn,
    LdSpNn,

    // LD (HL-), A
    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    // HL is decremented after the memory write.
    // Clock cycles: 8
    LddHlA,

    // LDH A, (n)
    // Load to the 8-bit A register, data from the address specified by the 8-bit immediate data n. The full 16-bit
    // absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    // value of n, so the possible range is 0xFF00-0xFFFF.
    // Clock cycles: 12
    LdHAn,

    // LDH (n), A
    // Load to the address specified by the 8-bit immediate data n, data from the 8-bit A register. The full 16-bit
    // absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    // value of n, so the possible range is 0xFF00-0xFFFF.
    // Clock cycles: 12
    LdHnA,

    // LDH A, (C)
    // Load to the 8-bit A register, data from the address specified by the 8-bit C register. The full 16-bit absolute
    // address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    // so the possible range is 0xFF00-0xFFFF.
    // Clock cycles: 8
    LdHAC,

    // LDH (C), A
    // Load to the address specified by the 8-bit C register, data from the 8-bit A register. The full 16-bit absolute
    // address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    // so the possible range is 0xFF00-0xFFFF.
    // Clock cycles: 8
    LdHCA,

    // LD A, (HL-)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    // HL is decremented after the memory read.
    // Clock cycles: 8
    LddAHl,

    // LD (nn), SP
    // Load to the absolute address specified by the 16-bit operand nn, data from the 16-bit SP register.
    // Clock cycles: 20
    LdNnSP,

    // LD A, (HL+)
    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    // HL is incremented after the memory read.
    // Clock cycles: 8
    LdiAHl,

    // LD (HL+), A
    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    // HL is incremented after the memory write.
    // Clock cycles: 8
    LdiHlA,

    // LD SP, HL
    // Load to the 16-bit SP register, data from the 16-bit HL register.
    // Clock cycles: 4
    LdSpHl,

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

    /// OR (HL)
    /// A bitwise OR operation is performed between the byte at the memory
    /// address specified in the virtual 16-bit register HL and the contents
    /// of register A, and the result is stored in register A.
    /// Clock cycles: 8
    OrHl,

    /// OR n
    /// A bitwise OR operation is performed between the byte n and the
    /// contents of register A, and the result is stored in register A.
    /// Clock cycles: 8
    OrN,

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

    /// XOR (HL)
    /// A bitwise XOR operation is performed between the byte at the memory address specified in the virtual 16-bit register HL and the contents
    /// of register A, and the result is stored in register A.
    /// Clock cycles: 8
    XorHl,

    /// XOR n
    /// A bitwise XOR operation is performed between the byte n and the contents of register A,
    /// and the result is stored in register A.
    /// Clock cycles: 8
    XorN,

    /// ADD HL, rr
    /// The contents of the register pair rr are added to the contents of the
    /// register pair HL and the result is stored in HL. Register pair rr may be any of BC, DE, HL or SP.
    /// Clock cycles: 8
    AddHlBc,
    AddHlDe,
    AddHlHl,
    AddHlSp,

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

    /// DEC rr
    /// The register pair rr is decremented by 1. Register pair rr may be any of BC, DE, HL or SP.
    /// Clock cycles: 8
    DecBc,
    DecDe,
    DecHl,
    DecSp,

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

    /// DEC (HL)
    /// The byte at the memory address specified in the register HL is decremented by 1.
    /// Clock cycles: 12
    DecHlSpecific,

    /// JP nn
    /// The 16-bit word nn is loaded into the program counter, from where execution continues.
    /// Clock cycles: 16
    JpNN,

    /// JP HL
    /// The contents of the register pair HL are loaded into the program
    /// counter, from where execution continues.
    /// Clock cycles: 4
    JpHl,

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

    /// JP f, nn
    /// The 16-bit word nn is loaded into the program counter only if the
    /// condition f is true. Execution will then continue from the program
    /// counter. Condition f may be any of nz, z, nc or c.
    /// Clock cycles: 16 if condition is met, otherwise 12
    JpNzNn,
    JpZNn,
    JpNcNn,
    JpCNn,

    /// CALL f, nn
    /// Only if the condition f is true is the current program counter (return
    /// address) pushed to the stack, high-order byte first, and the 16-bit word
    /// nn loaded into the program counter. Execution will them continue from
    /// the program counter. Condition f may be any of nz, z, nc or c.
    /// Clock cycles: 24 if condition is met, otherwise 12
    CallNzNn,
    CallZNn,
    CallNcNn,
    CallCNn,

    /// CP n
    /// The byte n is compared with (subtracted from) the register A, setting
    /// the appropriate flags but not storing the result.
    /// Clock cycles: 8
    CpN,

    /// RRA
    /// The contents of register A are rotated right by 1 bit position through the carry flag.
    /// Clock cycles: 4
    RrA,

    // LD HL, SP+s
    // The byte s is read as a signed integer and added to the register pair SP.
    // The result is then loaded into the register pair HL.
    // Clock cycles: 12
    LdHlSps,

    /// RLCA
    /// The contents of register A are rotated left by 1 bit position, after the sign bit (7) is copied into the carry flag.
    /// Clock cycles: 4
    Rlca,

    /// RET
    /// The 16-bit word on top of the stack is popped off, low-order byte first,
    /// and loaded into the program counter, from where execution continues.
    /// Clock cycles: 16
    Ret,

    /// RET f
    /// Only if the condition f is true is the 16-bit word on top of the stack
    /// popped off and loaded into the program counter. Execution will then
    /// continue from the program counter.
    /// Clock cycles: 20 if condition is met, otherwise 8
    RetNz,
    RetZ,
    RetNc,
    RetC,

    // PUSH rr
    // Push to the stack memory, data from the 16-bit register rr.
    // The contents of the register pair qq are pushed to the stack. First,
    // the stack pointer (SP) is decremented, and the high-order byte of qq
    // is loaded to the byte at the memory address specified by SP. Then,
    // SP is decremented again and the low-order byte of qq is loaded into
    // the byte at the memory address specified by SP. The register pair qq
    // may be any of BC, DE, HL or AF.
    // Clock cycles: 16
    PushBc,
    PushDe,
    PushHl,
    PushAf,

    // POP rr
    // Pops to the 16-bit register rr, data from the stack memory.
    // This instruction does not do calculations that affect flags, but POP AF completely replaces the F register
    // value, so all flags are changed based on the 8-bit data that is read from memory.
    // The top two bytes of the stack are popped into the register pair qq.
    // First, the byte at the memory address specified by SP is loaded into
    // the low-order byte of qq. SP is then incremented, and the byte at the
    // memory address which it now specifies is loaded into the high-order
    // byte of qq, and SP is incremented again. The register pair qq may be
    // any of BC, DE, HL or AF.
    // Clock cycles: 12
    PopBc,
    PopDe,
    PopHl,
    PopAf,

    // ADD A, n
    // Byte n is read as an integer and added to the contents of register A, and the result is stored in register A.
    // Clock cycles: 8
    AddaN,

    // ADD A, r:
    // The contents of register r are added to the contents of register A
    // (the Accumulator) and the result is stored in register A.
    // Clock cycles: 4
    AddaB,
    AddaC,
    AddaD,
    AddaE,
    AddaH,
    AddaL,
    AddaA,

    // ADD A,(HL)
    // Clock cycles: 8
    AddAHl,

    // ADC r
    // Clock cycles: 4
    AdcB,
    AdcC,
    AdcD,
    AdcE,
    AdcH,
    AdcL,
    AdcA,

    // ADC A, (HL)
    // The byte at the memory address specified in the virtual 16-bit
    // register HL along with the value of the carry flag are added to the
    // register A and the result is stored in register A.
    // Clock cycles: 8
    AdcAHl,

    // ADC A, n
    // Byte n is read as an integer and added to the contents of register
    // A along with the value of the carry flag. The result is then stored in register A.
    // Clock cycles: 8
    AdcAn,

    // SUB r
    // The contents of the register r are subtracted from the contents of register A, and the result is stored in register A.
    // Register r may be any of B, C, D, E, H, L, or A.
    // Clock cycles: 4
    SubB,
    SubC,
    SubD,
    SubE,
    SubH,
    SubL,
    SubA,

    // SUB (HL)
    // The byte at the memory address specified in the virtual 16-bit
    // register HL is subtracted from the register A and the result is
    // stored in register A.
    // Clock cycles: 8
    SubHl,

    // SUB n
    // Byte n is read as an integer and subtracted from the contents of register A, and the result is stored in register A.
    // Clock cycles: 8
    SubN,

    // SBC A, r
    // The contents of the register r along with the value of the carry
    // flag are both subtracted from the register A, and the result is
    // stored in register A. Register r may be any of B, C, D, E, H, L or A.
    // Clock cycles: 4
    SbcAB,
    SbcAC,
    SbcAD,
    SbcAE,
    SbcAH,
    SbcAL,
    SbcAA,

    // SBC A, (HL)
    // The byte at the memory address specified in the virtual 16-bit
    // register HL and the value of the carry flag are both subtracted from
    // the register A, and the result is stored in register A.
    // Clock cycles: 8
    SbcAHl,

    // SBC A, n
    // Byte n is read as an integer and along with the value of the carry
    // flag, it is subtracted from register A, and the result is stored in
    // register A.
    // Clock cycles: 8
    SbcAn,


    /// AND n
    /// A bitwise AND operation is performed between the byte n and the contents of register A, and the result is stored in register A.
    /// Clock cycles: 8
    AndN,

    /// DI
    /// Interrupts are disabled by resetting the Interrupt Master Flag (IME).
    /// Clock cycles: 4
    DI,

    /// CALL nn
    /// The current program counter (return address) is pushed to the stack, high-order byte first.
    /// The 16-bit word nn is then loaded into the program counter, from where execution continues.
    /// Clock cycles: 24
    CallNn,

    /// RST n
    /// The current program counter is pushed onto the stack, high-order byte
    /// first.
    /// The value of the operand n is then loaded into the program counter, from
    /// where execution continues. Operand n may be any of 0x00, 0x08, 0x10, 0x18,
    /// 0x20, 0x28, 0x30 or 0x38.
    /// Clock cycles: 16
    Rst00,
    Rst08,
    Rst10,
    Rst18,
    Rst20,
    Rst28,
    Rst30,
    Rst38,

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
            0x07 => OpCode::Rlca,
            0x08 => OpCode::LdNnSP,
            0x0A => OpCode::LdABc,
            0x0B => OpCode::DecBc,
            0x0C => OpCode::IncC,
            0x0D => OpCode::DecC,
            0x0E => OpCode::LdCNext,
            0x09 => OpCode::AddHlBc,
            0x10 => OpCode::Stop,
            0x11 => OpCode::LdDeNn,
            0x13 => OpCode::IncDE,
            0x12 => OpCode::LdDeA,
            0x14 => OpCode::IncD,
            0x15 => OpCode::DecD,
            0x16 => OpCode::LdDNext,
            0x18 => OpCode::JrPcDd,
            0x1A => OpCode::LdADe,
            0x1B => OpCode::DecDe,
            0x1C => OpCode::IncE,
            0x1D => OpCode::DecE,
            0x1E => OpCode::LdENext,
            0x1F => OpCode::RrA,
            0x19 => OpCode::AddHlDe,
            0x20 => OpCode::JrNzPcDd,
            0x21 => OpCode::LdHlNn,
            0x22 => OpCode::LdiHlA,
            0x23 => OpCode::IncHL,
            0x24 => OpCode::IncH,
            0x25 => OpCode::DecH,
            0x26 => OpCode::LdHNext,
            0x29 => OpCode::AddHlHl,
            0x2A => OpCode::LdiAHl,
            0x2B => OpCode::DecHl,
            0x28 => OpCode::JrZPcDd,
            0x2C => OpCode::IncL,
            0x2D => OpCode::DecL,
            0x2E => OpCode::LdLNext,
            0x30 => OpCode::JrNcPcDd,
            0x31 => OpCode::LdSpNn,
            0x32 => OpCode::LddHlA,
            0x33 => OpCode::IncSP,
            0x36 => OpCode::LdHlN,
            0x35 => OpCode::DecHlSpecific,
            0x38 => OpCode::JrCPcDd,
            0x3A => OpCode::LddAHl,
            0x3B => OpCode::DecSp,
            0x3C => OpCode::IncA,
            0x3D => OpCode::DecA,
            0x3E => OpCode::LdANext,
            0x39 => OpCode::AddHlSp,
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
            0x80 => OpCode::AddaB,
            0x81 => OpCode::AddaC,
            0x82 => OpCode::AddaD,
            0x83 => OpCode::AddaE,
            0x84 => OpCode::AddaH,
            0x85 => OpCode::AddaL,
            0x86 => OpCode::AddAHl,
            0x88 => OpCode::AdcB,
            0x89 => OpCode::AdcC,
            0x87 => OpCode::AddaA,
            0x8A => OpCode::AdcD,
            0x8B => OpCode::AdcE,
            0x8C => OpCode::AdcH,
            0x8D => OpCode::AdcL,
            0x8E => OpCode::AdcAHl,
            0x8F => OpCode::AdcA,
            0x90 => OpCode::SubB,
            0x91 => OpCode::SubC,
            0x92 => OpCode::SubD,
            0x93 => OpCode::SubE,
            0x94 => OpCode::SubH,
            0x95 => OpCode::SubL,
            0x96 => OpCode::SubHl,
            0x98 => OpCode::SbcAB,
            0x97 => OpCode::SubA,
            0x99 => OpCode::SbcAC,
            0x9A => OpCode::SbcAD,
            0x9B => OpCode::SbcAE,
            0x9C => OpCode::SbcAH,
            0x9D => OpCode::SbcAL,
            0x9E => OpCode::SbcAHl,
            0x9F => OpCode::SbcAA,
            0xA8 => OpCode::XorB,
            0xA9 => OpCode::XorC,
            0xAA => OpCode::XorD,
            0xAB => OpCode::XorE,
            0xAC => OpCode::XorH,
            0xAD => OpCode::XorL,
            0xAE => OpCode::XorHl,
            0xAF => OpCode::XorA,
            0xB0 => OpCode::OrB,
            0xB1 => OpCode::OrC,
            0xB2 => OpCode::OrD,
            0xB3 => OpCode::OrE,
            0xB4 => OpCode::OrH,
            0xB5 => OpCode::OrL,
            0xB6 => OpCode::OrHl,
            0xB7 => OpCode::OrA,
            0xC0 => OpCode::RetNz,
            0xC1 => OpCode::PopBc,
            0xC2 => OpCode::JpNzNn,
            0xC3 => OpCode::JpNN,
            0xC4 => OpCode::CallNzNn,
            0xC5 => OpCode::PushBc,
            0xC6 => OpCode::AddaN,
            0xC7 => OpCode::Rst00,
            0xc8 => OpCode::RetZ,
            0xC9 => OpCode::Ret,
            0xCA => OpCode::JpZNn,
            0xCB => OpCode::CB,
            0xCC => OpCode::CallZNn,
            0xCD => OpCode::CallNn,
            0xCE => OpCode::AdcAn,
            0xCF => OpCode::Rst08,
            0xD0 => OpCode::RetNc,
            0xD1 => OpCode::PopDe,
            0xD2 => OpCode::JpNcNn,
            0xD4 => OpCode::CallNcNn,
            0xD5 => OpCode::PushDe,
            0xD6 => OpCode::SubN,
            0xD7 => OpCode::Rst10,
            0xD8 => OpCode::RetC,
            0xDA => OpCode::JpCNn,
            0xDC => OpCode::CallCNn,
            0xDE => OpCode::SbcAn,
            0xDF => OpCode::Rst18,
            0xE0 => OpCode::LdHnA,
            0xE1 => OpCode::PopHl,
            0xE2 => OpCode::LdHCA,
            0xE5 => OpCode::PushHl,
            0xE6 => OpCode::AndN,
            0xE7 => OpCode::Rst20,
            0xE9 => OpCode::JpHl,
            0xEA => OpCode::LdNnA,
            0xEE => OpCode::XorN,
            0xEF => OpCode::Rst28,
            0xF0 => OpCode::LdHAn,
            0xF1 => OpCode::PopAf,
            0xF2 => OpCode::LdHAC,
            0xF3 => OpCode::DI,
            0xF5 => OpCode::PushAf,
            0xF6 => OpCode::OrN,
            0xF7 => OpCode::Rst30,
            0xF8 => OpCode::LdHlSps,
            0xF9 => OpCode::LdSpHl,
            0xFA => OpCode::LdANn,
            0xFE => OpCode::CpN,
            0xFF => OpCode::Rst38,

            _ => panic!("unknown opcode {}", orig),
        }
    }
}
