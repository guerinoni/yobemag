use crate::gpu::GraphicsProcessingUnit;
use crate::hdma::{Hdma, HdmaMode};
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

    hdma: Hdma,
}

impl MemoryManagmentUnit {
    pub fn new(cartridge: Box<dyn ReadWrite>) -> MemoryManagmentUnit {
        MemoryManagmentUnit {
            cartridge,
            gpu: GraphicsProcessingUnit::new(),
            internal: InternalMemory::new(),
            serial: SerialDataTransfer::default(),
            timer: Timer::default(),
            sound: Sound::new(),
            speed: Speed::Normal,
            toggle_speed_request: false,
            io_reg: InputOutputRegisters::new(),
            hdma: Hdma::default(),
        }
    }

    pub fn step(&mut self, cycles: u32) {
        let cpu_divider = self.speed as u32;
        let vram_cycles = self.run_dma();
        let gpu_cycles = cycles / cpu_divider + vram_cycles;
        let cpu_cycles = cycles + vram_cycles * cpu_divider;
        self.timer.step(cpu_cycles);
        self.gpu.step(gpu_cycles);

        self.gpu.step(cycles);

        // println!("{}", cycles);
    }

    //     run_dma_hrampart:
    //     ldh ($FF00+c), a
    //    wait:
    //     dec b
    //     jr nz,wait
    //     ret
    fn run_dma_hrampart(&mut self) {
        let mmu_src = self.hdma.source;
        for i in 0..0x10 {
            let b: u8 = self.read_byte((mmu_src + i) as usize).unwrap();
            self.gpu
                .write_byte((self.hdma.destination + i) as usize, b)
                .unwrap();
        }

        self.hdma.source += 0x10;
        self.hdma.destination += 0x10;
        self.hdma.update_remain_after_hrampart();
    }

    // Writing to this register launches a DMA transfer from ROM or RAM to OAM memory (sprite attribute table).
    // The written value specifies the transfer source address divided by 100h, ie. source & destination are:
    //  Source:      XX00-XX9F   ;XX in range from 00-F1h
    //  Destination: 0xFE00-0xFE9F
    // The transfer takes 160 machine cycles: 152 microseconds in normal speed or 76 microseconds in CGB Double Speed Mode.
    // On DMG, during this time, the CPU can access only HRAM (memory at FF80-FFFE); on CGB, the bus used by the source area cannot be used (this isn't understood well at the moment, it's recommended to assume same behavior as DMG). For this reason, the programmer must copy a short procedure into HRAM,
    // and use this procedure to start the transfer from inside HRAM, and wait until the transfer has finished:
    // run_dma:  ; This part is in ROM
    // ld a, start address / 100h
    // ld bc, 2946h  ; B: wait time; C: OAM trigger
    // jp run_dma_hrampart
    fn run_dma(&mut self) -> u32 {
        if !self.hdma.is_active() {
            return 0;
        }

        match self.hdma.mode {
            HdmaMode::Gdma => {
                let len = u32::from(self.hdma.remain) + 1;
                for _ in 0..len {
                    self.run_dma_hrampart();
                }
                self.hdma.set_active(false);
                len * 8
            }
            HdmaMode::Hdma => {
                if !self.gpu.h_blank {
                    return 0;
                }
                self.run_dma_hrampart();
                if self.hdma.remain == 0x7F {
                    self.hdma.set_active(false);
                }

                8
            }
        }
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

        if self.hdma.contains(address) {
            return self.hdma.read_byte(address);
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

        if self.hdma.contains(address) {
            return self.hdma.read_word(address);
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

        if self.hdma.contains(address) {
            return self.hdma.write_byte(address, value);
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
        if self.gpu.contains(address) {
            return self.gpu.write_word(address, value);
        }

        if self.cartridge.contains(address) {
            return self.cartridge.write_word(address, value);
        }

        if self.internal.contains(address) {
            return self.internal.write_word(address, value);
        }

        if self.serial.contains(address) {
            return self.serial.write_word(address, value);
        }

        if self.timer.contains(address) {
            return self.timer.write_word(address, value);
        }

        if self.sound.contains(address) {
            return self.sound.write_word(address, value);
        }

        if self.hdma.contains(address) {
            return self.hdma.write_word(address, value);
        }

        unimplemented!()
    }
}
