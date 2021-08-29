use crate::cartridge_header::*;
use std::fs;

/// Cartridge holds all bytes of rom loaded form file.
pub struct Cartridge {
    header: CartridgeHeader,
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn from_path(filename: &str) -> Result<Cartridge, std::io::Error> {
        let data = fs::read(filename)?;
        Ok(Cartridge {
            header: CartridgeHeader::new(&data)?,
            rom: data,
        })
    }
}
