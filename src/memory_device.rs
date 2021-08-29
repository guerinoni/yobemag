pub trait ReadWrite {
    fn read_byte(self: &Self, address: u16) -> Result<u8, std::io::Error>;
    fn read_word(self: &Self, address: u16) -> Result<u8, std::io::Error>;

    fn write_byte(self: &Self, address: u16, value: u8) -> Result<(), std::io::Error>;
    fn write_word(self: &Self, address: u16, value: u16) -> Result<(), std::io::Error>;
}
