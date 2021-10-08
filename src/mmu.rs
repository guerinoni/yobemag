use crate::gpu::GPU;
use crate::internal_memory::InternalMemory;
use crate::memory_device::ReadWrite;
use crate::serial_data_transfer::SerialDataTransfer;
use crate::timer::Timer;

/// Holds all memory space addressable for emulation.
pub struct MMU {
    cartridge: Box<dyn ReadWrite>,
    gpu: GPU,
    internal: InternalMemory,
    serial: SerialDataTransfer,
    timer: Timer,
}

impl MMU {
    pub fn new(cartridge: Box<dyn ReadWrite>) -> MMU {
        MMU {
            cartridge,
            gpu: GPU::new(),
            internal: InternalMemory::new(),
            serial: SerialDataTransfer::new(),
            timer: Timer::new(),
        }
    }
}

impl ReadWrite for MMU {
    fn contains(self: &Self, address: usize) -> bool {
        unimplemented!() // FIXME: maybe this should be refactored in better API
    }

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
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

        Err(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            format!(
                "MMU don't know where read byte for this address {:#04x}",
                address
            ),
        ))
    }

    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
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

        Err(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            format!(
                "MMU don't know where read word for this address {:#04x}",
                address
            ),
        ))
    }

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
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

        if (0xFF24..=0xFF27).contains(&address) {
            // NOTE: seems unused
            return Ok(());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            format!(
                "MMU don't know where write byte for this address {:#04x}",
                address
            ),
        ))
    }

    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
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
