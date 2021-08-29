use std::num::Wrapping;

pub struct CartridgeHeader {
    title: String,
    memory_bank_type: MemoryBankType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryBankType {
    NoMemoryBank,
    MBC1,
    MBC2,
    MMM01,
    MBC3,
    MBC4,
    MBC5,
}

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// original games have all nintengo logo bytes insiede its cartridge.
fn check_logo(data: &[u8]) -> Result<(), std::io::Error> {
    match data[0x104..0x134].iter().cmp(NINTENDO_LOGO.iter()) {
        std::cmp::Ordering::Equal => Ok(()),
        std::cmp::Ordering::Less | std::cmp::Ordering::Greater => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "logo bytes are corrupted.",
        )),
    }
}

/// since gameboy check for non original games when loading cartridge.
pub fn valid_checksum(data: &[u8]) -> Result<(), std::io::Error> {
    let checksum: Wrapping<u8> = data[0x134..0x14D]
        .iter()
        .cloned()
        .fold(Wrapping(0), |acc, v| acc - Wrapping(v) - Wrapping(1));

    dbg!(checksum);
    if checksum.0 != data[0x14D] {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "checksum not valid.",
        ));
    }

    Ok(())
}

fn decode_memory_bank_type(data: &[u8]) -> MemoryBankType {
    match data[0x147] {
        0x00 | 0x08..=0x09 => MemoryBankType::NoMemoryBank,
        0x01..=0x03 => MemoryBankType::MBC1,
        0x05..=0x06 => MemoryBankType::MBC2,
        0x0B..=0x0D => MemoryBankType::MMM01,
        0x0F..=0x13 => MemoryBankType::MBC3,
        0x15..=0x17 => MemoryBankType::MBC4,
        0x19..=0x1E => MemoryBankType::MBC5,
        _ => panic!("unknown memory bank type"),
    }
}

impl CartridgeHeader {
    pub fn new(data: &[u8]) -> Result<Self, std::io::Error> {
        let t = data[0x134..0x144]
            .iter()
            .take_while(|&&v| v > 0)
            .copied()
            .collect::<Vec<_>>();

        check_logo(data)?;
        valid_checksum(data)?;

        Ok(CartridgeHeader {
            title: String::from_utf8(t).unwrap(),
            memory_bank_type: decode_memory_bank_type(data),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::cartridge_header::*;

    #[test]
    fn verify_title() {
        let data = fs::read("./roms/Tetris.gb");
        assert_eq!(data.is_err(), false);
        let header = CartridgeHeader::new(&data.unwrap());
        assert_eq!(header.is_err(), false);
        assert_eq!(header.unwrap().title, "TETRIS");
    }

    #[test]
    fn verify_nintendo_logo() {
        let data = fs::read("./roms/Tetris.gb");
        assert_eq!(data.is_err(), false);
        let header = CartridgeHeader::new(&data.unwrap());
        assert_eq!(header.is_err(), false);
    }

    #[test]
    fn verify_checksum() {
        let data = fs::read("./roms/Tetris.gb");
        assert_eq!(data.is_err(), false);
        let header = CartridgeHeader::new(&data.unwrap());
        assert_eq!(header.is_err(), false);
    }

    #[test]
    fn verify_memory_bank_type() {
        let data = fs::read("./roms/Tetris.gb");
        assert_eq!(data.is_err(), false);
        let header = CartridgeHeader::new(&data.unwrap());
        assert_eq!(header.is_err(), false);
        assert_eq!(
            header.unwrap().memory_bank_type,
            MemoryBankType::NoMemoryBank
        );
    }
}
