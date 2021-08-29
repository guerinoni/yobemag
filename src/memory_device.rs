pub trait ReadWrite {
    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error>;
    fn read_word(self: &Self, address: usize) -> Result<u8, std::io::Error>;

    fn write_byte(self: &Self, address: usize, value: u8) -> Result<(), std::io::Error>;
    fn write_word(self: &Self, address: usize, value: u16) -> Result<(), std::io::Error>;
}
