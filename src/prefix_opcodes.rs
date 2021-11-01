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
            _ => panic!("unknown prefix opcode {:#04x}", orig),
        }
    }
}
