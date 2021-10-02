use core::panic;

use crate::{memory_device::ReadWrite, mmu::MMU, opcodes::*, prefix_opcodes::*, register::*};

pub struct CPU {
    registers: Registers,
    mmu: MMU,
    stop: bool,
    halt: bool,

    /// Interrupt master enable flag. EI/DI will set/reset this.
    ime: bool,
}

impl CPU {
    pub fn new(device: Box<dyn ReadWrite>) -> CPU {
        let mmu = MMU::new(device);
        CPU {
            registers: Registers::new(),
            mmu,
            stop: false,
            halt: false,
            ime: false,
        }
    }

    pub fn step(self: &mut Self) -> u8 {
        if self.stop {
            return 0;
        }

        if self.halt {
            return self.noop();
        }

        let op_code = self.fetch_byte();
        println!("{:#04x}", op_code);
        match op_code.into() {
            OpCode::LdBNext => self.ld_r_next(Register::B),
            OpCode::LdCNext => self.ld_r_next(Register::C),
            OpCode::LdDNext => self.ld_r_next(Register::D),
            OpCode::LdENext => self.ld_r_next(Register::E),
            OpCode::LdHNext => self.ld_r_next(Register::H),
            OpCode::LdLNext => self.ld_r_next(Register::L),
            OpCode::LdANext => self.ld_r_next(Register::A),

            OpCode::LdBB => self.ld_r_r(Register::B, Register::B),
            OpCode::LdBC => self.ld_r_r(Register::B, Register::C),
            OpCode::LdBD => self.ld_r_r(Register::B, Register::D),
            OpCode::LdBE => self.ld_r_r(Register::B, Register::E),
            OpCode::LdBH => self.ld_r_r(Register::B, Register::H),
            OpCode::LdBL => self.ld_r_r(Register::B, Register::L),
            OpCode::LdBA => self.ld_r_r(Register::B, Register::A),
            OpCode::LdCB => self.ld_r_r(Register::C, Register::B),
            OpCode::LdCC => self.ld_r_r(Register::C, Register::C),
            OpCode::LdCD => self.ld_r_r(Register::C, Register::D),
            OpCode::LdCE => self.ld_r_r(Register::C, Register::E),
            OpCode::LdCH => self.ld_r_r(Register::C, Register::H),
            OpCode::LdCL => self.ld_r_r(Register::C, Register::L),
            OpCode::LdCA => self.ld_r_r(Register::C, Register::A),
            OpCode::LdDB => self.ld_r_r(Register::D, Register::B),
            OpCode::LdDC => self.ld_r_r(Register::D, Register::C),
            OpCode::LdDD => self.ld_r_r(Register::D, Register::D),
            OpCode::LdDE => self.ld_r_r(Register::D, Register::E),
            OpCode::LdDH => self.ld_r_r(Register::D, Register::H),
            OpCode::LdDL => self.ld_r_r(Register::D, Register::L),
            OpCode::LdDA => self.ld_r_r(Register::D, Register::A),
            OpCode::LdEB => self.ld_r_r(Register::E, Register::B),
            OpCode::LdEC => self.ld_r_r(Register::E, Register::C),
            OpCode::LdED => self.ld_r_r(Register::E, Register::D),
            OpCode::LdEE => self.ld_r_r(Register::E, Register::E),
            OpCode::LdEH => self.ld_r_r(Register::E, Register::H),
            OpCode::LdEL => self.ld_r_r(Register::E, Register::L),
            OpCode::LdEA => self.ld_r_r(Register::E, Register::A),
            OpCode::LdHB => self.ld_r_r(Register::H, Register::B),
            OpCode::LdHC => self.ld_r_r(Register::H, Register::C),
            OpCode::LdHD => self.ld_r_r(Register::H, Register::D),
            OpCode::LdHE => self.ld_r_r(Register::H, Register::E),
            OpCode::LdHH => self.ld_r_r(Register::H, Register::H),
            OpCode::LdHL => self.ld_r_r(Register::H, Register::L),
            OpCode::LdHA => self.ld_r_r(Register::H, Register::A),
            OpCode::LdLB => self.ld_r_r(Register::L, Register::B),
            OpCode::LdLC => self.ld_r_r(Register::L, Register::C),
            OpCode::LdLD => self.ld_r_r(Register::L, Register::D),
            OpCode::LdLE => self.ld_r_r(Register::L, Register::E),
            OpCode::LdLH => self.ld_r_r(Register::L, Register::H),
            OpCode::LdLL => self.ld_r_r(Register::L, Register::L),
            OpCode::LdLA => self.ld_r_r(Register::L, Register::A),
            OpCode::LdAB => self.ld_r_r(Register::A, Register::B),
            OpCode::LdAC => self.ld_r_r(Register::A, Register::C),
            OpCode::LdAD => self.ld_r_r(Register::A, Register::D),
            OpCode::LdAE => self.ld_r_r(Register::A, Register::E),
            OpCode::LdAH => self.ld_r_r(Register::A, Register::H),
            OpCode::LdAL => self.ld_r_r(Register::A, Register::L),
            OpCode::LdAA => self.ld_r_r(Register::A, Register::A),

            OpCode::LdBHL => self.ld_r_hl(Register::B),
            OpCode::LdCHL => self.ld_r_hl(Register::C),
            OpCode::LdDHL => self.ld_r_hl(Register::D),
            OpCode::LdEHL => self.ld_r_hl(Register::E),
            OpCode::LdHHL => self.ld_r_hl(Register::H),
            OpCode::LdLHL => self.ld_r_hl(Register::L),
            OpCode::LdAHL => self.ld_r_hl(Register::A),

            OpCode::LdHlB => self.ld_hl_r(Register::B),
            OpCode::LdHlC => self.ld_hl_r(Register::C),
            OpCode::LdHlD => self.ld_hl_r(Register::D),
            OpCode::LdHlE => self.ld_hl_r(Register::E),
            OpCode::LdHlH => self.ld_hl_r(Register::H),
            OpCode::LdHlL => self.ld_hl_r(Register::L),
            OpCode::LdHlA => self.ld_hl_r(Register::A),

            OpCode::LdABc => self.ld_a_nn(RegisterWord::BC),

            OpCode::LdHlN => self.ld_hl_next(),

            OpCode::LdBcA => self.ld_bc_a(),

            OpCode::LdBcNn => self.ld_dd_nn(RegisterWord::BC),
            OpCode::LdDeNn => self.ld_dd_nn(RegisterWord::DE),
            OpCode::LdHlNn => self.ld_dd_nn(RegisterWord::HL),
            OpCode::LdSpNn => self.ld_dd_nn(RegisterWord::SP),

            OpCode::LddHlA => self.ldd_hl_a(),
            OpCode::LdAFF00n => self.ld_a_ff00_n(),
            OpCode::LdFF00nA => self.ld_ff00_na(),

            OpCode::OrB => self.or_r(Register::B),
            OpCode::OrC => self.or_r(Register::C),
            OpCode::OrD => self.or_r(Register::D),
            OpCode::OrE => self.or_r(Register::E),
            OpCode::OrH => self.or_r(Register::H),
            OpCode::OrL => self.or_r(Register::L),
            OpCode::OrA => self.or_r(Register::A),

            OpCode::XorB => self.xor_r(Register::B),
            OpCode::XorC => self.xor_r(Register::C),
            OpCode::XorD => self.xor_r(Register::D),
            OpCode::XorE => self.xor_r(Register::E),
            OpCode::XorH => self.xor_r(Register::H),
            OpCode::XorL => self.xor_r(Register::L),
            OpCode::XorA => self.xor_r(Register::A),

            OpCode::IncBC => self.inc_rr(RegisterWord::BC),
            OpCode::IncDE => self.inc_rr(RegisterWord::DE),
            OpCode::IncHL => self.inc_rr(RegisterWord::HL),
            OpCode::IncSP => self.inc_rr(RegisterWord::SP),

            OpCode::IncB => self.inc_r(Register::B),
            OpCode::IncC => self.inc_r(Register::C),
            OpCode::IncD => self.inc_r(Register::D),
            OpCode::IncE => self.inc_r(Register::E),
            OpCode::IncH => self.inc_r(Register::H),
            OpCode::IncL => self.inc_r(Register::L),
            OpCode::IncA => self.inc_r(Register::A),

            OpCode::DecB => self.dec_r(Register::B),
            OpCode::DecC => self.dec_r(Register::C),
            OpCode::DecD => self.dec_r(Register::D),
            OpCode::DecE => self.dec_r(Register::E),
            OpCode::DecH => self.dec_r(Register::H),
            OpCode::DecL => self.dec_r(Register::L),
            OpCode::DecA => self.dec_r(Register::A),

            OpCode::JpNN => self.jp_nn(),

            OpCode::JrNzPcDd => self.jr_f_pc_dd(ConditionOperand::NZ),
            OpCode::JrZPcDd => self.jr_f_pc_dd(ConditionOperand::Z),
            OpCode::JrNcPcDd => self.jr_f_pc_dd(ConditionOperand::NC),
            OpCode::JrCPcDd => self.jr_f_pc_dd(ConditionOperand::C),

            OpCode::CpN => self.cp_n(),

            OpCode::RrA => self.rr_a(),

            OpCode::DI => self.di(),

            OpCode::Noop => self.noop(),
            OpCode::Stop => self.stop(),
            OpCode::Halt => self.halt(),

            OpCode::CB => self.interpret_prefix(),
        }
    }

