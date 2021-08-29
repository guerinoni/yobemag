use crate::register::{self, Registers};

pub struct CPU {
    registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }
}
