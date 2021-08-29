pub struct CartridgeHeader {
    title: String,
}

impl CartridgeHeader {
    pub fn new(data: &Vec<u8>) -> Result<Self, std::io::Error> {
        let t = data[0x134..0x144]
            .iter()
            .take_while(|&&v| v > 0)
            .map(|&c| c)
            .collect::<Vec<_>>();

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
}
