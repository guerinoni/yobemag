use crate::memory_device::*;

/// Holds all memory space addressable for emulation.
/// This contains
pub struct MMU {
    cartridge: Box<dyn ReadWrite>,
}

impl MMU {
    pub fn new(cartridge: Box<dyn ReadWrite>) -> MMU {
        MMU { cartridge }
    }
}

pub const ROM_START_ADDRESS: usize = 0x0000;
pub const ROM_END_ADDRESS: usize = 0x7FFF;

impl ReadWrite for MMU {
    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            ROM_START_ADDRESS..=ROM_END_ADDRESS => self.cartridge.read_byte(address),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                format!(
                    "MMU don't know where read byte for this address {:#04x}",
                    address
                ),
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        match address {
            ROM_START_ADDRESS..=ROM_END_ADDRESS => self.cartridge.read_word(address),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                format!(
                    "MMU don't know where read word for this address {:#04x}",
                    address
                ),
            )),
        }
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
        Ok(())
    }
}
