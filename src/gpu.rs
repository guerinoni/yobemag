use crate::memory_device::ReadWrite;

pub struct GPU {
    /// video ram: 0x8000-0x9FFF
    vram: [u8; 0x1FFF + 1],
    /// Specifies the position in the 256x256 pixels BG map (32x32 tiles) which is to be displayed at the upper/left LCD display position.
    /// scroll Y 0xFF42 (read-write)
    scroll_y: u8,
    /// scroll X 0xFF43 (read-write)
    scroll_x: u8,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; 0x1FFF + 1],
            scroll_y: 0,
            scroll_x: 0,
        }
    }
}

impl ReadWrite for GPU {
    fn contains(self: &Self, address: usize) -> bool {
        (0x8000..=0x1FFF).contains(&address) || 0xFF42 == address || 0xFF43 == address
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0x8000..=0x9FFF => Ok(self.vram[address - 0x8000]),
            0xFF42 => Ok(self.scroll_y),
            0xFF43 => Ok(self.scroll_x),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        todo!("implement this func")
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0x8000..=0x9FFF => Ok(self.vram[address - 0x8000] = value),
            0xFF42 => Ok(self.scroll_y = value),
            0xFF43 => Ok(self.scroll_x = value),
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
