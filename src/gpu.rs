use crate::memory_device::ReadWrite;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Palette {
    index_0: Color,
    index_1: Color,
    index_2: Color,
    index_3: Color,
}

impl From<Palette> for u8 {
    fn from(p: Palette) -> Self {
        p.index_0 as u8 | (p.index_1 as u8) << 2 | (p.index_2 as u8) << 4 | (p.index_3 as u8) << 6
    }
}

impl From<u8> for Palette {
    fn from(reg: u8) -> Self {
        Self {
            index_0: (reg & 0b11).into(),
            index_1: (reg >> 2 & 0b11).into(),
            index_2: (reg >> 4 & 0b11).into(),
            index_3: (reg >> 6 & 0b11).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl From<u8> for Color {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::White,
            1 => Self::LightGray,
            2 => Self::DarkGray,
            3 => Self::Black,
            _ => unreachable!("2 bits value cannot exceed 3"),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}

const SCREEN_W: usize = 160;
const SCREEN_H: usize = 144;

#[allow(dead_code)]
pub struct GraphicsProcessingUnit {
    // video ram: 0x8000-0x9FFF
    vram: [u8; 0x4000],
    bank: u8,

    // Digital image with mode RGB.
    data: [u8; SCREEN_W * SCREEN_H * 3],

    // Gameboy video controller can display up to 40 sprites either in 8x8 or in 8x16 pixels.
    // Because of a limitation of hardware, only ten sprites can be displayed per scan line.
    // Sprite patterns have the same format as BG tiles, but they are taken from the Sprite
    // Pattern Table located at 0x8000-0x8FFF and have unsigned numbering.
    // Byte0 - Y Position
    // Specifies the sprites vertical position on the screen (minus 16). An off-screen value (for example, Y=0 or
    // Y>=160) hides the sprite.
    //
    // Byte1 - X Position
    // Specifies the sprites horizontal position on the screen (minus 8). An off-screen value (X=0 or X>=168) hides the
    // sprite, but the sprite still affects the priority ordering - a better way to hide a sprite is to set its
    // Y-coordinate off-screen.
    //
    // Byte2 - Tile/Pattern Number
    // Specifies the sprites Tile Number (00-FF). This (unsigned) value selects a tile from memory at 8000h-8FFFh. In
    // CGB Mode this could be either in VRAM Bank 0 or 1, depending on Bit 3 of the following byte. In 8x16 mode, the
    // lower bit of the tile number is ignored. IE: the upper 8x8 tile is "NN AND FEh", and the lower 8x8 tile
    // is "NN OR 01h".
    //
    // Byte3 - Attributes/Flags:
    // Bit7   OBJ-to-BG Priority (0=OBJ Above BG, 1=OBJ Behind BG color 1-3)
    //        (Used for both BG and Window. BG color 0 is always behind OBJ)
    // Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
    // Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
    // Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
    // Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
    // Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
    oam: [u8; 0xA0],

    /// Current status of LCD displsy: 0xFF41
    /// The LCD controller operates on a 222 Hz = 4.194 MHz dot clock.
    /// An entire frame is 154 scanlines, 70224 dots, or 16.74 ms.
    /// On scanlines 0 through 143, the LCD controller cycles through modes 2, 3, and 0 once every 456 dots.
    /// Scanlines 144 through 153 are mode 1.
    /// Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)
    ///           0: During H-Blank
    ///           1: During V-Blank
    ///           2: During Searching OAM
    ///           3: During Transferring Data to LCD Driver
    /// Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)
    /// Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)
    /// Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)
    /// Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
    /// Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
    status: u8,

    // Scroll Y (R/W), Scroll X (R/W)
    // Specifies the position in the 256x256 pixels BG map (32x32 tiles) which is to be displayed at the upper/left LCD
    // display position. Values in range from 0-255 may be used for X/Y each, the video controller automatically wraps
    // back to the upper (left) position in BG map when drawing exceeds the lower (right) border of the BG map area.
    // scroll Y 0xFF42 (read-write)
    scroll_y: u8,
    // scroll X 0xFF43 (read-write)
    scroll_x: u8,

    /// Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)
    /// Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
    /// Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
    /// Bit 3 - BG Tile Map Display Select     (0=0x9800-0x9BFF, 1=0x9C00-0x9FFF)
    /// Bit 4 - BG & Window Tile Data Select   (0=0x8800-0x97FF, 1=0x8000-0x8FFF)
    /// Bit 5 - Window Display Enable          (0=Off, 1=On)
    /// Bit 6 - Window Tile Map Display Select (0=0x9800-0x9BFF, 1=0x9C00-0x9FFF)
    /// Bit 7 - LCD Display Enable             (0=Off, 1=On)
    control: u8,

    // The LY indicates the vertical line to which the present data is transferred to the LCD Driver.
    // The LY can take on any value between 0 through 153. The values between 144 and 153 indicate the V-Blank period.
    current_y: u8,

    // This register assigns gray shades to the color indexes of the BG and Window tiles.
    // Bit 7-6 - Color for index 3
    // Bit 5-4 - Color for index 2
    // Bit 3-2 - Color for index 1
    // Bit 1-0 - Color for index 0
    bg_pallete: Palette,

    // These registers assigns gray shades to the color indexes of the OBJs that use the corresponding palette.
    // They work exactly like BGP, except that the lower two bits are ignored because color index 0 is transparent for OBJs.
    // This register assigns gray shades for sprite palette 0. It works exactly as BGP (FF47), except that the lower
    // two bits aren't used because sprite data 00 is transparent.
    bgj_pallete_0: Palette,
    // This register assigns gray shades for sprite palette 1. It works exactly as BGP (FF47), except that the lower
    // two bits aren't used because sprite data 00 is transparent.
    bgj_pallete_1: Palette,
}

impl GraphicsProcessingUnit {
    pub fn new() -> GraphicsProcessingUnit {
        GraphicsProcessingUnit {
            vram: [0; 0x4000],
            bank: 0,
            data: [0xFF; SCREEN_W * SCREEN_H * 3],
            oam: [0x00; 0xA0],
            status: 0,
            scroll_y: 0,
            scroll_x: 0,
            control: 0,
            current_y: 0,
            bg_pallete: Palette::default(),
            bgj_pallete_0: Palette::default(),
            bgj_pallete_1: Palette::from(1),
        }
    }
}

impl ReadWrite for GraphicsProcessingUnit {
    fn contains(&self, address: usize) -> bool {
        (0x8000..=0x9FFF).contains(&address)
            || (0xFE00..=0xFE9F).contains(&address)
            || 0xFF40 == address
            || 0xFF41 == address
            || 0xFF42 == address
            || 0xFF43 == address
            || 0xFF44 == address
            || 0xFF47 == address
            || 0xFF48 == address
            || 0xFF49 == address
            || 0xFF4F == address
    }

    fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
        match address {
            0x8000..=0x9FFF => {
                Ok(self.vram[self.bank as usize * 0x2000_usize + address - 0x8000_usize])
            }
            0xFE00..=0xFE9F => Ok(self.oam[address - 0xFE00]),
            0xFF40 => Ok(self.control),
            0xFF41 => Ok(self.status),
            0xFF42 => Ok(self.scroll_y),
            0xFF43 => Ok(self.scroll_x),
            0xFF44 => Ok(self.current_y),
            0xFF47 => Ok(self.bg_pallete.into()),
            0xFF48 => Ok(self.bgj_pallete_0.into()),
            0xFF49 => Ok(self.bgj_pallete_1.into()),
            0xFF4F => Ok(0xFE | self.bank),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "can't write byte here",
            )),
        }
    }

