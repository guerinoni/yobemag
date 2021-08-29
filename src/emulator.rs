use crate::{cartridge::make_cartridge, cpu::CPU};

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    pub fn new(filename: &str) -> Result<Emulator, std::io::Error> {
        let device = make_cartridge(filename)?;
        Ok(Emulator {
            cpu: CPU::new(device),
        })
    }

    pub fn step(self: &Self) {
        self.cpu.step();
    }
}
