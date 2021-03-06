use crate::{cartridge::make_cartridge, cpu::CentralProcessingUnit, mmu::MemoryManagmentUnit};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Emulator {
    mmu: Rc<RefCell<MemoryManagmentUnit>>,
    cpu: CentralProcessingUnit,
}

impl Emulator {
    pub fn new(filename: &str) -> Result<Emulator, std::io::Error> {
        let device = make_cartridge(filename)?;
        let mmu = Rc::new(RefCell::new(MemoryManagmentUnit::new(device)));
        let cpu = CentralProcessingUnit::new(mmu.clone());
        Ok(Emulator { mmu, cpu })
    }

    pub fn step(&mut self) {
        if self.cpu.need_toggle_speed() {
            self.mmu.borrow_mut().toggle_speed();
        }

        let clock_cycles = self.cpu.step();
        self.mmu.borrow_mut().step(clock_cycles);
    }
}
