use std::cmp::PartialEq;
use std::ops::BitXor;
use std::ops::Shl;

// Description of register of GB.
// -------------
// | A   Flags |  ---> Program Status Word
// | B       C |  ---> B
// | D       E |  ---> D
// | H       L |  ---> H
// |    SP     |  ---> Stack Pointer
// |    PC     |  ---> Program Counter
// -------------

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum RegisterWord {
    BC,
    DE,
    HL,
    SP,
    AF,
}

pub enum ConditionOperand {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CpuFlag {
    // Carry Flag. This bit is set if a carry occurred from the last math operation or if register A is the smaller valuewhen executing the CP instruction.
    pub carry: bool,
    // Half Carry Flag. This bit is set if a carry occurred from the lowernibble in the last math operation.
    pub half_carry: bool,
    // Negative Flag. This bit is set if a subtraction was performed in the last math instruction.
    pub negative: bool,
    // Zero Flag. This bit is set when the result of a math operations zero or two values match when using the CP instruction.
    pub zero: bool,
}

impl CpuFlag {
    pub fn to_u8(&self) -> u8 {
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

    pub fn from_u8(&mut self, value: u8) {
        self.zero = value & 0b10000000 == 128;
        self.negative = value & 0b01000000 == 64;
        self.half_carry = value & 0b00100000 == 32;
        self.carry = value & 0b00010000 == 16;
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
    pub program_counter: u16,
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

    pub fn set_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::A => self.a = value,
        };
    }

    pub fn get_register_word(&mut self, reg: &RegisterWord) -> u16 {
        match reg {
            RegisterWord::BC => self.bc(),
            RegisterWord::DE => self.de(),
            RegisterWord::HL => self.hl(),
            RegisterWord::AF => self.af(),
            RegisterWord::SP => self.stack_pointer,
        }
    }

    pub fn set_register_word(&mut self, reg: &RegisterWord, value: u16) {
        match reg {
            RegisterWord::BC => self.set_bc(value),
            RegisterWord::DE => self.set_de(value),
            RegisterWord::HL => self.set_hl(value),
            RegisterWord::AF => self.set_af(value),
            RegisterWord::SP => self.stack_pointer = value,
        }
    }

    fn set(reg1: &mut u8, reg2: &mut u8, value: u16) {
        *reg1 = (value >> 8_u16) as u8;
        *reg2 = value as u8;
    }

    pub fn bc(&self) -> u16 {
        let ret = (self.b as u16) << 8;
        ret | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        Registers::set(&mut self.b, &mut self.c, value);
    }

    pub fn de(&self) -> u16 {
        let ret = (self.d as u16) << 8;
        ret | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        Registers::set(&mut self.d, &mut self.e, value);
    }

    pub fn hl(&self) -> u16 {
        let ret = (self.h as u16) << 8;
        ret | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        Registers::set(&mut self.h, &mut self.l, value);
    }

    pub fn af(&self) -> u16 {
        let ret = (self.a as u16) << 8;
        ret | self.flags.to_u8() as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8_u16) as u8;
        self.flags.from_u8(value as u8);
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

        let mut flags = CpuFlag {
            carry: false,
            half_carry: false,
            negative: false,
            zero: false,
        };
        flags.from_u8(240);
        assert_eq!(
            flags,
            CpuFlag {
                carry: true,
                half_carry: true,
                negative: true,
                zero: true,
            }
        );
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

        assert_eq!(r.af(), 496);
    }
}
