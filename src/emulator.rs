use crate::cartridge::Cartridge;

pub struct Emulator {
    cartridge: Cartridge,
}

impl Emulator {
    pub fn new(filename: &str) -> Result<Emulator, std::io::Error> {
        Ok(Emulator {
            cartridge: Cartridge::from_path(filename)?,
        })
    }
}
