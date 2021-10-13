use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;

pub struct CpuFlag {
    pub carry: bool,
    pub half_carry: bool,
    pub negative: bool,
    pub zero: bool,
}

impl CpuFlag {
    pub fn to_u8(self: &Self) -> u8 {
        let bits = [
            self.zero as u8,
            self.negative as u8,
            self.half_carry as u8,
            self.carry as u8,
            0,
            0,
            0,
            0,
        ];
        convert(&bits).unwrap()
    }
}

#[derive(Debug)]
pub enum ConversionError {
    Overflow,
    NonBinaryInput,
}

pub fn convert<T: PartialEq + From<u8> + BitXor<Output = T> + Shl<Output = T> + Clone>(
    bits: &[u8],
) -> Result<T, ConversionError> {
    if bits.len() > (std::mem::size_of::<T>() * 8) {
        return Err(ConversionError::Overflow);
    }
    if bits.iter().filter(|&&bit| bit != 0 && bit != 1).count() > 0 {
        return Err(ConversionError::NonBinaryInput);
    }

    Ok(bits.iter().fold(T::from(0), |result, &bit| {
        (result << T::from(1)) ^ T::from(bit)
    }))
}

pub struct Registers {
    pub a: u8,
    pub flags: CpuFlag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub program_counter: i32,
    pub stack_pointer: u16,
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
        let ret = (self.b as u16) << 8;
        ret | self.c as u16
    }

    pub fn set_de(self: &mut Self, value: u16) {
        self.d = (value >> 8 as u16) as u8;
        self.e = value as u8;
    }

    pub fn de(self: &Self) -> u16 {
        let ret = (self.d as u16) << 8;
        ret | self.e as u16
    }

    pub fn set_hl(self: &mut Self, value: u16) {
        self.h = (value >> 8 as u16) as u8;
        self.l = value as u8;
    }

    pub fn hl(self: &Self) -> u16 {
        let ret = (self.h as u16) << 8;
        ret | self.l as u16
    }

    pub fn af(self: &Self) -> u16 {
        let ret = (self.a as u16) << 8;
        ret | self.flags.to_u8() as u16
    }
}

#[cfg(test)]
mod tests {
    use super::{CpuFlag, Registers};

    #[test]
    fn verify_conversion_flag() {
        let flags = CpuFlag {
            carry: false,
            half_carry: false,
            negative: false,
            zero: false,
        };

        assert_eq!(flags.to_u8(), 0);

        let flags = CpuFlag {
            carry: false,
            half_carry: false,
            negative: false,
            zero: true,
        };

        assert_eq!(flags.to_u8(), 128);

        let flags = CpuFlag {
            carry: false,
            half_carry: false,
            negative: true,
            zero: true,
        };

        assert_eq!(flags.to_u8(), 192);

        let flags = CpuFlag {
            carry: false,
            half_carry: true,
            negative: true,
            zero: true,
        };

        assert_eq!(flags.to_u8(), 224);

        let flags = CpuFlag {
            carry: true,
            half_carry: true,
            negative: true,
            zero: true,
        };

        assert_eq!(flags.to_u8(), 240);
    }

    #[test]
    fn verify_af() {
        let mut r = Registers::new();
        r.flags = CpuFlag {
            carry: true,
            half_carry: true,
            negative: true,
            zero: true,
        };

        dbg!(r.a, r.flags.to_u8());
        assert_eq!(r.af(), 496);
    }
}
