use crate::memory_device::ReadWrite;

pub struct SerialDataTransfer {
    // Before a transfer, it holds the next byte that will go out: 0xFF01
    // During a transfer, it has a blend of the outgoing and incoming bytes.
    // Each cycle, the leftmost bit is shifted
    // out (and over the wire) and the incoming bit is shifted in from the other side.
    data: u8,

    // Bit 0 - Shift Clock (0=External Clock, 1=Internal Clock)
    // Bit 1 - Clock Speed (0=Normal, 1=Fast) ** CGB Mode Only **
    // Bit 7 - Transfer Start Flag (0=No transfer is in progress or requested, 1=Transfer in progress, or requested)
    control: u8,
}

impl Default for SerialDataTransfer {
    fn default() -> Self {
        Self {
            data: 0,
            control: 0,
        }
    }
}

impl ReadWrite for SerialDataTransfer {
    fn contains(&self, address: usize) -> bool {
        0xFF01 == address || 0xFF02 == address
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF01 => Ok(self.data),
            0xFF02 => Ok(self.control),
            _ => unimplemented!(),
        }
    }

    fn read_word(&self, _address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xFF01 => self.data = value,
            0xFF02 => self.control = value,
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
