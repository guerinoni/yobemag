use crate::cartridge_header::*;
use crate::memory_device::*;
use std::fs;

pub struct NoMBCartridge {
    header: CartridgeHeader,
    rom: Vec<u8>,
}

impl NoMBCartridge {
    fn new(rom: Vec<u8>, header: CartridgeHeader) -> NoMBCartridge {
        NoMBCartridge { header, rom }
    }
}

impl ReadWrite for NoMBCartridge {
    fn read_byte(self: &Self, address: u16) -> Result<u8, std::io::Error> {
        Ok(0)
    }

    fn read_word(self: &Self, address: u16) -> Result<u8, std::io::Error> {
        Ok(0)
    }

    fn write_byte(self: &Self, address: u16, value: u8) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn write_word(self: &Self, address: u16, value: u16) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub fn make_cartridge(filename: &str) -> Result<Box<dyn ReadWrite>, std::io::Error> {
    let data = fs::read(filename)?;
    let header = CartridgeHeader::new(&data)?;
    match header.memory_bank_type {
        MemoryBankType::NoMemoryBank => Ok(Box::new(NoMBCartridge::new(data, header))),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no implementation for this memory bank type.",
        )),
    }
}