    fn fetch_byte(self: &mut Self) -> u8 {
        let byte = self
            .mmu
            .read_byte(self.registers.program_counter as usize)
            .unwrap();
        self.registers.program_counter += 1;

        byte
    }

    fn fetch_word(self: &mut Self) -> u16 {
        let word = self
            .mmu
            .read_word(self.registers.program_counter as usize)
            .unwrap();
        self.registers.program_counter += 2;

        word
    }

    fn noop(self: &Self) -> u8 {
        4
    }

    fn stop(self: &mut Self) -> u8 {
        self.stop = true;
        0
    }

    fn halt(self: &mut Self) -> u8 {
        self.halt = true;
        0
    }

    fn ld_r_next(self: &mut Self, reg: Register) -> u8 {
        let r = self.fetch_byte();
        match reg {
            Register::B => self.registers.b = r,
            Register::C => self.registers.c = r,
            Register::D => self.registers.d = r,
            Register::E => self.registers.e = r,
            Register::H => self.registers.h = r,
            Register::L => self.registers.l = r,
            Register::A => self.registers.a = r,
        };

        8
    }

    fn ld_r_r(self: &mut Self, reg1: Register, reg2: Register) -> u8 {
        let r2 = match reg2 {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
        };

        match reg1 {
            Register::B => self.registers.b = r2,
            Register::C => self.registers.c = r2,
            Register::D => self.registers.d = r2,
            Register::E => self.registers.e = r2,
            Register::H => self.registers.h = r2,
            Register::L => self.registers.l = r2,
            Register::A => self.registers.a = r2,
        };

        4
    }

