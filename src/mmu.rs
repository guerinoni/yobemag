use crate::gpu::GraphicsProcessingUnit;
use crate::internal_memory::InternalMemory;
use crate::memory_device::ReadWrite;
use crate::serial_data_transfer::SerialDataTransfer;
use crate::sound::Sound;
use crate::timer::Timer;

// Holds all memory space addressable for emulation.
pub struct MemoryManagmentUnit {
    cartridge: Box<dyn ReadWrite>,
    gpu: GraphicsProcessingUnit,
    internal: InternalMemory,
    serial: SerialDataTransfer,
    timer: Timer,
    sound: Sound,
}

impl MemoryManagmentUnit {
    pub fn new(cartridge: Box<dyn ReadWrite>) -> MemoryManagmentUnit {
        MemoryManagmentUnit {
            cartridge,
            gpu: GraphicsProcessingUnit::new(),
            internal: InternalMemory::new(),
            serial: SerialDataTransfer::new(),
            timer: Timer::new(),
            sound: Sound::new(),
        }
    }

    pub fn step(&mut self, cycles: u8) {
        println!("{}", cycles);
    }
}

impl ReadWrite for MemoryManagmentUnit {
    fn contains(&self, _address: usize) -> bool {
        unimplemented!() // FIXME: maybe this should be refactored in better API
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        if self.gpu.contains(address) {
            // TODO: refactor this generic func in array of memory devices I think or somethig generic
            return self.gpu.read_byte(address);
        }

        if self.cartridge.contains(address) {
            return self.cartridge.read_byte(address);
        }

        if self.internal.contains(address) {
            return self.internal.read_byte(address);
        }

        if self.serial.contains(address) {
            return self.serial.read_byte(address);
        }

        if self.timer.contains(address) {
            return self.timer.read_byte(address);
        }

        if self.sound.contains(address) {
            return self.sound.read_byte(address);
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            format!(
                "MMU don't know where read byte for this address {:#04x}",
                address
            ),
        ))
    }

    fn read_word(&self, address: usize) -> Result<u16, std::io::Error> {
        if self.gpu.contains(address) {
            // TODO: refactor this generic func in array of memory devices I think or somethig generic
            return self.gpu.read_word(address);
        }

        if self.cartridge.contains(address) {
            return self.cartridge.read_word(address);
        }

        if self.internal.contains(address) {
            return self.internal.read_word(address);
        }

        if self.serial.contains(address) {
            return self.serial.read_word(address);
        }

        if self.timer.contains(address) {
            return self.timer.read_word(address);
        }

        if self.sound.contains(address) {
            return self.sound.read_word(address);
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            format!(
                "MMU don't know where read word for this address {:#04x}",
                address
            ),
        ))
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        // TODO: refactor this generic func in array of memory devices I think or somethig generic
        if self.gpu.contains(address) {
            return self.gpu.write_byte(address, value);
        }

        if self.cartridge.contains(address) {
            return self.cartridge.write_byte(address, value);
        }

        if self.internal.contains(address) {
            return self.internal.write_byte(address, value);
        }

        if self.serial.contains(address) {
            return self.serial.write_byte(address, value);
        }

        if self.timer.contains(address) {
            return self.timer.write_byte(address, value);
        }

        if self.sound.contains(address) {
            return self.sound.write_byte(address, value);
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            format!(
                "MMU don't know where write byte for this address {:#04x}",
                address
            ),
        ))
    }

    fn write_word(&mut self, address: usize, value: u16) -> Result<(), std::io::Error> {
        match self.write_byte(address, (value & 0xFF) as u8) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        match self.write_byte(address + 1, (value >> 8) as u8) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        Ok(())
    }
}
