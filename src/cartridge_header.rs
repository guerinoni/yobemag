pub struct CartridgeHeader {
    title: String,
}

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

fn check_logo(data: &Vec<u8>) -> Result<(), std::io::Error> {
    match data[0x104..0x134].iter().cmp(NINTENDO_LOGO.iter()) {
        std::cmp::Ordering::Equal => Ok(()),
        std::cmp::Ordering::Less | std::cmp::Ordering::Greater => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "logo bytes are corrupted.",
        )),
    }
}

impl CartridgeHeader {
    pub fn new(data: &Vec<u8>) -> Result<Self, std::io::Error> {
        let t = data[0x134..0x144]
            .iter()
            .take_while(|&&v| v > 0)
            .map(|&c| c)
            .collect::<Vec<_>>();

        check_logo(&data)?;

        Ok(CartridgeHeader {
            title: String::from_utf8(t).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::cartridge_header::CartridgeHeader;

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
}
