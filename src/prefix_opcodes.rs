pub enum PrefixOpCode {
    /// RLC r
    /// The contents of the register r are rotated left by 1 bit position, after the sign bit (7)
    /// is copied into the carry flag.
    /// Clock cycles: 8
    RlcB,
    RlcC,
    RlcD,
    RlcE,
    RlcH,
    RlcL,
    RlcA,

    /// SRL r
    /// The contents of the register r are shifted right by 1 bit position, after bit 0
    /// is copied into the carry flag. Register r may be any of B, C, D, E, H, L or A.
    /// Clock cycles: 8
    SrlB,
    SrlC,
    SrlD,
    SrlE,
    SrlH,
    SrlL,
    SrlA,

    /// RR r
    /// The contents of the register r are rotated right by 1 bit position through the carry flag.
    /// Register r may be any of B, C, D, E, H, L or A.
    /// Clock cycles: 8
    RrB,
    RrC,
    RrD,
    RrE,
    RrH,
    RrL,
    RrA,

    /// SWAP r
    /// The upper and lower nibbles of the register r are swapped. Register r may be any of B, C, D, E, H, L or A.
    /// Clock cycles: 8
    SwapB,
    SwapC,
    SwapD,
    SwapE,
    SwapH,
    SwapL,
    SwapA,
}

impl From<u8> for PrefixOpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x00 => PrefixOpCode::RlcB,
            0x01 => PrefixOpCode::RlcC,
            0x02 => PrefixOpCode::RlcD,
            0x03 => PrefixOpCode::RlcE,
            0x04 => PrefixOpCode::RlcH,
            0x05 => PrefixOpCode::RlcL,
            0x07 => PrefixOpCode::RlcA,
            0x18 => PrefixOpCode::RrB,
            0x19 => PrefixOpCode::RrC,
            0x1A => PrefixOpCode::RrD,
            0x1B => PrefixOpCode::RrE,
            0x1C => PrefixOpCode::RrH,
            0x1D => PrefixOpCode::RrL,
            0x1F => PrefixOpCode::RrA,
            0x30 => PrefixOpCode::SwapB,
            0x31 => PrefixOpCode::SwapC,
            0x32 => PrefixOpCode::SwapD,
            0x33 => PrefixOpCode::SwapE,
            0x34 => PrefixOpCode::SwapH,
            0x35 => PrefixOpCode::SwapL,
            0x37 => PrefixOpCode::SwapA,
            0x38 => PrefixOpCode::SrlB,
            0x39 => PrefixOpCode::SrlC,
            0x3A => PrefixOpCode::SrlD,
            0x3B => PrefixOpCode::SrlE,
            0x3C => PrefixOpCode::SrlH,
            0x3D => PrefixOpCode::SrlL,
            0x3F => PrefixOpCode::SrlA,
            _ => panic!("unknown prefix opcode {:#04x}", orig),
        }
    }
}
