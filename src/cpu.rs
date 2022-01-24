use core::panic;

use crate::{
    memory_device::ReadWrite, mmu::MemoryManagmentUnit, opcodes::*, prefix_opcodes::PrefixOpCode,
    register::*,
};

pub struct CentralProcessingUnit {
    registers: Registers,
    mmu: MemoryManagmentUnit,
    stop: bool,
    halt: bool,

    // Interrupt master enable flag is reset by DI and prohibits all interrupts.
    // It is set by EI and acknowledges the interrupt setting by the IE register.
    ime: bool,
}

impl CentralProcessingUnit {
    pub fn new(device: Box<dyn ReadWrite>) -> CentralProcessingUnit {
        let mmu = MemoryManagmentUnit::new(device);
        CentralProcessingUnit {
            registers: Registers::new(),
            mmu,
            stop: false,
            halt: false,
            ime: false,
        }
    }

    pub fn step(&mut self) -> u8 {
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
            OpCode::LdABc => self.ld_a_rr(RegisterWord::BC),
            OpCode::LdADe => self.ld_a_rr(RegisterWord::DE),
            OpCode::LdNnA => self.ld_nn_a(),
            OpCode::LdANn => self.ld_a_nn(),
            OpCode::LdHlN => self.ld_hl_next(),
            OpCode::LdBcA => self.ld_rr_a(RegisterWord::BC),
            OpCode::LdDeA => self.ld_rr_a(RegisterWord::DE),
            OpCode::LdBcNn => self.ld_dd_nn(RegisterWord::BC),
            OpCode::LdDeNn => self.ld_dd_nn(RegisterWord::DE),
            OpCode::LdHlNn => self.ld_dd_nn(RegisterWord::HL),
            OpCode::LdSpNn => self.ld_dd_nn(RegisterWord::SP),
            OpCode::LddHlA => self.ldd_hl_a(),
            OpCode::LdHAn => self.ld_a_ff00_n(),
            OpCode::LdHnA => self.ld_ff00_na(),
            OpCode::LdHAC => self.ld_a_ff00c(),
            OpCode::LdHCA => self.ld_ff00_ca(),
            OpCode::LddAHl => self.ldd_a_hl(),
            OpCode::LdNnSP => self.ld_nn_sp(),
            OpCode::LdiAHl => self.ldi_a_hl(),
            OpCode::LdiHlA => self.ldi_hl_a(),
            OpCode::LdSpHl => self.ld_sp_hl(),
            OpCode::OrB => self.or_r(Register::B),
            OpCode::OrC => self.or_r(Register::C),
            OpCode::OrD => self.or_r(Register::D),
            OpCode::OrE => self.or_r(Register::E),
            OpCode::OrH => self.or_r(Register::H),
            OpCode::OrL => self.or_r(Register::L),
            OpCode::OrA => self.or_r(Register::A),
            OpCode::OrHl => self.or_hl(),
            OpCode::OrN => self.or_n(),
            OpCode::CpB => self.cp_r(Register::B),
            OpCode::CpC => self.cp_r(Register::C),
            OpCode::CpD => self.cp_r(Register::D),
            OpCode::CpE => self.cp_r(Register::E),
            OpCode::CpH => self.cp_r(Register::H),
            OpCode::CpL => self.cp_r(Register::L),
            OpCode::CpA => self.cp_r(Register::A),
            OpCode::CpHl => self.cp_hl(),
            OpCode::XorB => self.xor_r(Register::B),
            OpCode::XorC => self.xor_r(Register::C),
            OpCode::XorD => self.xor_r(Register::D),
            OpCode::XorE => self.xor_r(Register::E),
            OpCode::XorH => self.xor_r(Register::H),
            OpCode::XorL => self.xor_r(Register::L),
            OpCode::XorA => self.xor_r(Register::A),
            OpCode::XorHl => self.xor_hl(),
            OpCode::XorN => self.xor_n(),
            OpCode::AddHlBc => self.add_hl_rr(RegisterWord::BC),
            OpCode::AddHlDe => self.add_hl_rr(RegisterWord::DE),
            OpCode::AddHlHl => self.add_hl_rr(RegisterWord::HL),
            OpCode::AddHlSp => self.add_hl_rr(RegisterWord::SP),
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
            OpCode::DecBc => self.dec_rr(RegisterWord::BC),
            OpCode::DecDe => self.dec_rr(RegisterWord::DE),
            OpCode::DecHl => self.dec_rr(RegisterWord::HL),
            OpCode::DecSp => self.dec_rr(RegisterWord::SP),
            OpCode::DecB => self.dec_r(Register::B),
            OpCode::DecC => self.dec_r(Register::C),
            OpCode::DecD => self.dec_r(Register::D),
            OpCode::DecE => self.dec_r(Register::E),
            OpCode::DecH => self.dec_r(Register::H),
            OpCode::DecL => self.dec_r(Register::L),
            OpCode::DecA => self.dec_r(Register::A),
            OpCode::DecHlSpecific => self.dec_hl(),
            OpCode::JpNN => self.jp_nn(),
            OpCode::JpHl => self.jp_hl(),
            OpCode::JrNzPcDd => self.jr_f_pc_dd(ConditionOperand::NZ),
            OpCode::JrZPcDd => self.jr_f_pc_dd(ConditionOperand::Z),
            OpCode::JrNcPcDd => self.jr_f_pc_dd(ConditionOperand::NC),
            OpCode::JrCPcDd => self.jr_f_pc_dd(ConditionOperand::C),
            OpCode::JrPcDd => self.jr_pc_dd(),
            OpCode::JpNzNn => self.jp_f_nn(ConditionOperand::NZ),
            OpCode::JpZNn => self.jp_f_nn(ConditionOperand::Z),
            OpCode::JpNcNn => self.jp_f_nn(ConditionOperand::NC),
            OpCode::JpCNn => self.jp_f_nn(ConditionOperand::C),
            OpCode::CpN => self.cp_n(),
            OpCode::CallNzNn => self.call_flag_nn(ConditionOperand::NZ),
            OpCode::CallZNn => self.call_flag_nn(ConditionOperand::Z),
            OpCode::CallNcNn => self.call_flag_nn(ConditionOperand::NC),
            OpCode::CallCNn => self.call_flag_nn(ConditionOperand::C),
            OpCode::RrA => self.rr_a(),
            OpCode::Rlca => self.rlca(),
            OpCode::LdHlSps => self.ld_hl_sp(),
            OpCode::Ret => self.ret(),
            OpCode::RetNz => self.ret_f(ConditionOperand::NZ),
            OpCode::RetZ => self.ret_f(ConditionOperand::Z),
            OpCode::RetNc => self.ret_f(ConditionOperand::NC),
            OpCode::RetC => self.ret_f(ConditionOperand::C),
            OpCode::PushBc => self.push_rr(RegisterWord::BC),
            OpCode::PushDe => self.push_rr(RegisterWord::DE),
            OpCode::PushHl => self.push_rr(RegisterWord::HL),
            OpCode::PushAf => self.push_rr(RegisterWord::AF),
            OpCode::PopBc => self.pop_rr(RegisterWord::BC),
            OpCode::PopDe => self.pop_rr(RegisterWord::DE),
            OpCode::PopHl => self.pop_rr(RegisterWord::HL),
            OpCode::PopAf => self.pop_rr(RegisterWord::AF),
            OpCode::AddaN => self.add_a_n(),
            OpCode::AdcAn => self.adc_a_n(),
            OpCode::AddaB => self.add_a_r(Register::B),
            OpCode::AddaC => self.add_a_r(Register::C),
            OpCode::AddaD => self.add_a_r(Register::D),
            OpCode::AddaE => self.add_a_r(Register::E),
            OpCode::AddaH => self.add_a_r(Register::H),
            OpCode::AddaL => self.add_a_r(Register::L),
            OpCode::AddaA => self.add_a_r(Register::A),
            OpCode::AddAHl => self.add_a_hl(),
            OpCode::AdcB => self.adc_r(Register::B),
            OpCode::AdcC => self.adc_r(Register::C),
            OpCode::AdcD => self.adc_r(Register::D),
            OpCode::AdcE => self.adc_r(Register::E),
            OpCode::AdcH => self.adc_r(Register::H),
            OpCode::AdcL => self.adc_r(Register::L),
            OpCode::AdcA => self.adc_r(Register::A),
            OpCode::AdcAHl => self.adc_a_hl(),
            OpCode::SubB => self.sub_r(Register::B),
            OpCode::SubC => self.sub_r(Register::C),
            OpCode::SubD => self.sub_r(Register::D),
            OpCode::SubE => self.sub_r(Register::E),
            OpCode::SubH => self.sub_r(Register::H),
            OpCode::SubL => self.sub_r(Register::L),
            OpCode::SubA => self.sub_r(Register::A),
            OpCode::SubHl => self.sub_hl(),
            OpCode::SubN => self.sub_n(),
            OpCode::SbcAB => self.sbc_a_r(Register::B),
            OpCode::SbcAC => self.sbc_a_r(Register::C),
            OpCode::SbcAD => self.sbc_a_r(Register::D),
            OpCode::SbcAE => self.sbc_a_r(Register::E),
            OpCode::SbcAH => self.sbc_a_r(Register::H),
            OpCode::SbcAL => self.sbc_a_r(Register::L),
            OpCode::SbcAA => self.sbc_a_r(Register::A),
            OpCode::SbcAHl => self.sbc_a_hl(),
            OpCode::SbcAn => self.sbc_a_n(),
            OpCode::AndB => self.and_r(Register::B),
            OpCode::AndC => self.and_r(Register::C),
            OpCode::AndD => self.and_r(Register::D),
            OpCode::AndE => self.and_r(Register::E),
            OpCode::AndH => self.and_r(Register::H),
            OpCode::AndL => self.and_r(Register::L),
            OpCode::AndA => self.and_r(Register::A),
            OpCode::AndHl => self.and_hl(),
            OpCode::AndN => self.and_n(),
            OpCode::DI => self.di(),
            OpCode::CallNn => self.call_nn(),
            OpCode::Rst00 => self.rst(0x00),
            OpCode::Rst08 => self.rst(0x08),
            OpCode::Rst10 => self.rst(0x10),
            OpCode::Rst18 => self.rst(0x18),
            OpCode::Rst20 => self.rst(0x20),
            OpCode::Rst28 => self.rst(0x28),
            OpCode::Rst30 => self.rst(0x30),
            OpCode::Rst38 => self.rst(0x38),
            OpCode::Noop => self.noop(),
            OpCode::Stop => self.stop(),
            OpCode::Halt => self.halt(),
            OpCode::CB => self.interpret_prefix(),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let address = self.registers.program_counter as usize;
        self.registers.program_counter += 1;
        match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    fn fetch_word(&mut self) -> u16 {
        let address = self.registers.program_counter as usize;
        let word = match self.mmu.read_word(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.registers.program_counter += 2;

        word
    }

    fn noop(&self) -> u8 {
        4
    }

    fn stop(&mut self) -> u8 {
        self.stop = true;
        0
    }

    fn halt(&mut self) -> u8 {
        self.halt = true;
        0
    }

    pub fn check_for_half_carry_first_nible_add(a: u16, b: u16) -> bool {
        (((a & 0xF) + (b & 0xF)) & 0x10) != 0
    }

    pub fn check_for_half_carry_first_nible_sub(a: u8, b: u8) -> bool {
        let sa = (a & 0xF) as i8;
        let sb = (b & 0xF) as i8;
        (sa - sb) < 0
    }

    fn ld_r_next(&mut self, reg: Register) -> u8 {
        let r = self.fetch_byte();
        self.registers.set_register(reg, r);

        8
    }

    fn ld_r_r(&mut self, reg1: Register, reg2: Register) -> u8 {
        let v = self.registers.get_register(reg2);
        self.registers.set_register(reg1, v);

        4
    }

    fn ld_r_hl(&mut self, reg: Register) -> u8 {
        let address = self.registers.hl() as usize;
        let v = match self.mmu.read_byte(address) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.registers.set_register(reg, v);

        8
    }

    fn ld_hl_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        match self.mmu.write_byte(self.registers.hl() as usize, v) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        8
    }

    fn ld_a_rr(&mut self, reg: RegisterWord) -> u8 {
        let address = self.registers.get_register_word(&reg);
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.registers.a = v;

        8
    }

    fn ld_a_nn(&mut self) -> u8 {
        let address = self.fetch_word();
        self.registers.a = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        8
    }

    fn ld_rr_a(&mut self, reg: RegisterWord) -> u8 {
        let address = self.registers.get_register_word(&reg);
        match self.mmu.write_byte(address as usize, self.registers.a) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        8
    }

    fn ld_nn_a(&mut self) -> u8 {
        let address = self.fetch_word();
        match self.mmu.write_byte(address as usize, self.registers.a) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        16
    }

    fn ld_hl_next(&mut self) -> u8 {
        let hl = self.registers.hl();
        let next = self.fetch_byte();
        match self.mmu.write_byte(hl as usize, next) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        12
    }

    fn ld_dd_nn(&mut self, reg: RegisterWord) -> u8 {
        let v = self.fetch_word();
        self.registers.set_register_word(&reg, v);

        12
    }

    fn ldd_hl_a(&mut self) -> u8 {
        let address = self.registers.hl();
        match self.mmu.write_byte(address as usize, self.registers.a) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        self.registers.set_hl(address - 1);

        8
    }

    fn ld_a_ff00_n(&mut self) -> u8 {
        let address = 0xFF00_u16 | u16::from(self.fetch_byte());
        self.registers.a = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        12
    }

    fn ld_ff00_na(&mut self) -> u8 {
        let address = 0xFF00_u16 | u16::from(self.fetch_byte());
        match self.mmu.write_byte(address as usize, self.registers.a) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        12
    }

    fn ld_a_ff00c(&mut self) -> u8 {
        let address = 0xFF00 | self.registers.c as u16;
        self.registers.a = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        8
    }

    fn ld_ff00_ca(&mut self) -> u8 {
        let address = 0xFF00 | self.registers.c as u16;
        match self.mmu.write_byte(address as usize, self.registers.a) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        8
    }

    fn ldd_a_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        self.registers.a = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.registers.set_hl(address - 1);

        8
    }