    fn ld_r_hl(self: &mut Self, reg: Register) -> u8 {
        let hl = self.mmu.read_byte(self.registers.hl() as usize).unwrap();
        match reg {
            Register::B => self.registers.b = hl,
            Register::C => self.registers.c = hl,
            Register::D => self.registers.d = hl,
            Register::E => self.registers.e = hl,
            Register::H => self.registers.h = hl,
            Register::L => self.registers.l = hl,
            Register::A => self.registers.a = hl,
        };

        8
    }

    fn ld_hl_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
        };

        match self.mmu.write_byte(self.registers.hl() as usize, r) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        8
    }

    fn ld_a_nn(self: &mut Self, reg: RegisterWord) -> u8 {
        let address = match reg {
            RegisterWord::BC => self.registers.bc(),
            RegisterWord::DE => self.registers.de(),
            RegisterWord::HL => self.registers.hl(),
            RegisterWord::SP => self.registers.stack_pointer,
        };
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        self.registers.a = v;

        8
    }

    fn ld_hl_next(self: &mut Self) -> u8 {
        let hl = self.registers.hl();
        let next = self.fetch_byte();
        self.mmu.write_byte(hl as usize, next).unwrap(); // TODO: check result

        8
    }

    fn ld_bc_a(self: &mut Self) -> u8 {
        self.mmu
            .write_byte(self.registers.bc() as usize, self.registers.a)
            .unwrap();

        8
    }

    fn ld_dd_nn(self: &mut Self, reg: RegisterWord) -> u8 {
        let w = self.fetch_word();
        match reg {
            RegisterWord::BC => self.registers.set_bc(w),
            RegisterWord::DE => self.registers.set_de(w),
            RegisterWord::HL => self.registers.set_hl(w),
            RegisterWord::SP => self.registers.stack_pointer = w,
        };

        12
    }

    fn ldd_hl_a(self: &mut Self) -> u8 {
        self.mmu
            .write_byte((self.registers.hl() - 1) as usize, self.registers.a)
            .unwrap();

        8
    }

    fn ld_a_ff00_n(self: &mut Self) -> u8 {
        let add = 0xFF00 as usize + self.fetch_byte() as usize;
        self.registers.a = self.mmu.read_byte(add).unwrap();

        12
    }

    fn ld_ff00_na(self: &mut Self) -> u8 {
        let add = 0xFF00 as u16 + self.fetch_byte() as u16;
        self.mmu.write_byte(add as usize, self.registers.a).unwrap();

        12
    }

    fn or_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
            _ => panic!("can't ld_hl_r"),
        };

        self.registers.a |= r;
        // self.registers.flags.evaluate_effect(
        // self.registers.a,
        // self.registers.a,
        //     SideEffectsCpuFlags {
        //         carry: SideEffect::Unset,
        //         half_carry: SideEffect::Unset,
        //         negative: SideEffect::Unset,
        //         zero: SideEffect::Dependent,
        //     },
        // );

        4
    }

    fn xor_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
        };

        self.registers.a ^= r;
        // self.registers.flags.evaluate_effect(
        //     self.registers.a,
        //     self.registers.a,
        //     SideEffectsCpuFlags {
        //         carry: SideEffect::Unmodified,
        //         half_carry: SideEffect::Unmodified,
        //         negative: SideEffect::Unmodified,
        //         zero: SideEffect::Dependent,
        //     },
        // );

        4
    }

    fn inc_rr(self: &mut Self, reg: RegisterWord) -> u8 {
        match reg {
            RegisterWord::BC => self.registers.set_bc(self.registers.bc() + 1),
            RegisterWord::DE => self.registers.set_de(self.registers.de() + 1),
            RegisterWord::HL => self.registers.set_hl(self.registers.hl() + 1),
            RegisterWord::SP => self.registers.stack_pointer = self.registers.stack_pointer + 1,
        };

        8
    }

    fn inc_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => &mut self.registers.b,
            Register::C => &mut self.registers.c,
            Register::D => &mut self.registers.d,
            Register::E => &mut self.registers.e,
            Register::H => &mut self.registers.h,
            Register::L => &mut self.registers.l,
            Register::A => &mut self.registers.a,
        };

        let result = *r + 1;
        // self.registers.flags.evaluate_effect(
        //     *r,
        //     result,
        //     SideEffectsCpuFlags {
        //         carry: SideEffect::Unmodified,
        //         half_carry: SideEffect::Unmodified,
        //         negative: SideEffect::Unset,
        //         zero: SideEffect::Dependent,
        //     },
        // );

        *r = result;

        8
    }

    fn dec_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => &mut self.registers.b,
            Register::C => &mut self.registers.c,
            Register::D => &mut self.registers.d,
            Register::E => &mut self.registers.e,
            Register::H => &mut self.registers.h,
            Register::L => &mut self.registers.l,
            Register::A => &mut self.registers.a,
        };

        *r = r.wrapping_sub(1);

        // self.registers.flags.evaluate_effect(
        //     *r,
        //     *r,
        //     SideEffectsCpuFlags {
        //         carry: SideEffect::Unmodified,
        //         half_carry: SideEffect::Unmodified,
        //         negative: SideEffect::Unmodified,
        //         zero: SideEffect::Dependent,
        //     },
        // );

        4
    }

    fn jp_nn(self: &mut Self) -> u8 {
        let nn = self.fetch_word();
        self.registers.program_counter = nn as i32;
        16
    }

    fn jr_f_pc_dd(self: &mut Self, op: ConditionOperand) -> u8 {
        let dd = self.fetch_byte() as i8;
        let condition = match op {
            ConditionOperand::NZ => !self.registers.flags.zero,
            ConditionOperand::Z => self.registers.flags.zero,
            ConditionOperand::NC => !self.registers.flags.carry,
            ConditionOperand::C => self.registers.flags.carry,
        };

        if condition {
            self.registers.program_counter += dd as i32;
            return 12;
        }

        8
    }

    fn cp_n(self: &mut Self) -> u8 {
        let n = self.fetch_byte();
        let ret = self.registers.a - n;
        // self.registers.flags.evaluate_effect(
        // self.registers.a,
        // self.registers.a,
        // SideEffectsCpuFlags {
        // carry: SideEffect::Dependent,
        // half_carry: SideEffect::Dependent,
        // negative: SideEffect::Set,
        // zero: SideEffect::Dependent,
        // },
        // );
        // FIXME: finish this consistent on comment in evaluate_effect

        8
    }

    fn di(self: &mut Self) -> u8 {
        self.ime = false;
        4
    }

    fn rr_a(self: &mut Self) -> u8 {
        let old_bit_zero_data = self.registers.a & 0x01 == 0x01;
        let old_carry = self.registers.flags.carry as u16;
        let big_a = (self.registers.a as u16).rotate_left(1) | old_carry;
        self.registers.flags.carry = big_a & (1 << 1) != 0;
        self.registers.a = ((big_a.rotate_right(1) ^ (old_carry << 8)) >> 1) as u8;
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        self.registers.flags.carry = old_bit_zero_data;

        4
    }

    fn interpret_prefix(self: &mut Self) -> u8 {
        let prefix_opcode = self.fetch_byte();
        match prefix_opcode.into() {
            PrefixOpCode::RlcB => self.rlc_r(Register::B),
            PrefixOpCode::RlcC => self.rlc_r(Register::B),
            PrefixOpCode::RlcD => self.rlc_r(Register::B),
            PrefixOpCode::RlcE => self.rlc_r(Register::B),
            PrefixOpCode::RlcH => self.rlc_r(Register::B),
            PrefixOpCode::RlcL => self.rlc_r(Register::B),
            PrefixOpCode::RlcA => self.rlc_r(Register::B),
        }
    }

    fn rlc_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => &mut self.registers.b,
            Register::C => &mut self.registers.c,
            Register::D => &mut self.registers.d,
            Register::E => &mut self.registers.e,
            Register::H => &mut self.registers.h,
            Register::L => &mut self.registers.l,
            Register::A => &mut self.registers.a,
        };

        let sign = *r >> 7;
        let tmp = *r << 1;
        *r = tmp ^ sign;

        // self.registers.flags.evaluate_effect(
        //     *r,
        //     *r,
        //     SideEffectsCpuFlags {
        //         carry: SideEffect::Dependent,
        //         half_carry: SideEffect::Unset,
        //         negative: SideEffect::Unset,
        //         zero: SideEffect::Dependent,
        //     },
        // );

        8
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_device::ReadWrite;
    use crate::opcodes::{Register, RegisterWord};

    use super::CPU;

    struct MockDevice {
        fake_byte_to_return: u8,
        fake_word_to_return: u16,
    }

    impl ReadWrite for MockDevice {
        fn contains(self: &Self, address: usize) -> bool {
            let _ = address;
            true
        }

        fn read_byte(self: &Self, address: usize) -> Result<u8, std::io::Error> {
            let _ = address;
            Ok(self.fake_byte_to_return)
        }
        fn read_word(self: &Self, address: usize) -> Result<u16, std::io::Error> {
            let _ = address;
            Ok(self.fake_word_to_return)
        }

        fn write_byte(self: &mut Self, address: usize, value: u8) -> Result<(), std::io::Error> {
            let _ = address;
            Ok(self.fake_byte_to_return = value)
        }
        fn write_word(self: &mut Self, address: usize, value: u16) -> Result<(), std::io::Error> {
            let _ = address;
            Ok(self.fake_word_to_return = value)
        }
    }

    #[test]
    fn verify_ld_r_next() {
        {
            let mc = MockDevice {
                fake_byte_to_return: 10,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.ld_r_next(Register::A);
            assert_eq!(cpu.registers.a, 10);
        }
    }

    #[test]
    fn verify_ld_r_r() {
        {
            let mc = MockDevice {
                fake_byte_to_return: 0,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.b = 9;
            let cycle = cpu.ld_r_r(Register::A, Register::B);
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 9);
        }
    }

    #[test]
    fn verify_ld_r_hl() {
        {
            let mc = MockDevice {
                fake_byte_to_return: 87,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.set_hl(44);
            let cycle = cpu.ld_r_hl(Register::B);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.b, 87);
        }
    }

    #[test]
    fn verify_ld_hl_r() {
        {
            let mc = MockDevice {
                fake_byte_to_return: 0,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.b = 99;
            let cycle = cpu.ld_hl_r(Register::B);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.mmu.read_byte(0).unwrap(), 99);
        }
    }

    #[test]
    fn verify_ld_a_nn() {
        {
            let mc = MockDevice {
                fake_byte_to_return: 100,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.set_bc(99);
            let cycle = cpu.ld_a_nn(RegisterWord::BC);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 100);
        }
    }

    #[test]
    fn verify_rr_a() {
        {
            let mc = MockDevice {
                fake_byte_to_return: 0,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.a = 1;
            let cycle = cpu.rr_a();
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 0);
            assert_eq!(cpu.registers.flags.zero, true);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, true);
        }
        {
            let mc = MockDevice {
                fake_byte_to_return: 0,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.a = 2;
            let cycle = cpu.rr_a();
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 1);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                fake_byte_to_return: 0,
                fake_word_to_return: 0,
            };
            let mut cpu = CPU::new(Box::new(mc));
            cpu.registers.a = 3;
            let cycle = cpu.rr_a();
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 1);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, true);
        }
    }
}
