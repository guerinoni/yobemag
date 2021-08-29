#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Noop,
    Stop,
    Halt,
}

impl From<u8> for OpCode {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => OpCode::Noop,
            _ => panic!("unknown ram size"),
        }
    }
}