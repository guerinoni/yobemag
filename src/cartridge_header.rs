use std::{fmt, num::Wrapping};

pub struct CartridgeHeader {
    // title: String,
    pub memory_bank_type: MemoryBankType,
    ram_size: RamSize,
    // gameboy_color_support: GameBoyColorFlag,
}

/// Specifies which Memory Bank Controller (if any) is used in
/// the cartridge, and if further external hardware exists
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

impl fmt::Display for MemoryBankType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryBankType::NoMemoryBank => write!(f, "NoMemoryBank"),
            MemoryBankType::MBC1 => write!(f, "MBC1"),
            MemoryBankType::MBC2 => write!(f, "MBC2"),
            MemoryBankType::MMM01 => write!(f, "MMM01"),
            MemoryBankType::MBC3 => write!(f, "MBC3"),
            MemoryBankType::MBC4 => write!(f, "MBC4"),
            MemoryBankType::MBC5 => write!(f, "MBC5"),
        }
    }
}

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// original games have all nintengo logo bytes inside its cartridge.
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

/// Specifies the size of the external RAM in the cartridge (if any).
enum RamSize {
    None,
    OneBankOf2Kb,
    OneBankOf8Kb,
    FourBankOf8Kb,
}

impl From<u8> for RamSize {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => RamSize::None,
            0x1 => RamSize::OneBankOf2Kb,
            0x2 => RamSize::OneBankOf8Kb,
            0x3 => RamSize::FourBankOf8Kb,
            _ => panic!("unknown ram size"),
        }
    }
}

/// In CGB cartridges the upper bit is used to enable CGB functions.
/// This is required, otherwise the CGB switches itself into Non-CGB-Mode.
///  - 80h: Game supports CGB functions, but works on old gameboys also.
///  - C0h: Game works on CGB only (physically the same as 80h).
/// Values with Bit 7 set, and either Bit 2 or 3 set, will switch the gameboy
/// into a special non-CGB-mode with uninitialized palettes. Purpose unknown,
/// eventually this has been supposed to be used to colorize monochrome games
/// that include fixed palette data at a special location in ROM.
#[derive(Debug, PartialEq, Eq)]
enum GameBoyColorFlag {
    /// Uses GB features only; default
    GB,
    /// Uses CGB features but works on GB
    CgbGb = 0x80,
    /// Uses CGB features and does not work on GB
    Cgb = 0xC0,
}

impl From<u8> for GameBoyColorFlag {
    fn from(orig: u8) -> Self {
        match orig {
            0x80 => GameBoyColorFlag::CgbGb,
            0xC0 => GameBoyColorFlag::Cgb,
            _ => GameBoyColorFlag::GB,
        }
    }
}

impl CartridgeHeader {
    pub fn new(data: &[u8]) -> Result<Self, std::io::Error> {
        check_logo(data)?;
        valid_checksum(data)?;

        Ok(CartridgeHeader {
            // title: String::from_utf8(t).unwrap(),
            memory_bank_type: decode_memory_bank_type(data),
            ram_size: data[0x149].into(),
            // gameboy_color_support: data[0x149].into(),
        })
    }

    pub fn ram_in_bytes(&self) -> usize {
        match self.ram_size {
            RamSize::None => 0,
            RamSize::OneBankOf2Kb => 2 * 1024,
            RamSize::OneBankOf8Kb => 8 * 1024,
            RamSize::FourBankOf8Kb => 4 * (8 * 1024),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::cartridge_header::*;

    #[test]
    fn verify_len() {
        let data = fs::read_to_string("./testdata/tetris")
            .expect("file not found!")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>();
        assert_eq!(32768, data.len());
        let header = CartridgeHeader::new(&data);
        assert_eq!(header.is_err(), false);
    }

    // FIXME: enable it when title is needed
    // #[test]
    // fn verify_title() {
    //     let data = fs::read_to_string("./testdata/tetris")
    //         .expect("file not found!")
    //         .split(',')
    //         .map(|n| n.parse().unwrap())
    //         .collect::<Vec<u8>>();
    //     let header = CartridgeHeader::new(&data);
    //     assert_eq!(header.unwrap().title, "TETRIS");
    // }

    #[test]
    fn verify_memory_bank_type() {
        let data = fs::read_to_string("./testdata/tetris")
            .expect("file not found!")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>();
        let header = CartridgeHeader::new(&data);
        assert_eq!(
            header.unwrap().memory_bank_type,
            MemoryBankType::NoMemoryBank
        );
    }

    #[test]
    fn verify_ram_size() {
        let data = fs::read_to_string("./testdata/tetris")
            .expect("file not found!")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>();
        let header = CartridgeHeader::new(&data);
        assert_eq!(header.unwrap().ram_in_bytes(), 0);
    }

    // FIXME: enable it when gameboy_color_support is needed
    // #[test]
    // fn verify_gameboy_color_support() {
    //     let data = fs::read_to_string("./testdata/tetris")
    //         .expect("file not found!")
    //         .split(',')
    //         .map(|n| n.parse().unwrap())
    //         .collect::<Vec<u8>>();
    //     let header = CartridgeHeader::new(&data);
    //     assert_eq!(header.unwrap().gameboy_color_support, GameBoyColorFlag::GB);
    // }
}
