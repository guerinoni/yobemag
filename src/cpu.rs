use crate::{
    memory_device::ReadWrite,
    opcodes::*,
    register::{self, Registers},
};

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
        let op_code = self.fetch_byte();
        dbg!(op_code);
        match op_code.into() {
            OpCode::Noop => self.noop(),
            OpCode::Stop => self.stop(),
            OpCode::Halt => self.halt(),

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
            OpCode::LdCNext => self.ld_r_next(Register::B),
            OpCode::LdDNext => self.ld_r_next(Register::B),
            OpCode::LdENext => self.ld_r_next(Register::B),
            OpCode::LdHNext => self.ld_r_next(Register::B),
            OpCode::LdLNext => self.ld_r_next(Register::B),
            OpCode::LdANext => self.ld_r_next(Register::B),

            OpCode::LdBHL => self.ld_r_next(Register::B),
            OpCode::LdCHL => self.ld_r_next(Register::B),
            OpCode::LdDHL => self.ld_r_next(Register::B),
            OpCode::LdEHL => self.ld_r_next(Register::B),
            OpCode::LdHHL => self.ld_r_next(Register::B),
            OpCode::LdLHL => self.ld_r_next(Register::B),
            OpCode::LdAHL => self.ld_r_next(Register::B),
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
            _ => panic!("can't ld_r_next"),
        };

        8
    }
}
