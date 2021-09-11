pub trait ReadWrite {
    fn contains(self: &Self, address: usize) -> bool;

    fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error>;
    fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error>;

    fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error>;
    fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error>;
}
