use crate::cartridge_header::*;
use crate::memory_device::*;
use std::fs;

#[allow(dead_code)]
pub struct NoMBCartridge {
    header: CartridgeHeader,

    // 0x0150-0x3FFF
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

    // 0x0150-0x3FFF
    rom: Vec<u8>,
    ram: Vec<u8>,

    // 0x0000-0x1FFF: RAM Enable (write only lower 4 bits)
    //  - 00: Disable RAM (default)
    //  - 0A: Enable RAM
    ram_enable: bool,

    // 0x6000-0x7FFF: ROM/RAM Mode Select (write only)
    // Selects whether the above register should be used as the upper 2 bits
    // of the ROM Bank Number or as the RAM Bank Number.
    //  - 00 = ROM Banking Mode (up to 8KB RAM, 2MB ROM) (default)
    //  - 01 = RAM Banking Mode (up to 32KB RAM, 512KB ROM)
    romram_mode: bool,
    bank: u8,
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
            bank: 0,
        }
    }

    fn rom_bank(&self) -> u8 {
        if self.romram_mode {
            self.bank & 0x7F
        } else {
            self.bank & 0x1F
        }
    }

    fn ram_bank(&self) -> u8 {
        if self.romram_mode {
            0
        } else {
            (self.bank & 0x60) >> 5
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
            0x4000..=0x7FFF => {
                let i = self.rom_bank() as usize * 0x4000_usize + address - 0x4000_usize;
                Ok(self.rom[i])
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let i = self.ram_bank() as usize * 0x2000_usize + address - 0xA000_usize;
                    Ok(self.ram[i])
                } else {
                    Ok(0)
                }
            }
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
            0x0000..=0x1FFF => self.ram_enable = value & 0x0F == 0x0A,
            0x2000..=0x3FFF => {
                self.bank = if value == 0x00 {
                    1
                } else {
                    ((value & 0x1F) as usize).try_into().unwrap()
                }
            }
            0x4000..=0x5fff => self.bank = self.bank & 0x9F | (value & 0x03 << 5),
            0x6000..=0x7fff => {
                if value == 0 {
                    self.romram_mode = false;
                } else if value == 1 {
                    self.romram_mode = true;
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid cartridge type.",
                    ));
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    let i =
                        self.ram_bank() as usize * 0x2000_usize + address as usize - 0xA000_usize;
                    self.ram[i] = value;
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
