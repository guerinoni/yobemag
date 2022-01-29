use crate::memory_device::ReadWrite;

/// InternalMemory holds all memory banks for internal handling of the emulating job, not GPU or
/// cartridge related, just internal stuff to read and write during execution.
pub struct InternalMemory {
    // working ram bank
    wram: [u8; 0x8000],
    wram_bank: u8,
    // high ram (zero-page): 0xFF80-0xFFFE
    hram: [u8; 0x007F],
    // interrupt flag (request) register: 0xFF0F
    // Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
    // Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
    // Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
    // Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
    // Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)
    interrupt_flag: u8,
    // interrupt flag enable: 0xFFFF
    interrupt_enable: u8,
}

impl InternalMemory {
    pub fn new() -> InternalMemory {
        InternalMemory {
            wram: [0; 0x8000],
            wram_bank: 1,
            hram: [0; 0x007F],
            interrupt_flag: 0,
            interrupt_enable: 0,
        }
    }
}

impl ReadWrite for InternalMemory {
    fn contains(&self, address: usize) -> bool {
        (0xC000..=0xCFFF).contains(&address)
            || (0xD000..=0xDFFF).contains(&address)
            || (0xE000..=0xEFFF).contains(&address)
            || (0xF000..=0xFDFF).contains(&address)
            || (0xFF80..=0xFFFE).contains(&address)
            || 0xFF0F == address
            || 0xFFFF == address
            || 0xFF70 == address
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0xC000..=0xCFFF => Ok(self.wram[address - 0xC000]),
            0xD000..=0xDFFF => Ok(self.wram[address - 0xD000 + 0x1000 * self.wram_bank as usize]),
            0xE000..=0xEFFF => Ok(self.wram[address - 0xE000]),
            0xF000..=0xFDFF => Ok(self.wram[address - 0xF000 + 0x1000 * self.wram_bank as usize]),
            0xFF80..=0xFFFE => Ok(self.hram[address - 0xFF80]),
            0xFF70 => Ok(self.wram_bank),
            0xFF0F => Ok(self.interrupt_flag),
            0xFFFF => Ok(self.interrupt_enable),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't read byte here",
            )),
        }
    }

    fn read_word(&self, address: usize) -> Result<u16, std::io::Error> {
        let low = self.read_byte(address)?;
        let high = self.read_byte(address + 1)?;
        Ok(u16::from(low) | (u16::from(high) << 8))
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0xC000..=0xCFFF => self.wram[address - 0xC000] = value,
            0xD000..=0xDFFF => {
                self.wram[address - 0xD000 + 0x1000 * self.wram_bank as usize] = value
            }
            0xE000..=0xEFFF => self.wram[address - 0xE000] = value,
            0xF000..=0xFDFF => {
                self.wram[address - 0xF000 + 0x1000 * self.wram_bank as usize] = value
            }
            0xFF80..=0xFFFE => self.hram[address - 0xFF80] = value,
            0xFF70 => {
                self.wram_bank = match value & 0x7 {
                    0 => 1,
                    n => n,
                }
            }
            0xFF0F => self.interrupt_flag = value,
            0xFFFF => self.interrupt_enable = value,
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
        todo!("implement this func")
    }
}
