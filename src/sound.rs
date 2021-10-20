use crate::memory_device::ReadWrite;

pub struct Sound {
    /// On/Off sound 0xFF26.
    /// Bit 7 - All sound on/off  (0: stop all sound circuits) (Read/Write)
    /// Bit 3 - Sound 4 ON flag (Read Only)
    /// Bit 2 - Sound 3 ON flag (Read Only)
    /// Bit 1 - Sound 2 ON flag (Read Only)
    /// Bit 0 - Sound 1 ON flag (Read Only)
    on: u8, // TODO: create dedicated struct for better reading code.

    /// Each channel can be panned hard left, center, or hard right 0xFF25.
    /// Bit 7 - Output sound 4 to SO2 terminal
    /// Bit 6 - Output sound 3 to SO2 terminal
    /// Bit 5 - Output sound 2 to SO2 terminal
    /// Bit 4 - Output sound 1 to SO2 terminal
    /// Bit 3 - Output sound 4 to SO1 terminal
    /// Bit 2 - Output sound 3 to SO1 terminal
    /// Bit 1 - Output sound 2 to SO1 terminal
    /// Bit 0 - Output sound 1 to SO1 terminal
    sound_output: u8,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
            on: 0,
            sound_output: 0,
        }
    }
}

impl ReadWrite for Sound {
    fn contains(&self, address: usize) -> bool {
        0xFF26 == address || 0xFF25 == address
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF26 => Ok(self.on),
            0xFF25 => Ok(self.sound_output),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't read byte here",
            )),
        }
    }

    fn read_word(&self, _address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        println!("set sound on/off -> {}", value);
        match address {
            0xFF26 => {
                self.on = value;
                Ok(())
            }
            0xFF25 => {
                self.sound_output = value;
                Ok(())
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
