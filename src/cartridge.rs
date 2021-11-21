use crate::cartridge_header::*;
use crate::memory_device::*;
use std::fs;

#[allow(dead_code)]
pub struct NoMBCartridge {
    header: CartridgeHeader,

    /// 0x0150-0x3FFF
    rom: Vec<u8>,
}

impl NoMBCartridge {
    fn new(rom: Vec<u8>, header: CartridgeHeader) -> NoMBCartridge {
        NoMBCartridge { header, rom }
    }
}

impl ReadWrite for NoMBCartridge {
    fn contains(&self, address: usize) -> bool {
        (0x0000..=0x3FFF).contains(&address)
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        Ok(self.rom[address])
    }

    fn read_word(&self, address: usize) -> Result<u16, std::io::Error> {
        Ok(u16::from_le_bytes([
            self.rom[address],
            self.rom[address + 1],
        ]))
    }

    fn write_byte(&mut self, _address: usize, _value: u8) -> Result<(), std::io::Error> {
        unimplemented!()
    }

    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
#[allow(dead_code)]
pub struct MBC1 {
    header: CartridgeHeader,

    /// 0x0150-0x3FFF
    rom: Vec<u8>,
    ram: Vec<u8>,

    /// 0x0000-0x1FFF: RAM Enable (write only lower 4 bits)
    ///  - 00: Disable RAM (default)
    ///  - 0A: Enable RAM
    ram_enable: bool,

    /// 0x6000-0x7FFF: ROM/RAM Mode Select (write only)
    /// Selects whether the above register should be used as the upper 2 bits
    /// of the ROM Bank Number or as the RAM Bank Number.
    ///  - 00 = ROM Banking Mode (up to 8KB RAM, 2MB ROM) (default)
    ///  - 01 = RAM Banking Mode (up to 32KB RAM, 512KB ROM)
    romram_mode: bool,

    /// 0x2000-0x3FFF: ROM Bank Number (write only)
    /// Selects the lower 5 bits of the ROM Bank Number (in range 01-1F)
    rombank: usize,

    /// 0x4000-0x5FFF: RAM Bank Number / Upper Bits of ROM Bank Number (write only)
    /// Selects the 2-bit RAM Bank Number (in range 00-03) or the upper 2 bits
    /// of the ROM Bank Number, depending on the ROM/RAM Mode Select.
    rambank: usize,
}

impl MBC1 {
    fn new(rom: Vec<u8>, header: CartridgeHeader) -> MBC1 {
        let ram_size = header.ram_in_bytes();
        MBC1 {
            header,
            rom,
            ram: Vec::with_capacity(ram_size),
            ram_enable: false,
            romram_mode: false,
            rombank: 0,
            rambank: 0,
        }
    }
}

impl ReadWrite for MBC1 {
    fn contains(&self, address: usize) -> bool {
        (0x0000..=0x3FFF).contains(&address)
            || (0x4000..=0x7FFF).contains(&address)
            || (0xA000..=0xBFFF).contains(&address)
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0x0000..=0x3FFF => Ok(self.rom[address]),
            0x4000..=0x5FFF => {
                let rom_bank_number = if self.romram_mode {
                    (self.rambank << 0x5)  | self.rombank
                } else {
                    self.rombank
                };

                Ok(self.rom[address - 0x4000 + 0x4000 * rom_bank_number])
            },
            _ => unimplemented!(),
        }
    }

    fn read_word(&self, address: usize) -> Result<u16, std::io::Error> {
        let low = self.rom[address] as u16;
        let high = self.rom[address + 1] as u16;
        Ok(high << 8 | low)
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            (0x2000..=0x3FFF) => {
                self.rombank = if value == 0x00 {
                    1
                } else {
                    (value & 0x1F) as usize
                }
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}

pub fn make_cartridge(filename: &str) -> Result<Box<dyn ReadWrite>, std::io::Error> {
    let data = fs::read(filename)?;
    let header = CartridgeHeader::new(&data)?;
    match header.memory_bank_type {
        MemoryBankType::NoMemoryBank => Ok(Box::new(NoMBCartridge::new(data, header))),
        MemoryBankType::MBC1 => Ok(Box::new(MBC1::new(data, header))),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no implementation for this memory bank type.",
        )),
    }
}
