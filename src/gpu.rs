use std::fmt::format;

use crate::memory_device::ReadWrite;

pub struct GPU {
    /// video ram: 0x8000-0x9FFF
    vram: [u8; 0x1FFF + 1],
    /// working ram bank 0: 0xC000-0xCFFF
    wram0: [u8; 0x1FFF + 1],
    /// working ram bank n: 0xD000-0xDFFF
    wramn: [u8; 0x1FFF + 1],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; 0x1FFF + 1],
            wram0: [0; 0x1FFF + 1],
            wramn: [0; 0x1FFF + 1],
        }
    }
}

impl ReadWrite for GPU {
    fn contains(self: &Self, address: usize) -> bool {
        (0x8000..=0x1FFF).contains(&address)
            || (0xC000..=0xCFFF).contains(&address)
            || (0xD000..=0xDFFF).contains(&address)
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        Ok(self.vram[address])
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        todo!("implement this func")
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xC000..=0xCFFF => Ok(self.wram0[address - 0xC000] = value),
            0xD000..=0xDFFF => Ok(self.wramn[address - 0xD000] = value),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
        todo!("implement this func")
    }
}