    fn ld_nn_sp(&mut self) -> u8 {
        let address = self.fetch_word();
        self.mmu
            .write_word(address as usize, self.registers.stack_pointer)
            .unwrap();

        20
    }

    fn ldi_a_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        self.registers.a = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.registers.set_hl(address + 1);

        8
    }

    fn ldi_hl_a(&mut self) -> u8 {
        let address = self.registers.hl();
        match self.mmu.write_byte(address as usize, self.registers.a) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        self.registers.set_hl(address + 1);

        8
    }

    fn ld_sp_hl(&mut self) -> u8 {
        self.registers.stack_pointer = self.registers.hl();

        4
    }

    // Logical OR n with register A, result in A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Reset.
    // C - Reset.
    fn alu_or(&mut self, n: u8) {
        let result = self.registers.a | n;
        self.registers.flags.carry = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn or_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        self.alu_or(v);

        4
    }

    fn or_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_or(v);

        8
    }

    fn or_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_or(v);

        8
    }

    // Compare A with n. This is basically an A - n subtraction instruction but the results are thrown away.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero. (Set if A = n.)
    // N - Set.
    // H - Set if no borrow from bit 4.
    // C - Set for no borrow. (Set if A < n.)
    fn alu_cp(&mut self, n: u8) {
        let result = self.registers.a;
        self.alu_sub(n);
        self.registers.a = result;
    }

    fn cp_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        self.alu_cp(v);

        4
    }

    fn cp_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_cp(v);

        8
    }

    // Logical exclusive OR n with register A, result in A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Reset.
    // C - Reset.
    fn alu_xor(&mut self, n: u8) {
        let result = self.registers.a ^ n;
        self.registers.flags.carry = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn xor_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        self.alu_xor(v);

        4
    }

    fn xor_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_xor(v);

        8
    }

    fn xor_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_xor(v);

        8
    }

    fn add_hl_rr(&mut self, reg: RegisterWord) -> u8 {
        let rr = self.registers.get_register_word(&reg);
        let hl = self.registers.hl();
        let result = hl + rr;
        self.registers.flags.zero = false;
        self.registers.flags.carry = (result & 0xFF) < (hl & 0xFF);
        self.registers.flags.half_carry = (result & 0xF) < (hl & 0xF);
        self.registers.flags.negative = false;
        self.registers.set_hl(result);

        8
    }

    fn inc_rr(&mut self, reg: RegisterWord) -> u8 {
        let rr = self.registers.get_register_word(&reg);
        self.registers.set_register_word(&reg, rr.wrapping_add(1));
        8
    }

    fn inc_r(&mut self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => &mut self.registers.b,
            Register::C => &mut self.registers.c,
            Register::D => &mut self.registers.d,
            Register::E => &mut self.registers.e,
            Register::H => &mut self.registers.h,
            Register::L => &mut self.registers.l,
            Register::A => &mut self.registers.a,
        };

        let ret = r.wrapping_add(1);
        self.registers.flags.zero = ret == 0;
        self.registers.flags.half_carry =
            CentralProcessingUnit::check_for_half_carry_first_nible_add(*r as u16, 1);
        self.registers.flags.negative = false;
        *r = ret;

        8
    }

    fn dec(&mut self, input: u8) -> u8 {
        let result = input.wrapping_sub(1);
        self.registers.flags.zero = result == 0;
        self.registers.flags.half_carry =
            CentralProcessingUnit::check_for_half_carry_first_nible_sub(input, result);
        self.registers.flags.negative = true;
        result
    }

    fn dec_rr(&mut self, reg: RegisterWord) -> u8 {
        let rr = self.registers.get_register_word(&reg);
        self.registers.set_register_word(&reg, rr.wrapping_sub(1));
        8
    }

    fn dec_r(&mut self, reg: Register) -> u8 {
        match reg {
            Register::B => self.registers.b = self.dec(self.registers.b),
            Register::C => self.registers.c = self.dec(self.registers.c),
            Register::D => self.registers.d = self.dec(self.registers.d),
            Register::E => self.registers.e = self.dec(self.registers.e),
            Register::H => self.registers.h = self.dec(self.registers.h),
            Register::L => self.registers.l = self.dec(self.registers.l),
            Register::A => self.registers.a = self.dec(self.registers.a),
        };

        4
    }

    fn dec_hl(&mut self) -> u8 {
        let addr = self.registers.hl() as usize;
        let result = self.dec(match self.mmu.read_byte(addr) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        });
        match self.mmu.write_byte(addr, result) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        12
    }

    fn jp_nn(&mut self) -> u8 {
        self.registers.program_counter = self.fetch_word();
        16
    }

    fn jp_hl(&mut self) -> u8 {
        self.registers.program_counter = self.registers.hl();
        4
    }

    fn jr_f_pc_dd(&mut self, op: ConditionOperand) -> u8 {
        let condition = match op {
            ConditionOperand::NZ => !self.registers.flags.zero,
            ConditionOperand::Z => self.registers.flags.zero,
            ConditionOperand::NC => !self.registers.flags.carry,
            ConditionOperand::C => self.registers.flags.carry,
        };

        if condition {
            let dd = self.fetch_byte();
            self.registers.program_counter += dd as u16;
            return 12;
        }

        8
    }

    fn jr_pc_dd(&mut self) -> u8 {
        self.registers.program_counter += self.fetch_byte() as u16;
        12
    }

    fn jp_f_nn(&mut self, op: ConditionOperand) -> u8 {
        let condition = match op {
            ConditionOperand::NZ => !self.registers.flags.zero,
            ConditionOperand::Z => self.registers.flags.zero,
            ConditionOperand::NC => !self.registers.flags.carry,
            ConditionOperand::C => self.registers.flags.carry,
        };

        if condition {
            self.registers.program_counter += self.fetch_word();
            return 16;
        }

        12
    }

    fn cp_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_cp(v);

        8
    }

    fn call_flag_nn(&mut self, operator: ConditionOperand) -> u8 {
        let nn = self.fetch_word();
        let flag = match operator {
            ConditionOperand::Z => self.registers.flags.zero,
            ConditionOperand::NZ => !self.registers.flags.zero,
            ConditionOperand::C => self.registers.flags.carry,
            ConditionOperand::NC => !self.registers.flags.carry,
        };

        if flag {
            let high = (self.registers.program_counter >> 8) as u8;
            self.registers.stack_pointer -= 1;
            match self
                .mmu
                .write_byte(self.registers.stack_pointer as usize, high)
            {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            }

            let low = (self.registers.program_counter & 0xFF) as u8;
            self.registers.stack_pointer -= 1;
            match self
                .mmu
                .write_byte(self.registers.stack_pointer as usize, low)
            {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            }

            self.registers.program_counter = nn;
            return 24;
        }

        12
    }

    fn di(&mut self) -> u8 {
        self.ime = false;
        4
    }

    fn call_nn(&mut self) -> u8 {
        let nn = self.fetch_word();
        let high = (self.registers.program_counter >> 8) as u8;
        self.registers.stack_pointer -= 1;
        match self
            .mmu
            .write_byte(self.registers.stack_pointer as usize, high)
        {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        let low = (self.registers.program_counter & 0xFF) as u8;
        self.registers.stack_pointer -= 1;
        match self
            .mmu
            .write_byte(self.registers.stack_pointer as usize, low)
        {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        self.registers.program_counter = nn;

        24
    }

    fn rst(&mut self, value: u8) -> u8 {
        self.push(self.registers.program_counter as u16);
        self.registers.program_counter = value as u16;
        16
    }

    fn rr_a(&mut self) -> u8 {
        let mut v = self.registers.a;
        self.rotate_right_through_carry(&mut v);
        self.registers.a = v;

        4
    }

    fn rlca(&mut self) -> u8 {
        let old_bit_zero_data = self.registers.a & 0x01 == 0x01;
        self.registers.a = self.registers.a.rotate_left(1);
        self.registers.flags.half_carry = false;
        self.registers.flags.negative = false;
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.carry = old_bit_zero_data;

        4
    }

    fn ld_hl_sp(&mut self) -> u8 {
        let a = self.registers.stack_pointer;
        let b = i16::from(self.fetch_byte() as i8) as u16;
        self.registers.flags.carry = (a & 0x00FF) + (b & 0x00FF) > 0x00FF;
        self.registers.flags.half_carry = (a & 0x000F) + (b & 0x000F) > 0x000F;
        self.registers.flags.negative = false;
        self.registers.flags.zero = false;
        self.registers.set_hl(a.wrapping_add(b));

        12
    }

    fn ret(&mut self) -> u8 {
        self.registers.program_counter =
            match self.mmu.read_word(self.registers.stack_pointer as usize) {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };

        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(2);

        16
    }

    fn ret_f(&mut self, flag: ConditionOperand) -> u8 {
        let condition = match flag {
            ConditionOperand::NZ => !self.registers.flags.zero,
            ConditionOperand::Z => self.registers.flags.zero,
            ConditionOperand::NC => !self.registers.flags.carry,
            ConditionOperand::C => self.registers.flags.carry,
        };

        if condition {
            self.ret();
            return 20;
        }

        8
    }

    fn push(&mut self, value: u16) {
        let high = (value >> 8) as u8;
        self.registers.stack_pointer -= 1;
        match self
            .mmu
            .write_byte(self.registers.stack_pointer as usize, high)
        {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }

        let low = (value & 0xFF) as u8;
        self.registers.stack_pointer -= 1;
        match self
            .mmu
            .write_byte(self.registers.stack_pointer as usize, low)
        {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    fn push_rr(&mut self, reg: RegisterWord) -> u8 {
        let v = self.registers.get_register_word(&reg);
        self.push(v);

        16
    }

    fn stack_pop(&mut self) -> u16 {
        let rr = match self.mmu.read_word(self.registers.stack_pointer as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.registers.stack_pointer += 2;
        rr
    }

    fn pop_rr(&mut self, reg: RegisterWord) -> u8 {
        let v = self.stack_pop();
        self.registers.set_register_word(&reg, v);

        12
    }

    fn add_a_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_add(v);

        8
    }

    fn adc_a_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_adc(v);

        8
    }

    // Add n to A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Set if carry from bit 3.
    // C - Set if carry from bit 7.
    fn alu_add(&mut self, n: u8) {
        let a = self.registers.a;
        let result = a.wrapping_add(n);
        self.registers.flags.negative = false;
        self.registers.flags.carry = u16::from(a) + u16::from(n) > 0xFF;
        self.registers.flags.half_carry = (a & 0x0F) + (n & 0x0F) > 0x0F;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn add_a_r(&mut self, reg: Register) -> u8 {
        self.alu_add(self.registers.get_register(reg));

        4
    }

    fn add_a_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_add(v);

        8
    }

    // Add n + Carry flag to A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Set if carry from bit 3.
    // C - Set if carry from bit 7.
    fn alu_adc(&mut self, n: u8) {
        let a = self.registers.a;
        let c = u8::from(self.registers.flags.carry);
        let result = a.wrapping_add(n).wrapping_add(c);
        self.registers.flags.carry = u16::from(a) + u16::from(n) + u16::from(c) > 0xFF;
        self.registers.flags.half_carry = (a & 0x0f) + (n & 0x0F) + (c & 0x0F) > 0x0F;
        self.registers.flags.negative = false;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn adc_r(&mut self, reg: Register) -> u8 {
        self.alu_adc(self.registers.get_register(reg));

        4
    }

    fn adc_a_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_adc(v);

        8
    }

    // Subtract n from A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Set.
    // H - Set if no borrow from bit 4.
    // C - Set if no borrow
    fn alu_sub(&mut self, n: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(n);
        self.registers.flags.carry = u16::from(a) < u16::from(n);
        self.registers.flags.half_carry = (a & 0x0F) < (n & 0x0F);
        self.registers.flags.negative = true;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn sub_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        self.alu_sub(v);

        4
    }

    fn sub_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_sub(v);

        8
    }

    fn sub_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_sub(v);

        8
    }

    // Subtract n + Carry flag from A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Set.
    // H - Set if no borrow from bit 4.
    // C - Set if no borrow.
    fn alu_sbc(&mut self, n: u8) {
        let a = self.registers.a;
        let c = u8::from(self.registers.flags.carry);
        let result = a.wrapping_sub(n).wrapping_sub(c);
        self.registers.flags.carry = u16::from(a) < u16::from(n) + u16::from(c);
        self.registers.flags.half_carry = (a & 0x0F) < (n & 0x0F) + c;
        self.registers.flags.negative = true;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn sbc_a_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        self.alu_sbc(v);

        4
    }

    fn sbc_a_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_sbc(v);

        8
    }

    fn sbc_a_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_sbc(v);

        8
    }

    // Logically AND n with A, result in A.
    // n = A,B,C,D,E,H,L,(HL),#
    //
    // Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Set.
    // C - Reset
    fn alu_and(&mut self, n: u8) {
        let result = self.registers.a & n;
        self.registers.flags.carry = false;
        self.registers.flags.half_carry = true;
        self.registers.flags.negative = false;
        self.registers.flags.zero = result == 0x00;
        self.registers.a = result;
    }

    fn and_r(&mut self, reg: Register) -> u8 {
        let v = self.registers.get_register(reg);
        self.alu_and(v);

        4
    }

    fn and_hl(&mut self) -> u8 {
        let address = self.registers.hl();
        let v = match self.mmu.read_byte(address as usize) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        self.alu_and(v);

        8
    }

    fn and_n(&mut self) -> u8 {
        let v = self.fetch_byte();
        self.alu_and(v);

        8
    }

    fn interpret_prefix(&mut self) -> u8 {
        let prefix_opcode = self.fetch_byte();
        match prefix_opcode.into() {
            PrefixOpCode::RlcB => self.rlc_r(Register::B),
            PrefixOpCode::RlcC => self.rlc_r(Register::C),
            PrefixOpCode::RlcD => self.rlc_r(Register::D),
            PrefixOpCode::RlcE => self.rlc_r(Register::E),
            PrefixOpCode::RlcH => self.rlc_r(Register::H),
            PrefixOpCode::RlcL => self.rlc_r(Register::L),
            PrefixOpCode::RlcA => self.rlc_r(Register::A),

            PrefixOpCode::SrlB => self.srl_r(Register::B),
            PrefixOpCode::SrlC => self.srl_r(Register::C),
            PrefixOpCode::SrlD => self.srl_r(Register::D),
            PrefixOpCode::SrlE => self.srl_r(Register::E),
            PrefixOpCode::SrlH => self.srl_r(Register::H),
            PrefixOpCode::SrlL => self.srl_r(Register::L),
            PrefixOpCode::SrlA => self.srl_r(Register::A),

            PrefixOpCode::RrB => self.rr_r(Register::B),
            PrefixOpCode::RrC => self.rr_r(Register::C),
            PrefixOpCode::RrD => self.rr_r(Register::D),
            PrefixOpCode::RrE => self.rr_r(Register::E),
            PrefixOpCode::RrH => self.rr_r(Register::H),
            PrefixOpCode::RrL => self.rr_r(Register::L),
            PrefixOpCode::RrA => self.rr_r(Register::A),

            PrefixOpCode::SwapB => self.swap_r(Register::B),
            PrefixOpCode::SwapC => self.swap_r(Register::C),
            PrefixOpCode::SwapD => self.swap_r(Register::D),
            PrefixOpCode::SwapE => self.swap_r(Register::E),
            PrefixOpCode::SwapH => self.swap_r(Register::H),
            PrefixOpCode::SwapL => self.swap_r(Register::L),
            PrefixOpCode::SwapA => self.swap_r(Register::A),
        }
    }

    fn rlc_r(&mut self, reg: Register) -> u8 {
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
        *r = (*r << 1) ^ sign;
        self.registers.flags.zero = *r == 0;
        self.registers.flags.negative = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = sign != 0;
        8
    }

    fn srl_r(&mut self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => &mut self.registers.b,
            Register::C => &mut self.registers.c,
            Register::D => &mut self.registers.d,
            Register::E => &mut self.registers.e,
            Register::H => &mut self.registers.h,
            Register::L => &mut self.registers.l,
            Register::A => &mut self.registers.a,
        };

        let c = *r & 1 != 0;
        *r >>= 1;
        self.registers.flags.zero = *r == 0;
        self.registers.flags.negative = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = c;

        8
    }

    fn rr_r(&mut self, reg: Register) -> u8 {
        let mut r = match reg {
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
            Register::A => self.registers.a,
        };

        self.rotate_right_through_carry(&mut r);
        self.registers.set_register(reg, r);

        8
    }

    fn swap_r(&mut self, reg: Register) -> u8 {
        let r = match reg {
            Register::B => &mut self.registers.b,
            Register::C => &mut self.registers.c,
            Register::D => &mut self.registers.d,
            Register::E => &mut self.registers.e,
            Register::H => &mut self.registers.h,
            Register::L => &mut self.registers.l,
            Register::A => &mut self.registers.a,
        };

        let result = (*r >> 4) | (*r << 4);
        self.registers.flags.zero = result == 0;
        self.registers.flags.negative = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = false;
        *r = result;

        8
    }

    fn rotate_right_through_carry(&mut self, value: &mut u8) {
        let temp = *value;
        *value >>= 1;
        if self.registers.flags.carry {
            *value |= 0x80;
        }

        self.registers.flags.zero = *value == 0;
        self.registers.flags.negative = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = (temp & 0x01) != 0;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::memory_device::ReadWrite;
    use crate::register::{ConditionOperand, Register, RegisterWord};

    use super::CentralProcessingUnit;

    struct MockDevice {
        bytes: HashMap<usize, u8>,
        words: HashMap<usize, u16>,
    }

    impl ReadWrite for MockDevice {
        fn contains(&self, address: usize) -> bool {
            let _ = address;
            true
        }

        fn read_byte(&self, address: usize) -> Result<u8, std::io::Error> {
            Ok(self.bytes[&address])
        }
        fn read_word(&self, address: usize) -> Result<u16, std::io::Error> {
            Ok(self.words[&address])
        }

        fn write_byte(&mut self, address: usize, value: u8) -> Result<(), std::io::Error> {
            dbg!(address, value);
            self.bytes.insert(address, value);
            Ok(())
        }
        fn write_word(&mut self, _address: usize, _value: u16) -> Result<(), std::io::Error> {
            unimplemented!()
        }
    }

    macro_rules! collection {
        ($($k:expr => $v:expr),* $(,)?) => {{
            use std::iter::{Iterator, IntoIterator};
            Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
        }};
    }

    #[test]
    fn verify_ld_r_next() {
        let mc = MockDevice {
            bytes: collection! { 256 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.ld_r_next(Register::A);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 10);
    }

    #[test]
    fn verify_ld_r_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.b = 9;
        let cycle = cpu.ld_r_r(Register::A, Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 9);
    }

    #[test]
    fn verify_ld_r_hl() {
        let mc = MockDevice {
            bytes: collection! { 44 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(44);
        let cycle = cpu.ld_r_hl(Register::B);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.b, 10);
    }

    #[test]
    fn verify_ld_hl_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(44);
        cpu.registers.b = 99;
        let cycle = cpu.ld_hl_r(Register::B);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.mmu.read_byte(44).unwrap(), 99);
    }

    #[test]
    fn verify_ld_a_rr() {
        let mc = MockDevice {
            bytes: collection! { 44 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_bc(44);
        let cycle = cpu.ld_a_rr(RegisterWord::BC);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 10);
    }

    #[test]
    fn verify_ld_a_nn() {
        let mc = MockDevice {
            bytes: collection! { 44 => 10 },
            words: collection! { 256 => 44 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.ld_a_nn();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 10);
    }

    #[test]
    fn verify_ld_rr_a() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.set_bc(11);
            cpu.registers.a = 99;
            let cycle = cpu.ld_rr_a(RegisterWord::BC);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.mmu.read_byte(11).unwrap(), 99);
        }
    }

    #[test]
    fn verify_ld_nn_a() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 256 => 44 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 99;
        let cycle = cpu.ld_nn_a();
        assert_eq!(cycle, 16);
        assert_eq!(cpu.mmu.read_byte(44).unwrap(), 99);
    }

    #[test]
    fn verify_ld_hl_next() {
        let mc = MockDevice {
            bytes: collection! { 256 => 94 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        let cycle = cpu.ld_hl_next();
        assert_eq!(cycle, 12);
        assert_eq!(cpu.mmu.read_byte(99).unwrap(), 94);
    }

    #[test]
    fn verify_ld_a_ff00_n() {
        let mc = MockDevice {
            bytes: collection! { 256 => 1, 0xFF01 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.ld_a_ff00_n();
        assert_eq!(cycle, 12);
        assert_eq!(cpu.registers.a, 10);
    }

    #[test]
    fn verify_ld_ff00_na() {
        let mc = MockDevice {
            bytes: collection! { 256 => 1 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 94;
        let cycle = cpu.ld_ff00_na();
        assert_eq!(cycle, 12);
        assert_eq!(cpu.mmu.read_byte(0xFF00 + 1).unwrap(), 94);
    }

    #[test]
    fn verify_ld_a_ff00c() {
        let mc = MockDevice {
            bytes: collection! { 0xFF02 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.c = 2;
        let cycle = cpu.ld_a_ff00c();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 10);
    }

    #[test]
    fn verify_ld_ff00_ca() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 10;
        cpu.registers.c = 2;
        let cycle = cpu.ld_ff00_ca();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.mmu.read_byte(0xFF02).unwrap(), 10);
    }

    #[test]
    fn verify_ldd_a_hl() {
        let mc = MockDevice {
            bytes: collection! { 88 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(88);
        let cycle = cpu.ldd_a_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 10);
        assert_eq!(cpu.registers.hl(), 87);
    }

    #[test]
    fn verify_ld_nn_sp() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 256 => 1000 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.ld_nn_sp();
        assert_eq!(cycle, 20);
        assert_eq!(cpu.mmu.read_byte(1000).unwrap(), 254);
        assert_eq!(cpu.mmu.read_byte(1001).unwrap(), 255);
    }

    #[test]
    fn verify_rlca() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 1;
            let cycle = cpu.rlca();
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 2);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, true);
        }
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 2;
            let cycle = cpu.rlca();
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 4);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_rr_a() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
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
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
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
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
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

    #[test]
    fn verify_call_nn() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 256 => 1000 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.stack_pointer = 2;
        let cycle = cpu.call_nn();
        assert_eq!(cycle, 24);
        assert_eq!(cpu.registers.program_counter, 1000);
        assert_eq!(cpu.mmu.read_byte(1).unwrap(), 1);
        assert_eq!(cpu.mmu.read_byte(0).unwrap(), 2);
    }

    #[test]
    fn verify_or_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 10;
        cpu.registers.b = 5;
        let cycle = cpu.or_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 15);
    }

    #[test]
    fn verify_xor_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 10;
        cpu.registers.b = 1;
        let cycle = cpu.xor_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 11);
    }

    #[test]
    fn verify_jr_pc_dd() {
        let mc = MockDevice {
            bytes: collection! { 256 => 99 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.jr_pc_dd();
        assert_eq!(cycle, 12);
        assert_eq!(cpu.registers.program_counter, 356);
    }

    #[test]
    fn verify_inc_r() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 99 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 15;
            let cycle = cpu.inc_r(Register::A);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 16);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 99 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 1;
            let cycle = cpu.inc_r(Register::A);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 2);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_dec_r() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 99 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 16;
            let cycle = cpu.dec_r(Register::A);
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 15);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, true);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 99 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 1;
            let cycle = cpu.dec_r(Register::A);
            assert_eq!(cycle, 4);
            assert_eq!(cpu.registers.a, 0);
            assert_eq!(cpu.registers.flags.zero, true);
            assert_eq!(cpu.registers.flags.negative, true);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_ret() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 65534 => 99 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.ret();
        assert_eq!(cycle, 16);
        assert_eq!(cpu.registers.program_counter, 99);
        assert_eq!(cpu.registers.stack_pointer, 0);
    }

    #[test]
    fn verify_ld_dd_nn() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 256 => 99 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.ld_dd_nn(RegisterWord::SP);
        assert_eq!(cycle, 12);
        assert_eq!(cpu.registers.stack_pointer, 99);
    }

    #[test]
    fn verify_push_qq() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_bc(10001);
        let cycle = cpu.push_rr(RegisterWord::BC);
        assert_eq!(cycle, 16);
        assert_eq!(cpu.registers.stack_pointer, 0xFFFE - 2);
        assert_eq!(cpu.mmu.read_byte(65533).unwrap(), 39);
        assert_eq!(cpu.mmu.read_byte(65532).unwrap(), 17);
    }

    #[test]
    fn verify_pop_qq() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 100 => 99 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.stack_pointer = 100;
        let cycle = cpu.pop_rr(RegisterWord::BC);
        assert_eq!(cycle, 12);
        assert_eq!(cpu.registers.bc(), 99);
        assert_eq!(cpu.registers.stack_pointer, 102);
    }

    #[test]
    fn verify_ldi_a_hl() {
        let mc = MockDevice {
            bytes: collection! { 99 => 8 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        let cycle = cpu.ldi_a_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 8);
        assert_eq!(cpu.registers.hl(), 100);
    }

    #[test]
    fn verify_sub_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 80;
        cpu.registers.b = 20;
        let cycle = cpu.sub_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 60);
        assert_eq!(cpu.registers.b, 20);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_and_n() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 2 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 1;
            let cycle = cpu.and_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 0);
            assert_eq!(cpu.registers.flags.zero, true);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 7 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 3;
            let cycle = cpu.and_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 3);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_call_flag_nn() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! { 256 => 1000 },
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.stack_pointer = 2;
            let cycle = cpu.call_flag_nn(ConditionOperand::Z);
            assert_eq!(cycle, 12);
            assert_eq!(cpu.registers.program_counter, 258);
        }
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! { 256 => 1000 },
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.stack_pointer = 2;
            cpu.registers.flags.zero = true;
            let cycle = cpu.call_flag_nn(ConditionOperand::Z);
            assert_eq!(cycle, 24);
            assert_eq!(cpu.registers.program_counter, 1000);
            assert_eq!(cpu.mmu.read_byte(1).unwrap(), 1);
            assert_eq!(cpu.mmu.read_byte(0).unwrap(), 2);
        }
    }

    #[test]
    fn verify_cp_n() {
        let mc = MockDevice {
            bytes: collection! { 256 => 99 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 100;
        let cycle = cpu.cp_n();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 100);
    }

    #[test]
    fn verify_ldi_hl_a() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(10);
        cpu.registers.a = 11;
        let cycle = cpu.ldi_hl_a();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.mmu.read_byte(10).unwrap(), 11);
        assert_eq!(cpu.registers.hl(), 11);
    }

    #[test]
    fn verify_ldd_hl_a() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(10);
        cpu.registers.a = 11;
        let cycle = cpu.ldd_hl_a();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.mmu.read_byte(10).unwrap(), 11);
        assert_eq!(cpu.registers.hl(), 9);
    }

    #[test]
    fn verify_add_a_n() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 10 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 11;
            let cycle = cpu.add_a_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 21);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 2 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = u8::MAX - 2;
            let cycle = cpu.add_a_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 255);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_sub_n() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 10 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 80;
            let cycle = cpu.sub_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 70);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, true);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 80 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 80;
            let cycle = cpu.sub_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 0);
            assert_eq!(cpu.registers.flags.zero, true);
            assert_eq!(cpu.registers.flags.negative, true);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_xor_hl() {
        let mc = MockDevice {
            bytes: collection! { 100 => 90 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(100);
        cpu.registers.a = 1;
        let cycle = cpu.xor_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 91);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_rlc_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.c = 3;
        let cycle = cpu.rlc_r(Register::C);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.c, 6);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_srl_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.c = 3;
        let cycle = cpu.srl_r(Register::C);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.c, 1);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, true);
    }

    #[test]
    fn verify_rr_r() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.b = 3;
            let cycle = cpu.rr_r(Register::B);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.b, 1);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, true);
        }
    }

    #[test]
    fn verify_xor_n() {
        let mc = MockDevice {
            bytes: collection! { 256 => 10 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 1;
        let cycle = cpu.xor_n();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 11);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_adca_n() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 9 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 1;
            let cycle = cpu.adc_a_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 10);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 1 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 0xFF;
            let cycle = cpu.adc_a_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 0);
            assert_eq!(cpu.registers.flags.zero, true);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, true);
            assert_eq!(cpu.registers.flags.carry, true);
        }
    }

    #[test]
    fn verify_ret_f() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! { 65534 => 99 },
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            let cycle = cpu.ret_f(ConditionOperand::NZ);
            assert_eq!(cycle, 20);
            assert_eq!(cpu.registers.stack_pointer, 0);
            assert_eq!(cpu.registers.program_counter, 99);
        }
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! { 65534 => 99 },
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            let cycle = cpu.ret_f(ConditionOperand::Z);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.stack_pointer, 65534);
            assert_eq!(cpu.registers.program_counter, 256);
        }
    }

    #[test]
    fn verify_or_hl() {
        let mc = MockDevice {
            bytes: collection! { 33 => 5},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 10;
        cpu.registers.set_hl(33);
        let cycle = cpu.or_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 15);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_dec_hl() {
        let mc = MockDevice {
            bytes: collection! { 16 => 99 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(16);
        let cycle = cpu.dec_hl();
        assert_eq!(cycle, 12);
        assert_eq!(cpu.mmu.read_byte(16).unwrap(), 98);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_add_hl_rr() {
        let mc = MockDevice {
            bytes: collection! { 16 => 99 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(10);
        cpu.registers.set_bc(100);
        let cycle = cpu.add_hl_rr(RegisterWord::BC);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.hl(), 110);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_jp_nn() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! { 256 => 99 },
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.jp_nn();
        assert_eq!(cycle, 16);
        assert_eq!(cpu.registers.program_counter, 99);
    }

    #[test]
    fn verify_jp_hl() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(10);
        let cycle = cpu.jp_hl();
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.hl(), 10);
        assert_eq!(cpu.registers.program_counter, 10);
    }

    #[test]
    fn verify_swap_r() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 5;
            cpu.registers.b = 10;
            let cycle = cpu.swap_r(Register::B);
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 5);
            assert_eq!(cpu.registers.b, 160);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_or_n() {
        {
            let mc = MockDevice {
                bytes: collection! { 256 => 1 },
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            cpu.registers.a = 10;
            let cycle = cpu.or_n();
            assert_eq!(cycle, 8);
            assert_eq!(cpu.registers.a, 11);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_jp_f_nn() {
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! { 256 => 300 },
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            let cycle = cpu.jp_f_nn(ConditionOperand::NZ);
            assert_eq!(cycle, 16);
            assert_eq!(cpu.registers.program_counter, 558);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
        {
            let mc = MockDevice {
                bytes: collection! {},
                words: collection! {},
            };
            let mut cpu = CentralProcessingUnit::new(Box::new(mc));
            let cycle = cpu.jp_f_nn(ConditionOperand::Z);
            assert_eq!(cycle, 12);
            assert_eq!(cpu.registers.program_counter, 256);
            assert_eq!(cpu.registers.flags.zero, false);
            assert_eq!(cpu.registers.flags.negative, false);
            assert_eq!(cpu.registers.flags.half_carry, false);
            assert_eq!(cpu.registers.flags.carry, false);
        }
    }

    #[test]
    fn verify_add_a_r() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 5;
        cpu.registers.b = 10;
        let cycle = cpu.add_a_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 15);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_ld_hl_sp() {
        let mc = MockDevice {
            bytes: collection! { 256 => 20 },
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        dbg!(cpu.registers.stack_pointer);
        let cycle = cpu.ld_hl_sp();
        assert_eq!(cycle, 12);
        assert_eq!(cpu.registers.stack_pointer, 65534);
        assert_eq!(cpu.registers.hl(), 18);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, true);
    }

    #[test]
    fn verify_dec_rr() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_de(100);
        let cycle = cpu.dec_rr(RegisterWord::DE);
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.de(), 99);
    }

    #[test]
    fn verify_rst() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };
        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        let cycle = cpu.rst(0x08);
        assert_eq!(cycle, 16);
        assert_eq!(cpu.registers.program_counter, 0x08);
        assert_eq!(cpu.mmu.read_byte(65533).unwrap(), 1);
        assert_eq!(cpu.mmu.read_byte(65532).unwrap(), 0);
    }

    #[test]
    fn verify_ld_sp_hl() {
        let mc = MockDevice {
            bytes: collection! {},
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        let cycle = cpu.ld_sp_hl();
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.stack_pointer, 99);
    }

    #[test]
    fn verify_add_a_hl() {
        let mc = MockDevice {
            bytes: collection! { 99 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        cpu.registers.a = 1;
        let cycle = cpu.add_a_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 11);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_adc_r() {
        let mc = MockDevice {
            bytes: collection! { 99 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.flags.carry = true;
        cpu.registers.a = 11;
        cpu.registers.b = 5;
        let cycle = cpu.adc_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 17);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_adc_a_hl() {
        let mc = MockDevice {
            bytes: collection! { 99 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        let cycle = cpu.adc_a_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 11);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_sub_hl() {
        let mc = MockDevice {
            bytes: collection! { 99 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        cpu.registers.a = 25;
        let cycle = cpu.sub_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 15);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_sbc_a_r() {
        let mc = MockDevice {
            bytes: collection! { 99 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 35;
        cpu.registers.b = 25;
        cpu.registers.flags.carry = true;
        let cycle = cpu.sbc_a_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 9);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_sbc_a_hl() {
        let mc = MockDevice {
            bytes: collection! { 99 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.set_hl(99);
        cpu.registers.flags.carry = true;
        let cycle = cpu.sbc_a_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 246);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, true);
    }

    #[test]
    fn verify_sbc_a_n() {
        let mc = MockDevice {
            bytes: collection! { 256 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.flags.carry = true;
        let cycle = cpu.sbc_a_n();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 246);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, true);
    }

    #[test]
    fn verify_and_r() {
        let mc = MockDevice {
            bytes: collection! { 256 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 64;
        cpu.registers.b = 32;
        let cycle = cpu.and_r(Register::B);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.flags.zero, true);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_and_hl() {
        let mc = MockDevice {
            bytes: collection! { 32 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 64;
        cpu.registers.set_hl(32);
        let cycle = cpu.and_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.flags.zero, true);
        assert_eq!(cpu.registers.flags.negative, false);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_cp_r() {
        let mc = MockDevice {
            bytes: collection! { 32 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 64;
        cpu.registers.c = 60;
        let cycle = cpu.cp_r(Register::C);
        assert_eq!(cycle, 4);
        assert_eq!(cpu.registers.a, 64);
        assert_eq!(cpu.registers.c, 60);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, false);
    }

    #[test]
    fn verify_cp_hl() {
        let mc = MockDevice {
            bytes: collection! { 50 => 10 },
            words: collection! {},
        };

        let mut cpu = CentralProcessingUnit::new(Box::new(mc));
        cpu.registers.a = 1;
        cpu.registers.set_hl(50);
        let cycle = cpu.cp_hl();
        assert_eq!(cycle, 8);
        assert_eq!(cpu.registers.a, 1);
        assert_eq!(cpu.registers.hl(), 50);
        assert_eq!(cpu.registers.flags.zero, false);
        assert_eq!(cpu.registers.flags.negative, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
        assert_eq!(cpu.registers.flags.carry, true);
    }
}
