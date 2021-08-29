pub struct Registers {
    pub a: u8,
    flags: CpuFlag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub program_counter: u16,
    pub stack_pointer: u16,
}

struct CpuFlag {
    carry: bool,
    half_carry: bool,
    negative: bool,
    zero: bool,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            flags: CpuFlag {
                carry: false,
                half_carry: false,
                negative: false,
                zero: false,
            },
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            program_counter: 0x0100,
            stack_pointer: 0xFFFE,
        }
    }

    pub fn hl(self: &Self) -> u16 {
        let ret = self.h << 8;
        let mut ret = ret as u16;
        ret = ret | self.l as u16;
        ret
    }
}
