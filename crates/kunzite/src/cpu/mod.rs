//! The cpu

pub use self::register::Flag;
use self::register::Registers;
use crate::{cpu::instruction::Register16, memory::Memory};

mod decode;
pub mod instruction;
mod register;

/// The cpu
pub struct Cpu {
	registers: Registers,
	halted: bool,
	interrupt_enabled: bool,
	instruction_cycle: i32,
	is_cgb: bool,
	pending_enable_interrupts: i32,
	pending_disable_interrupts: i32,
}

impl Cpu {
	pub fn new(is_cgb: bool) -> Self {
		let mut registers = Registers::default();

		registers.set_af(0x01B0);
		registers.set_bc(0x0013);
		registers.set_de(0x00D8);
		registers.set_hl(0x014D);
		registers.pc = 0x0100;
		registers.sp = 0xFFFE;

		Self {
			registers,
			halted: false,
			interrupt_enabled: false,
			instruction_cycle: 0,
			is_cgb,
			pending_enable_interrupts: -1,
			pending_disable_interrupts: -1,
		}
	}

	pub fn step(&mut self, memory: &mut Memory) -> i32 {
		self.instruction_cycle = 0;

		if !self.halted {
			// TODO: handle interrupts

			let opcode = self.fetch(memory, false);
			self.execute_new(opcode, memory);
		} else {
			panic!("Support halts")
		}

		self.instruction_cycle
	}

	pub fn pc(&self) -> &u16 {
		&self.registers.pc
	}

	pub fn regs(&self) -> &Registers {
		&self.registers
	}

