use crate::memory_device::ReadWrite;

// Each time when the timer overflows (ie. when TIMA gets bigger than FFh), then an interrupt is requested by
// setting Bit 2 in the IF Register (0xFF0F). When that interrupt is enabled, then the CPU will execute it by calling
// the timer interrupt vector at 0050h.
#[derive(Default)]
pub struct Timer {
    // This register is incremented at rate of 16384Hz (~16779Hz on SGB).
    // Writing any value to this register resets it to 00h.
    // Note: The divider is affected by CGB double speed mode, and will increment at 32768Hz in double speed.
    divider: u8,
    // This timer is incremented by a clock frequency specified by the TAC register (0xFF07).
    // When the value overflows (gets bigger than FFh) then it will be reset to the value specified in TMA (0xFF06),
    // and an interrupt will be requested, as described below.
    tima: u8,
    // When the TIMA overflows, this data will be loaded.
    tma: u8,
    // FF07: Timer Control (TAC)
    //  - Bit 2: Timer Stop
    //    - 0: Stop
    //    - 1: Start
    //  - Bits 1-0: Input Clock Select (TIMA rate)
    //    - 00: CPU Clock / 1024 (DMG, CGB:   4096 Hz, SGB:   ~4194 Hz)
    //    - 01: CPU Clock / 16   (DMG, CGB: 262144 Hz, SGB: ~268400 Hz)
    //    - 10: CPU Clock / 64   (DMG, CGB:  65536 Hz, SGB:  ~67110 Hz)
    //    - 11: CPU Clock / 256  (DMG, CGB:  16384 Hz, SGB:  ~16780 Hz)
    tac: u8,
}

impl Timer {
    pub fn step(&mut self, cycles: u32) {
        let _lol = cycles;
        // self.divider = self.divider.wrapping_add(self.div_clock.next(cycles) as u8);
    }
}

impl ReadWrite for Timer {
    fn contains(&self, address: usize) -> bool {
        0xFF07 == address
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF04 => Ok(self.divider),
            0xFF05 => Ok(self.tima),
            0xFF06 => Ok(self.tma),
            0xFF07 => Ok(self.tac),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn read_word(&self, _address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xFF04 => self.divider = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "can't write byte here",
                ))
            }
        }
        Ok(())
    }

    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}
