pub trait ReadWrite {
    fn contains(&self, address: usize) -> bool;

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error>;
    fn read_word(&self, address: usize) -> Result<u16, std::io::Error>;

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error>;
    fn write_word(&mut self, address: usize, value: u16) -> Result<(), std::io::Error>;
}
