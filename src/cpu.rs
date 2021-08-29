use crate::{memory_device::ReadWrite, register::Registers};

pub struct CPU {
    registers: Registers,
    device: Box<dyn ReadWrite>,
}

impl CPU {
    pub fn new(device: Box<dyn ReadWrite>) -> CPU {
        CPU {
            registers: Registers::new(),
            device,
        }
    }

    pub fn step(self: &Self) {}
}
