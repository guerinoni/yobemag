use crate::{cartridge::make_cartridge, cpu::CentralProcessingUnit};

pub struct Emulator {
    cpu: CentralProcessingUnit,
}

impl Emulator {
    pub fn new(filename: &str) -> Result<Emulator, std::io::Error> {
        let device = make_cartridge(filename)?;
        Ok(Emulator {
            cpu: CentralProcessingUnit::new(device),
        })
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}
