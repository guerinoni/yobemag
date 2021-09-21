use crate::memory_device::ReadWrite;

pub struct SerialDataTransfer {
    /// Before a transfer, it holds the next byte that will go out: 0xFF01
    data: u8,
}

impl SerialDataTransfer {
    pub fn new() -> SerialDataTransfer {
        SerialDataTransfer { data: 0 }
    }
}

impl ReadWrite for SerialDataTransfer {
    fn contains(self: &Self, address: usize) -> bool {
        0xFF01 == address
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF01 => Ok(self.data),
            _ => unimplemented!(),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xFF01 => Ok(self.data = value),
            _ => unimplemented!(),
        }
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
