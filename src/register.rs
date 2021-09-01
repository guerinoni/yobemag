pub struct Registers {
    pub a: u8,
    pub flags: CpuFlag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub program_counter: u16,
    pub stack_pointer: u16,
}

#[derive(PartialEq, Eq)]
pub enum SideEffect {
    Unmodified,
    Set,
    Unset,
    Dependent,
}

pub struct SideEffectsCpuFlags {
    pub carry: SideEffect,
    pub half_carry: SideEffect,
    pub negative: SideEffect,
    pub zero: SideEffect,
}

pub struct CpuFlag {
    carry: bool,
    half_carry: bool,
    negative: bool,
    zero: bool,
}

impl CpuFlag {
    pub fn evaluate_effect(self: &mut Self, value: u8, effects: SideEffectsCpuFlags) {
        self.zero = match effects.zero {
            SideEffect::Dependent => value == 0,
            SideEffect::Set => true,
            SideEffect::Unset => false,
            SideEffect::Unmodified => self.zero,
        };

        self.carry = match effects.carry {
            SideEffect::Dependent => value != 0,
            SideEffect::Set => true,
            SideEffect::Unset => false,
            SideEffect::Unmodified => self.zero,
        };
    }
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

    pub fn set_bc(self: &mut Self, value: u16) {
        self.b = (value >> 8 as u16) as u8;
        self.c = value as u8;
    }

    pub fn bc(self: &Self) -> u16 {
        let mut ret = self.c as u16;
        ret = ret << 8;
        ret = ret | self.c as u16;
        ret
    }

    pub fn set_de(self: &mut Self, value: u16) {
        self.d = (value >> 8 as u16) as u8;
        self.e = value as u8;
    }

    pub fn de(self: &Self) -> u16 {
        let mut ret = self.d as u16;
        ret = ret << 8;
        ret = ret | self.e as u16;
        ret
    }

    pub fn set_hl(self: &mut Self, value: u16) {
        self.h = (value >> 8 as u16) as u8;
        self.l = value as u8;
    }

    pub fn hl(self: &Self) -> u16 {
        let mut ret = self.h as u16;
        ret = ret << 8;
        ret = ret | self.l as u16;
        ret
    }
}
