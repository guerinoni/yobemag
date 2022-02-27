use crate::memory_device::ReadWrite;

// BCPS/BGPI - CGB Mode Only - Background Palette Index
// This register is used to address a byte in the CGBs Background Palette Memory.
// Each two byte in that memory define a color value.
// The first 8 bytes define Color 0-3 of Palette 0 (BGP0), and so on for BGP1-7.
//  Bit 0-5   Index (00-3F)
//  Bit 7     Auto Increment  (0=Disabled, 1=Increment after Writing)
// Data can be read/written to/from the specified index address through Register 0xFF69.
// When the Auto Increment bit is set then the index is automatically incremented after each <write> to 0xFF69.
// Auto Increment has no effect when <reading> from 0xFF69,
// so the index must be manually incremented in that case.
// Writing to 0xFF69 during rendering still causes auto-increment to occur.
// Unlike the following, this register can be accessed outside V-Blank and H-Blank.
#[derive(Default)]
pub(crate) struct BackgroundPaletteIndex {
    value: u8,
    auto_increment: bool,
}

impl BackgroundPaletteIndex {
    /// Get the background palette index's value.
    pub(crate) fn value(&self) -> u8 {
        self.value
    }
}

impl ReadWrite for BackgroundPaletteIndex {
    fn contains(&self, address: usize) -> bool {
        address == 0xFF68
    }

    fn read_byte(&self, _address: usize) -> Result<u8, std::io::Error> {
        let a = if self.auto_increment { 0x80 } else { 0x00 };
        Ok(a | self.value)
    }

    fn read_word(&self, address: usize) -> Result<u16, std::io::Error> {
        Err(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            format!("can't read word in {:#04x}.", address),
        ))
    }

    fn write_byte(&mut self, _address: usize, value: u8) -> Result<(), std::io::Error> {
        self.auto_increment = value & 0x80 != 0x00;
        self.value = value & 0x3F;
        Ok(())
    }

    fn write_word(&mut self, address: usize, _value: u16) -> Result<(), std::io::Error> {
        Err(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            format!("can't read word in {:#04x}.", address),
        ))
    }
}
