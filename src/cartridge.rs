use crate::cartridge_header::*;
use crate::memory_device::*;
use std::fs;

pub struct NoMBCartridge {
    header: CartridgeHeader,
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl NoMBCartridge {
    fn new(rom: Vec<u8>, header: CartridgeHeader) -> NoMBCartridge {
        let mut ram = Vec::new();
        ram.resize(rom.len(), 0);
        NoMBCartridge { header, rom, ram }
    }
}

impl ReadWrite for NoMBCartridge {
    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0x0000..=0x7FFF => Ok(self.rom[address]),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "can't read byte for NoMBCartridge over 0x7FFF.",
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        match address {
            0x0000..=0x7FFF => {
                let low = self.rom[address] as u16;
                let high = self.rom[address + 1] as u16;
                Ok(high << 8 | low)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "can't read byte for NoMBCartridge over 0x7FFF.",
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
