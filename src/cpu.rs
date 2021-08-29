use crate::{opcodes::*, memory_device::ReadWrite, register::Registers};

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

    pub fn step(self: &Self) {
        let op_code = self.fetch_byte();
        match op_code.into() {
            OpCode::Noop => println!(""),
            OpCode::Stop => println!(""),
            OpCode::Halt => println!(""),
        }
    }

    fn fetch_byte(self: &Self) -> u8 {
        self.device.read_byte(self.registers.program_counter as usize).unwrap()
    }
}