    fn read_word(&self, _address: usize) -> Result<u16, std::io::Error> {
        unimplemented!()
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
        match address {
            0x8000..=0x9FFF => {
                self.vram[self.bank as usize * 0x2000_usize + address - 0x8000_usize] = value
            }
            0xFE00..=0xFE9F => self.oam[address - 0xFE00] = value,
            0xFF40 => self.control = value,
            0xFF41 => self.status = value,
            0xFF42 => self.scroll_y = value,
            0xFF43 => self.scroll_x = value,
            0xFF44 => self.current_y = value,
            0xFF47 => self.bg_pallete = value.into(),
            0xFF48 => self.bgj_pallete_0 = value.into(),
            0xFF49 => self.bgj_pallete_1 = value.into(),
            0xFF4F => self.bank = value & 0x01,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "can't write byte here",
                ))
            }
        }

        Ok(())
    }

    fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::gpu::{Color, Palette};

    #[test]
    fn palette_from_u8() {
        let value = 0b00_01_10_11;
        assert_eq!(
            Palette::from(value),
            Palette {
                index_0: Color::Black,
                index_1: Color::DarkGray,
                index_2: Color::LightGray,
                index_3: Color::White,
            }
        );
    }

    #[test]
    fn u8_from_palette() {
        let palette = Palette {
            index_0: Color::Black,
            index_1: Color::DarkGray,
            index_2: Color::LightGray,
            index_3: Color::White,
        };
        assert_eq!(Into::<u8>::into(palette), 0b00_01_10_11);
    }
}
