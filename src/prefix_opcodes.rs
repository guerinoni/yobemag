#[derive(Debug, PartialEq, Eq)]
pub enum PrefixOpCode {
    // / RLC r
// / The contents of the register r are rotated left by 1 bit position, after the sign bit (7)
// / is copied into the carry flag.
// / Clock cycles: 8
// RlcB,
// RlcC,
// RlcD,
// RlcE,
// RlcH,
// RlcL,
// RlcA,
}

impl From<u8> for PrefixOpCode {
    fn from(orig: u8) -> Self {
        match orig {
            // 0x0 => PrefixOpCode::RlcB,
            // 0x1 => PrefixOpCode::RlcC,
            // 0x2 => PrefixOpCode::RlcD,
            // 0x3 => PrefixOpCode::RlcE,
            // 0x4 => PrefixOpCode::RlcH,
            // 0x5 => PrefixOpCode::RlcL,
            // 0x7 => PrefixOpCode::RlcA,
            _ => panic!("unknown prefix opcode"),
        }
    }
}
