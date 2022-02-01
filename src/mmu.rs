use crate::gpu::GraphicsProcessingUnit;
use crate::input_output_registers::InputOutputRegisters;
use crate::internal_memory::InternalMemory;
use crate::memory_device::ReadWrite;
use crate::serial_data_transfer::SerialDataTransfer;
use crate::sound::Sound;
use crate::timer::Timer;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Speed {
    Normal = 0x01,
    Double = 0x02,
}

// Holds all memory space addressable for emulation.
pub struct MemoryManagmentUnit {
    cartridge: Box<dyn ReadWrite>,
    gpu: GraphicsProcessingUnit,
    internal: InternalMemory,
    serial: SerialDataTransfer,
    timer: Timer,
    sound: Sound,

    // Bit 7: Current Speed     (0=Normal, 1=Double) (Read Only)
    // Bit 0: Prepare Speed Switch (0=No, 1=Prepare) (Read/Write)
    speed: Speed,
    toggle_speed_request: bool,

    // I/O registers, like joypad.
    io_reg: InputOutputRegisters,
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
            speed: Speed::Normal,
            toggle_speed_request: false,
            io_reg: InputOutputRegisters::new(),
        }
    }

    pub fn step(&mut self, cycles: u8) {
        let _cpu_divider = self.speed as u32;
        println!("{}", cycles);
    }

    pub fn toggle_speed(&mut self) {
        if self.toggle_speed_request {
            if self.speed == Speed::Double {
                self.speed = Speed::Normal;
            } else {
                self.speed = Speed::Double;
            }
        }

        self.toggle_speed_request = false;
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

        if self.io_reg.contains(address) {
            return self.io_reg.read_byte(address);
        }

        match address {
            0xFF4D => {
                let s = if self.speed == Speed::Double {
                    0x80
                } else {
                    0x00
                };
                let t = if self.toggle_speed_request {
                    0x01
                } else {
                    0x00
                };
                Ok(s | t)
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    format!(
                        "MMU don't know where read byte for this address {:#04x}",
                        address
                    ),
                ))
            }
        }
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

        if self.io_reg.contains(address) {
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

        match address {
            0xFF4D => self.toggle_speed_request = (value & 0x01) == 0x01,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    format!(
                        "MMU don't know where write byte for this address {:#04x}",
                        address
                    ),
                ))
            }
        }

        Ok(())
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
