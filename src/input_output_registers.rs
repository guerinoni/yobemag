use crate::memory_device::ReadWrite;

pub struct InputOutputRegisters {
    /// Mask that holds input comes from gameboy button: 0xFF00.
    /// Bit 0 - P10 Input Right or Button A (0=Pressed) (Read Only)
    /// Bit 1 - P11 Input Left  or Button B (0=Pressed) (Read Only)
    /// Bit 2 - P12 Input Up    or Select   (0=Pressed) (Read Only)
    /// Bit 3 - P13 Input Down  or Start    (0=Pressed) (Read Only)
    /// Bit 4 - P14 Select Direction Keys   (0=Select)
    /// Bit 5 - P15 Select Button Keys      (0=Select)
    /// Bit 6 and 7 unused.
    /// TODO: protect write on read-only register.
    buttons: u8,
}

impl InputOutputRegisters {
    pub fn new() -> InputOutputRegisters {
        InputOutputRegisters { buttons: 0 }
    }
}

impl ReadWrite for InputOutputRegisters {
    fn contains(self: &Self, address: usize) -> bool {
        0xFF00 == address
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF00 => Ok(self.buttons),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't read byte here",
            )),
        }
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
        todo!("implement this func")
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xFF00 => Ok(self.buttons = value),
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
