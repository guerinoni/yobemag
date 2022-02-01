use crate::memory_device::ReadWrite;

#[derive(Debug, Eq, PartialEq)]
pub enum HdmaMode {
    // When using this transfer method, all data is transferred at once. The execution of the program is halted until
    // the transfer has completed. Note that the General Purpose DMA blindly attempts to copy the data, even if the
    // CD controller is currently accessing VRAM. So General Purpose DMA should be used only if the Display is disabled,
    // or during V-Blank, or (for rather short blocks) during H-Blank. The execution of the program continues when the
    // transfer has been completed, and FF55 then contains a value of FFh.
    Gdma,
    // The H-Blank DMA transfers 10h bytes of data during each H-Blank, ie. at LY=0-143, no data is transferred during
    // V-Blank (LY=144-153), but the transfer will then continue at LY=00. The execution of the program is halted
    // during the separate transfers, but the program execution continues during the 'spaces' between each data block.
    // Note that the program should not change the Destination VRAM bank (FF4F), or the Source ROM/RAM bank (in case
    // data is transferred from bankable memory) until the transfer has completed! (The transfer should be paused as
    // described below while the banks are switched) Reading from Register FF55 returns the remaining length (divided
    // by 10h, minus 1), a value of 0FFh indicates that the transfer has completed. It is also possible to terminate
    // an active H-Blank transfer by writing zero to Bit 7 of FF55. In that case reading from FF55 will return how many
    // $10 "blocks" remained (minus 1) in the lower 7 bits, but Bit 7 will be read as "1". Stopping the transfer
    // doesn't set HDMA1-4 to $FF.
    Hdma,
}

pub struct Hdma {
    // These two registers specify the address at which the transfer will read data from. Normally, this should be
    // either in ROM, SRAM or WRAM, thus either in range 0x0000-0x7FF0 or 0xA000-0xDFF0. [Note : this has yet to be tested on
    // Echo RAM, OAM, FEXX, IO and HRAM]. Trying to specify a source address in VRAM will cause garbage to be copied.
    // The four lower bits of this address will be ignored and treated as 0.
    pub source: u16,
    // These two registers specify the address within 0x8000-0x9FF0 to which the data will be copied. Only bits 12-4 are
    // respected; others are ignored. The four lower bits of this address will be ignored and treated as 0.
    pub destination: u16,
    pub active: bool,
    pub mode: HdmaMode,
    pub remain: u8,
}

impl Default for Hdma {
    fn default() -> Self {
        Self {
            source: 0,
            destination: 0x8000,
            active: false,
            mode: HdmaMode::Gdma,
            remain: 0,
        }
    }
}

impl ReadWrite for Hdma {
    fn contains(&self, address: usize) -> bool {
        address == 0xFF51
            || address == 0xFF52
            || address == 0xFF53
            || address == 0xFF54
            || address == 0xFF55
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xFF51 => Ok((self.source >> 8) as u8),
            0xFF52 => Ok(self.source as u8),
            0xFF53 => Ok((self.destination >> 8) as u8),
            0xFF54 => Ok(self.destination as u8),
            0xFF55 => Ok(self.remain | if self.active { 0x00 } else { 0x80 }),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    format!(
                        "hdma don't know where read byte for this address {:#04x}",
                        address
                    ),
                ))
            }
        }
    }

    fn read_word(&self, _address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xFF51 => self.source = (u16::from(value) << 8) | (self.source & 0x00FF),
            0xFF52 => self.source = (self.source & 0xFF00) | u16::from(value & 0xF0),
            0xFF53 => {
                self.destination =
                    0x8000 | (u16::from(value & 0x1F) << 8) | (self.destination & 0x00FF)
            }
            0xFF54 => self.destination = (self.destination & 0xFF00) | u16::from(value & 0xF0),
            0xFF55 => {
                if self.active && self.mode == HdmaMode::Hdma {
                    if value & 0x80 == 0x00 {
                        self.active = false;
                    };

                    return Ok(());
                }
                self.active = true;
                self.remain = value & 0x7F;
                self.mode = if value & 0x80 != 0x00 {
                    HdmaMode::Hdma
                } else {
                    HdmaMode::Gdma
                };
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    format!(
                        "hdma don't know where write byte for this address {:#04x}",
                        address
                    ),
                ))
            }
        }

        Ok(())
    }
    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_default() {
        let hdma = Hdma::default();
        assert_eq!(hdma.source, 0);
        assert_eq!(hdma.destination, 0x8000);
        assert!(!hdma.active);
        assert_eq!(hdma.remain, 0);
        assert_eq!(hdma.mode, HdmaMode::Gdma);
    }

    #[test]
    fn verify_ff51() {
        let mut hdma = Hdma::default();
        let r = hdma.write_byte(0xFF51, 200);
        assert!(r.is_ok());
        assert_eq!(hdma.read_byte(0xFF51).unwrap(), 200);
    }

    #[test]
    fn verify_ff52() {
        let mut hdma = Hdma::default();
        let r = hdma.write_byte(0xFF52, 200);
        assert!(r.is_ok());
        assert_eq!(hdma.source, 192);
        assert_eq!(hdma.read_byte(0xFF52).unwrap(), 192);
    }

    #[test]
    fn verify_ff53() {
        let mut hdma = Hdma::default();
        let r = hdma.write_byte(0xFF53, 200);
        assert!(r.is_ok());
        assert_eq!(hdma.destination, 34816);
        assert_eq!(hdma.read_byte(0xFF53).unwrap(), 136);
    }
}
