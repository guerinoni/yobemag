use crate::memory_device::ReadWrite;

pub struct GPU {
    /// video ram: 0x8000-0x9FFF
    vram: [u8; 0x1FFF + 1],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; 0x1FFF + 1],
        }
    }
}

impl ReadWrite for GPU {
    fn contains(self: &Self, address: usize) -> bool {
        (0x8000..=0x1FFF).contains(&address)
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        Ok(self.vram[address - 0x8000])
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        todo!("implement this func")
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0x8000..=0x9FFF => Ok(self.vram[address - 0x8000] = value),
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
