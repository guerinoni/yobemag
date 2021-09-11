use std::fmt::format;

use crate::memory_device::ReadWrite;

pub struct GPU {
    /// VRAM: 8000-9FFF
    vram: [u8; 0x1FFF + 1],
}

impl ReadWrite for GPU {
    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0x8000..=0x9FFF => Ok(self.vram[address]),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                format!("can't read byte for the address {:#04x} from GPU.", address),
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        todo!("implement this func")
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        todo!("implement this func")
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
        todo!("implement this func")
    }
}
