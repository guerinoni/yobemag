// FF0F - IF - Interrupt Flag (R/W)
// Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
// Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
// Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
// Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
// Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)
// #[derive(Clone)]
pub enum Flag {
    VBlank  = 0,
    LCDStat = 1,
    Timer   = 2,
    Serial  = 3,
    Joypad  = 4,
}

#[derive(Default)]
pub struct InterruptFlag {
    pub data: u8,
}

impl InterruptFlag {
    pub fn request(&mut self, flag: Flag) {
        self.data |= 1 << flag as u8;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_vblank() {
        let mut interrupt = InterruptFlag::default();
        interrupt.request(Flag::VBlank);
        assert_eq!(interrupt.data, 1);
    }

    #[test]
    fn request_lcd() {
        let mut interrupt = InterruptFlag::default();
        interrupt.request(Flag::LCDStat);
        assert_eq!(interrupt.data, 2);
    }

    #[test]
    fn request_timer() {
        let mut interrupt = InterruptFlag::default();
        interrupt.request(Flag::Timer);
        assert_eq!(interrupt.data, 4);
    }

    #[test]
    fn request_serial() {
        let mut interrupt = InterruptFlag::default();
        interrupt.request(Flag::Serial);
        assert_eq!(interrupt.data, 8);
    }

    #[test]
    fn request_joypad() {
        let mut interrupt = InterruptFlag::default();
        interrupt.request(Flag::Joypad);
        assert_eq!(interrupt.data, 16);
    }
}
