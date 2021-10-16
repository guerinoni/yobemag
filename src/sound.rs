use crate::memory_device::ReadWrite;

pub struct Sound {
    /// On/Off sound 0xFF26.
    /// Bit 7 - All sound on/off  (0: stop all sound circuits) (Read/Write)
    /// Bit 3 - Sound 4 ON flag (Read Only)
    /// Bit 2 - Sound 3 ON flag (Read Only)
    /// Bit 1 - Sound 2 ON flag (Read Only)
    /// Bit 0 - Sound 1 ON flag (Read Only)
    on: u8, // TODO: create dedicated struct for better reading code.
}

impl Sound {
    pub fn new() -> Sound {
        Sound { on: 0 }
    }
}

impl ReadWrite for Sound {
    fn contains(self: &Self, address: usize) -> bool {
        0xFF26 == address
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF26 => Ok(self.on),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't read byte here",
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        println!("set sound on/off -> {}", value);
        match address {
            0xFF26 => Ok(self.on = value),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
