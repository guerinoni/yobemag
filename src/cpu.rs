use crate::{memory_device::ReadWrite, opcodes::*, register::*};

pub struct CPU {
    registers: Registers,
    device: Box<dyn ReadWrite>,
    stop: bool,
    halt: bool,
}

impl CPU {
    pub fn new(device: Box<dyn ReadWrite>) -> CPU {
        CPU {
            registers: Registers::new(),
            device,
            stop: false,
            halt: false,
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
        dbg!(op_code);
        match op_code.into() {
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

            OpCode::LdBNext => self.ld_r_next(Register::B),
            OpCode::LdCNext => self.ld_r_next(Register::C),
            OpCode::LdDNext => self.ld_r_next(Register::D),
            OpCode::LdENext => self.ld_r_next(Register::E),
            OpCode::LdHNext => self.ld_r_next(Register::H),
            OpCode::LdLNext => self.ld_r_next(Register::L),
            OpCode::LdANext => self.ld_r_next(Register::A),

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

            OpCode::LdHlN => self.ld_hl_next(),

            OpCode::LdBcNn => self.ld_dd_nn(RegisterWord::BC),
            OpCode::LdDeNn => self.ld_dd_nn(RegisterWord::DE),
            OpCode::LdHlNn => self.ld_dd_nn(RegisterWord::HL),
            OpCode::LdSpNn => self.ld_dd_nn(RegisterWord::SP),

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

            OpCode::JpNN => self.jp_nn(),

            OpCode::Noop => self.noop(),
            OpCode::Stop => self.stop(),
            OpCode::Halt => self.halt(),
        }
    }

    fn fetch_byte(self: &mut Self) -> u8 {
        let byte = self
            .device
            .read_byte(self.registers.program_counter as usize)
            .unwrap();
        self.registers.program_counter += 1;

        byte
    }

    fn fetch_word(self: &mut Self) -> u16 {
        let word = self
            .device
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

    fn ld_r_r(self: &mut Self, reg1: Register, reg2: Register) -> u8 {
        let r2 = match reg2 {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
            _ => panic!("can't ld_r_r"),
        };

        match reg1 {
            Register::B => self.registers.b = r2,
            Register::C => self.registers.c = r2,
            Register::D => self.registers.d = r2,
            Register::E => self.registers.e = r2,
            Register::H => self.registers.h = r2,
            Register::L => self.registers.l = r2,
            Register::A => self.registers.a = r2,
            _ => panic!("can't ld_r_r"),
        };

        4
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
            _ => panic!("can't ld_r_next"),
        };

        8
    }

    fn ld_r_hl(self: &mut Self, reg: Register) -> u8 {
        let hl = self.device.read_byte(self.registers.hl() as usize).unwrap();
        match reg {
            Register::B => self.registers.b = hl,
            Register::C => self.registers.c = hl,
            Register::D => self.registers.d = hl,
            Register::E => self.registers.e = hl,
            Register::H => self.registers.h = hl,
            Register::L => self.registers.l = hl,
            Register::A => self.registers.a = hl,
            _ => panic!("can't ld_r_hl"),
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
            _ => panic!("can't ld_hl_r"),
        };

        self.device
            .write_byte(self.registers.hl() as usize, r)
            .unwrap(); // TODO: check result

        8
    }

    fn ld_hl_next(self: &mut Self) -> u8 {
        let hl = self.registers.hl();
        let next = self.fetch_byte();
        self.device.write_byte(hl as usize, next).unwrap(); // TODO: check result

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

    fn xor_r(self: &mut Self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
            _ => panic!("can't xor_r"),
        };

        self.registers.a ^= r;
        self.registers.flags.evaluate_effect(
            self.registers.a,
            SideEffectsCpuFlags {
                carry: SideEffect::Unmodified,
                half_carry: SideEffect::Unmodified,
                negative: SideEffect::Unmodified,
                zero: SideEffect::Dependent,
            },
        );

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

    fn jp_nn(self: &mut Self) -> u8 {
        let nn = self.fetch_word();
        self.registers.program_counter = nn;
        16
    }
}

#[cfg(test)]
mod tests {
    use crate::cartridge::make_cartridge;

    use super::CPU;

    #[test]
    fn check() {
        let device = make_cartridge("./roms/Tetris.gb").unwrap();
        let mut cpu = CPU::new(device);
        let cycles = vec![4, 16, 16, 4, 12, 8, 8, 8];
        for c in cycles {
            assert_eq!(cpu.step(), c);
        }
    }
}