	fn execute_new(&mut self, opcode: u8, memory: &mut Memory) {
		let reg = opcode & 7;
		let reg2 = opcode >> 3 & 7;

		match opcode {
			// NOP
			0x00 => self.nop(),

			// LD r16, u16
			0x01 | 0x11 | 0x21 | 0x31 => self.ld_r16_nn(Self::reg16(opcode >> 4), memory),

			// LD (u16), SP
			0x08 => {
				let val = self.get_nn(memory);
				self.ld_nn_sp(val)
			}

			// LD SP, HL
			0xF9 => self.ld_sp_hl(),

			// LD (r16), A
			0x02 => self.ld_addr(self.registers.get_bc(), true),
			0x12 => self.ld_addr(self.registers.get_de(), true),

			// LD A, (r16)
			0x0A => self.ld_addr(self.registers.get_bc(), false),
			0x1A => self.ld_addr(self.registers.get_de(), false),

			// PUSH r16
			0xC5 => self.push(self.registers.get_bc(), memory),
			0xD5 => self.push(self.registers.get_de(), memory),
			0xE5 => self.push(self.registers.get_hl(), memory),
			0xF5 => self.push(self.registers.get_af(), memory),

			// POP r16
			0xC1 => {
				let val = self.pop(memory);
				self.registers.set_bc(val)
			}
			0xD1 => {
				let val = self.pop(memory);
				self.registers.set_de(val)
			}
			0xE1 => {
				let val = self.pop(memory);
				self.registers.set_hl(val)
			}
			0xF1 => {
				let val = self.pop(memory);
				self.registers.set_af(val)
			}

			// Conditional absolute jump
			0xC2 => self.jp_cc_nn(Flag::NOT_ZERO, memory),
			0xD2 => self.jp_cc_nn(Flag::NOT_FULL_CARRY, memory),
			0xCA => self.jp_cc_nn(Flag::ZERO, memory),
			0xDA => self.jp_cc_nn(Flag::FULL_CARRY, memory),

			// Unconditional absolute jump
			0xC3 => self.jp_nn(memory),
			0xE9 => self.jp_hl(),

			// Conditional relative jump
			0x20 => self.jr_cc_d(Flag::NOT_ZERO, memory),
			0x30 => self.jr_cc_d(Flag::NOT_FULL_CARRY, memory),
			0x28 => self.jr_cc_d(Flag::ZERO, memory),
			0x38 => self.jr_cc_d(Flag::FULL_CARRY, memory),

			// Unconditional relative jump
			0x18 => self.jr_d(),

			// Bit rotate on A
			0x07 => self.rlca(),
			0x17 => self.rla(),
			0x0F => self.rrca(),
			0x1F => self.rra(),

			// Arithmethic/logical operation on 16-bit register
			0x09 | 0x19 | 0x29 | 0x39 => self.add_hl_r16(opcode >> 4),
			0xE8 => self.add_sp_d(memory),
			0xF8 => self.ld_hl_sp_d(memory),

			// Arithmethic/logical operation on 8-bit register
			0x80..=0x87 => self.add_r8(opcode & 7),
			0x88..=0x8F => self.adc_r8(opcode & 7),
			0x90..=0x97 => self.sub_r8(opcode & 7),
			0x98..=0x9F => self.sbc_r8(opcode & 7),
			0xA0..=0xA7 => self.and_r8(opcode & 7),
			0xA8..=0xAF => self.xor_r8(opcode & 7),
			0xB0..=0xB7 => self.or_r8(opcode & 7),
			0xB8..=0xBF => self.cp_r8(opcode & 7),

			// DAA
			0x27 => self.daa(),

			// CPL
			0x2F => self.cpl(),

			// SCF, CCF
			0x37 => self.scf(),
			0x3F => self.ccf(),

			// Arithmethic/logical operation on A
			0xC6 => self.add_d8(),
			0xD6 => self.sub_d8(),
			0xE6 => self.and_d8(),
			0xF6 => self.or_d8(),
			0xCE => self.adc_d8(),
			0xDE => self.sbc_d8(),
			0xEE => self.xor_d8(),
			0xFE => self.cp_d8(),

			// LDI, LDD
			0x22 => self.ld_hl_id(false, true),
			0x32 => self.ld_hl_id(false, false),
			0x2A => self.ld_hl_id(true, true),
			0x3A => self.ld_hl_id(true, false),

			// LD IO port
			0xE0 => self.ld_io_n(false),
			0xF0 => self.ld_io_n(true),
			0xE2 => self.ld_io_c(false),
			0xF2 => self.ld_io_c(true),

			// INC r8
			0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => self.inc_r8(reg2),

			// DEC r8
			0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => self.dec_r8(reg2),

			// LD r8, d8
			0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => self.ld_r8_d8(reg2),

			// LD r8, r8
			0x40..=0x75 | 0x77..=0x7F => self.mov8(reg2, reg),

			// LD (d16), A
			0xEA => self.ld_nn(false),

			// LD A, (d16)
			0xFA => self.ld_nn(false),

			// INC, DEC r16
			0x03 | 0x13 | 0x23 | 0x33 => self.inc_r16(opcode >> 4),
			0x0B | 0x1B | 0x2B | 0x3B => self.dec_r16(opcode >> 4),

			// Unconditional call
			0xCD => self.call_nn(memory),

			// Conditional call
			0xC4 | 0xD4 | 0xCC | 0xDC => self.call_cc_d16(Self::flag(reg2), memory),

			// Unconditional ret
			0xC9 => self.ret(),

			// Conditional ret
			0xC0 | 0xD0 | 0xC8 | 0xD8 => self.ret_cc(reg2),

			// RETI
			0xD9 => self.reti(),

			// RST
			0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst(opcode - 0xC7),

			// DI, EI
			0xF3 => self.di(),
			0xFB => self.ei(),

			// CB prefixed
			0xCB => self.prefix(),

			// HALT
			0x76 => self.halt(),

			// STOP
			0x10 => self.stop(memory),

			// Unused
			0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
				panic!("unused opcode: {:02X}", opcode)
			}
		}
	}

	fn nop(&self) {}

	fn ld_r16_nn(&mut self, reg: Register16, memory: &mut Memory) {
		let val = self.get_nn(memory);
		match reg {
			Register16::BC => self.registers.set_bc(val),
			Register16::DE => self.registers.set_de(val),
			Register16::HL => self.registers.set_hl(val),
			Register16::SP => self.registers.sp = val,
			_ => unreachable!(),
		}
	}

	fn ld_nn_sp(&self, val: u16) {
		todo!()
	}

	fn ld_sp_hl(&self) {
		todo!()
	}

	fn ld_addr(&self, addr: u16, arg: bool) {
		todo!()
	}

	fn push(&mut self, nn: u16, memory: &mut Memory) {
		self.registers.sp -= 2;
		memory.write_word(self.registers.sp, nn)
	}

	fn pop(&mut self, memory: &mut Memory) -> u16 {
		let word = memory.read_word(self.registers.sp);
		self.registers.sp += 2;

		word
	}

	fn jp_cc_nn(&mut self, flag: Flag, memory: &mut Memory) {
		let addr = self.get_nn(memory);
		todo!()
	}

	fn jp_nn(&mut self, memory: &mut Memory) {
		let addr = self.get_nn(memory);
		self.registers.pc = addr;
	}

	fn jp_hl(&mut self) {
		let addr = self.registers.get_hl();
		self.registers.pc = addr;
	}

	fn jr_cc_d(&mut self, flag: Flag, memory: &mut Memory) {
		let jr = self.get_d(memory);
		todo!()
	}

	fn jr_d(&self) {
		todo!()
	}

	fn rlca(&self) {
		todo!()
	}

	fn rla(&self) {
		todo!()
	}

	fn rrca(&self) {
		todo!()
	}

	fn rra(&self) {
		todo!()
	}

	fn add_hl_r16(&self, opcode: u8) {
		todo!()
	}

	fn add_sp_d(&mut self, memory: &mut Memory) {
		let d = self.get_d(memory);
		todo!()
	}

	fn ld_hl_sp_d(&mut self, memory: &mut Memory) {
		let d = self.get_d(memory);
		todo!()
	}

	fn add_r8(&self, opcode: u8) {
		todo!()
	}

	fn adc_r8(&self, opcode: u8) {
		todo!()
	}

	fn sub_r8(&self, opcode: u8) {
		todo!()
	}

	fn sbc_r8(&self, opcode: u8) {
		todo!()
	}

	fn and_r8(&self, opcode: u8) {
		todo!()
	}

	fn or_r8(&self, opcode: u8) {
		todo!()
	}

	fn xor_r8(&self, opcode: u8) {
		todo!()
	}

	fn cp_r8(&self, opcode: u8) {
		todo!()
	}

	fn daa(&self) {
		todo!()
	}

	fn cpl(&self) {
		todo!()
	}

	fn scf(&self) {
		todo!()
	}

	fn ccf(&self) {
		todo!()
	}

	fn add_d8(&self) {
		todo!()
	}

	fn adc_d8(&self) {
		todo!()
	}

	fn sub_d8(&self) {
		todo!()
	}

	fn sbc_d8(&self) {
		todo!()
	}

	fn and_d8(&self) {
		todo!()
	}

	fn or_d8(&self) {
		todo!()
	}

	fn xor_d8(&self) {
		todo!()
	}

	fn cp_d8(&self) {
		todo!()
	}

	fn ld_hl_id(&self, ldstore: bool, incdec: bool) {
		todo!()
	}

	fn ld_r8_d8(&self, reg2: u8) {
		todo!()
	}

	fn inc_r8(&self, reg2: u8) {
		todo!()
	}

	fn dec_r8(&self, reg2: u8) {
		todo!()
	}

	fn mov8(&self, reg2: u8, reg: u8) {
		todo!()
	}

	fn ld_nn(&self, ldstore: bool) {
		todo!()
	}

	fn inc_r16(&self, opcode: u8) {
		todo!()
	}

	fn dec_r16(&self, opcode: u8) {
		todo!()
	}

	fn call_nn(&mut self, memory: &mut Memory) {
		let nn = self.get_nn(memory);
		self.push(self.registers.pc, memory);
		self.registers.pc = nn;
	}

	fn call_cc_d16(&mut self, flag: Flag, memory: &mut Memory) {
		if self.registers.f.contains(flag) {
			self.call_nn(memory);
			self.instruction_cycle = 24;
		}
	}

	fn ret(&self) {
		todo!()
	}

	fn ret_cc(&self, reg2: u8) {
		todo!()
	}

	fn reti(&self) {
		todo!()
	}

	fn di(&mut self) {
		self.pending_enable_interrupts = -1;
		if self.pending_disable_interrupts == -1 {
			self.pending_disable_interrupts = 1;
		}
	}

	fn ei(&mut self) {
		self.pending_disable_interrupts = -1;
		if self.pending_enable_interrupts == -1 {
			self.pending_enable_interrupts = 1;
		}
	}

	fn rst(&self, opcode: u8) {
		todo!()
	}

	fn prefix(&self) {
		todo!()
	}

	fn halt(&self) {
		todo!()
	}

	fn ld_io_n(&self, ldstore: bool) {
		todo!()
	}

	fn ld_io_c(&self, ldstore: bool) {
		todo!()
	}

	fn stop(&self, memory: &mut Memory) {
		todo!()
	}
}
