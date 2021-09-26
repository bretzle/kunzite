//! The cpu

pub use self::register::Flag;
use self::{instruction::Register8, register::Registers};
use crate::{cpu::instruction::Register16, memory::Memory, util::is_set};

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
			0x02 => self.ld_addr(self.registers.get_bc(), true, memory),
			0x12 => self.ld_addr(self.registers.get_de(), true, memory),

			// LD A, (r16)
			0x0A => self.ld_addr(self.registers.get_bc(), false, memory),
			0x1A => self.ld_addr(self.registers.get_de(), false, memory),

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
			0x18 => self.jr_d(memory),

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
			0x80..=0x87 => self.add_r8(Self::reg8(opcode & 7), memory),
			0x88..=0x8F => self.adc_r8(Self::reg8(opcode & 7), memory),
			0x90..=0x97 => self.sub_r8(Self::reg8(opcode & 7), memory),
			0x98..=0x9F => self.sbc_r8(Self::reg8(opcode & 7), memory),
			0xA0..=0xA7 => self.and_r8(Self::reg8(opcode & 7), memory),
			0xA8..=0xAF => self.xor_r8(Self::reg8(opcode & 7), memory),
			0xB0..=0xB7 => self.or_r8(Self::reg8(opcode & 7), memory),
			0xB8..=0xBF => self.cp_r8(Self::reg8(opcode & 7), memory),

			// DAA
			0x27 => self.daa(),

			// CPL
			0x2F => self.cpl(),

			// SCF, CCF
			0x37 => self.scf(),
			0x3F => self.ccf(),

			// Arithmethic/logical operation on A
			0xC6 => self.add_d8(memory),
			0xD6 => self.sub_d8(memory),
			0xE6 => self.and_d8(memory),
			0xF6 => self.or_d8(memory),
			0xCE => self.adc_d8(memory),
			0xDE => self.sbc_d8(memory),
			0xEE => self.xor_d8(memory),
			0xFE => self.cp_d8(memory),

			// LDI, LDD
			0x22 => self.ld_hl_id(false, true, memory),
			0x32 => self.ld_hl_id(false, false, memory),
			0x2A => self.ld_hl_id(true, true, memory),
			0x3A => self.ld_hl_id(true, false, memory),

			// LD IO port
			0xE0 => self.ld_io_n(false, memory),
			0xF0 => self.ld_io_n(true, memory),
			0xE2 => self.ld_io_c(false, memory),
			0xF2 => self.ld_io_c(true, memory),

			// INC r8
			0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x34 | 0x3C => {
				self.inc_r8(Self::reg8(reg2), memory)
			}

			// DEC r8
			0x05 | 0x0D | 0x15 | 0x1D | 0x25 | 0x2D | 0x35 | 0x3D => {
				self.dec_r8(Self::reg8(reg2), memory)
			}

			// LD r8, d8
			0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
				self.ld_r8_d8(Self::reg8(reg2), memory)
			}

			// LD r8, r8
			0x40..=0x75 | 0x77..=0x7F => self.mov8(Self::reg8(reg2), Self::reg8(reg), memory),

			// LD (d16), A
			0xEA => self.ld_nn(false, memory),

			// LD A, (d16)
			0xFA => self.ld_nn(true, memory),

			// INC, DEC r16
			0x03 | 0x13 | 0x23 | 0x33 => self.inc_r16(Self::reg16(opcode >> 4)),
			0x0B | 0x1B | 0x2B | 0x3B => self.dec_r16(Self::reg16(opcode >> 4)),

			// Unconditional call
			0xCD => self.call_nn(memory),

			// Conditional call
			0xC4 | 0xD4 | 0xCC | 0xDC => self.call_cc_d16(Self::flag(reg2), memory),

			// Unconditional ret
			0xC9 => self.ret(memory),

			// Conditional ret
			0xC0 | 0xD0 | 0xC8 | 0xD8 => self.ret_cc(Self::flag(reg2), memory),

			// RETI
			0xD9 => self.reti(),

			// RST
			0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst(reg2 * 8, memory),

			// DI, EI
			0xF3 => self.di(),
			0xFB => self.ei(),

			// CB prefixed
			0xCB => {
				let opcode = self.get_n(memory);
				let pos = opcode >> 3 & 0x7;
				let reg = Self::reg8(opcode & 0x7);

				match opcode {
					0x00..=0x07 => self.rlc(reg),
					0x08..=0x0f => self.rrc(reg),
					0x10..=0x17 => self.rl(reg),
					0x18..=0x1f => self.rr(reg),
					0x20..=0x27 => self.sla(reg),
					0x28..=0x2f => self.sra(reg),
					0x30..=0x37 => self.swap(reg),
					0x38..=0x3f => self.srl(reg),
					0x40..=0x7f => self.bit(pos, self.read_reg(reg, memory)),
					0x80..=0xbf => self.res(pos, reg),
					0xc0..=0xff => self.set(pos, reg),
				}
			}

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

	fn ld_addr(&mut self, addr: u16, ldstore: bool, memory: &mut Memory) {
		if ldstore {
			self.registers.a = memory.read_byte(addr);
		} else {
			memory.write_byte(addr, self.registers.a);
		}
	}

	fn push(&mut self, nn: u16, memory: &mut Memory) {
		self.registers.sp = self.registers.sp.wrapping_sub(2);
		memory.write_word(self.registers.sp, nn)
	}

	fn pop(&mut self, memory: &mut Memory) -> u16 {
		let word = memory.read_word(self.registers.sp);
		self.registers.sp = self.registers.sp.wrapping_add(2);

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
		if self.registers.f.contains(flag) {
			let jr = self.get_d(memory);
			self.registers.pc = self.registers.pc.wrapping_add(i16::from(jr) as u16);
			self.instruction_cycle = 12;
		}
	}

	fn jr_d(&mut self, memory: &mut Memory) {
		let d = self.get_d(memory);
		self.registers.pc = self.registers.pc.wrapping_add(d as u16);
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

	fn add_r8(&mut self, reg: Register8, memory: &Memory) {
		self.add(self.read_reg(reg, memory))
	}

	fn adc_r8(&mut self, reg: Register8, memory: &Memory) {
		self.adc(self.read_reg(reg, memory))
	}

	fn sub_r8(&mut self, reg: Register8, memory: &Memory) {
		self.sub(self.read_reg(reg, memory))
	}

	fn sbc_r8(&mut self, reg: Register8, memory: &Memory) {
		self.sbc(self.read_reg(reg, memory))
	}

	fn and_r8(&mut self, reg: Register8, memory: &Memory) {
		self.and(self.read_reg(reg, memory))
	}

	fn or_r8(&mut self, reg: Register8, memory: &Memory) {
		self.or(self.read_reg(reg, memory))
	}

	fn xor_r8(&mut self, reg: Register8, memory: &Memory) {
		self.xor(self.read_reg(reg, memory))
	}

	fn cp_r8(&mut self, reg: Register8, memory: &Memory) {
		self.cp(self.read_reg(reg, memory))
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

	fn add_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.add(n)
	}

	fn adc_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.adc(n)
	}

	fn sub_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.sub(n)
	}

	fn sbc_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.sbc(n)
	}

	fn and_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.and(n)
	}

	fn or_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.or(n)
	}

	fn xor_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.xor(n)
	}

	fn cp_d8(&mut self, memory: &mut Memory) {
		let n = self.get_n(memory);
		self.cp(n)
	}

	fn sub(&mut self, val: u8) {
		let half_carry = (self.registers.a & 0xF) < (val & 0xF);
		let full_carry = self.registers.a < val;
		let zero = self.registers.a == val;

		self.registers.a = self.registers.a.wrapping_sub(val);

		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.set(Flag::FULL_CARRY, full_carry);
		self.registers.f.set(Flag::ZERO, zero);
		self.registers.f.insert(Flag::NEGATIVE);
	}

	fn ld_hl_id(&mut self, ldstore: bool, incdec: bool, memory: &mut Memory) {
		let hl = self.registers.get_hl();

		if ldstore {
			self.registers.a = memory.read_byte(hl);
		} else {
			memory.write_byte(hl, self.registers.a);
		}

		let hl = if incdec {
			hl.wrapping_add(1)
		} else {
			hl.wrapping_sub(1)
		};

		self.registers.set_hl(hl);
	}

	fn ld_r8_d8(&mut self, reg: Register8, memory: &mut Memory) {
		let d = self.get_n(memory);
		self.write_reg(reg, d, memory);
	}

	fn inc_r8(&mut self, reg: Register8, memory: &mut Memory) {
		let val = self.read_reg(reg, memory);
		let half_carry = (((val & 0xF) + 1) & 0x10) == 0x10;

		let val = val.wrapping_add(1);

		self.registers.f.set(Flag::ZERO, val == 0);
		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.remove(Flag::NEGATIVE);

		self.write_reg(reg, val, memory);
	}

	fn dec_r8(&mut self, reg: Register8, memory: &mut Memory) {
		let val = self.read_reg(reg, memory);
		let half_carry = (val & 0xF) < 1;

		let val = val.wrapping_sub(1);

		self.registers.f.set(Flag::ZERO, val == 0);
		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.insert(Flag::NEGATIVE);

		self.write_reg(reg, val, memory);
	}

	fn mov8(&mut self, dest: Register8, src: Register8, memory: &mut Memory) {
		let val = self.read_reg(src, memory);
		self.write_reg(dest, val, memory)
	}

	fn ld_nn(&mut self, ldstore: bool, memory: &mut Memory) {
		let nn = self.get_nn(memory);
		if ldstore {
			// load
			self.registers.a = memory.read_byte(nn);
		} else {
			// store
			memory.write_byte(nn, self.registers.a)
		}
	}

	fn inc_r16(&mut self, reg: Register16) {
		match reg {
			Register16::BC => self
				.registers
				.set_bc(self.registers.get_bc().wrapping_add(1)),
			Register16::DE => self
				.registers
				.set_de(self.registers.get_de().wrapping_add(1)),
			Register16::HL => self
				.registers
				.set_hl(self.registers.get_hl().wrapping_add(1)),
			Register16::SP => self.registers.sp = self.registers.sp.wrapping_add(1),
			Register16::AF => unreachable!(),
		}
	}

	fn dec_r16(&mut self, reg: Register16) {
		match reg {
			Register16::BC => self
				.registers
				.set_bc(self.registers.get_bc().wrapping_sub(1)),
			Register16::DE => self
				.registers
				.set_de(self.registers.get_de().wrapping_sub(1)),
			Register16::HL => self
				.registers
				.set_hl(self.registers.get_hl().wrapping_sub(1)),
			Register16::SP => self.registers.sp = self.registers.sp.wrapping_sub(1),
			Register16::AF => unreachable!(),
		}
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

	fn ret(&mut self, memory: &mut Memory) {
		self.registers.pc = self.pop(memory);
		self.instruction_cycle = 16;
	}

	fn ret_cc(&mut self, flag: Flag, memory: &mut Memory) {
		if self.registers.f.contains(flag) {
			self.registers.pc = self.pop(memory);
			self.instruction_cycle = 20;
		}
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

	fn rst(&mut self, addr: u8, memory: &mut Memory) {
		self.push(self.registers.pc, memory);
		self.registers.pc = addr as u16;
	}

	fn halt(&self) {
		todo!()
	}

	fn ld_io_n(&mut self, ldstore: bool, memory: &mut Memory) {
		let addr = self.get_n(memory) as u16 + 0xFF00;

		if ldstore {
			// load
			self.registers.a = memory.read_byte(addr);
		} else {
			// store
			memory.write_byte(addr, self.registers.a);
		}
	}

	fn ld_io_c(&mut self, ldstore: bool, memory: &mut Memory) {
		let addr = self.registers.c as u16 + 0xFF00;

		if ldstore {
			self.registers.a = memory.read_byte(addr);
		} else {
			memory.write_byte(addr, self.registers.a);
		}
	}

	fn stop(&self, memory: &mut Memory) {
		todo!()
	}

	fn rlc(&self, reg: Register8) {
		todo!()
	}

	fn rrc(&self, reg: Register8) {
		todo!()
	}

	fn rl(&self, reg: Register8) {
		todo!()
	}

	fn rr(&self, reg: Register8) {
		todo!()
	}

	fn sla(&self, reg: Register8) {
		todo!()
	}

	fn sra(&self, reg: Register8) {
		todo!()
	}

	fn swap(&self, reg: Register8) {
		todo!()
	}

	fn srl(&self, reg: Register8) {
		todo!()
	}

	fn bit(&mut self, bit: u8, reg_val: u8) {
		self.registers.f.insert(Flag::HALF_CARRY);
		self.registers.f.remove(Flag::NEGATIVE);
		self.registers.f.set(Flag::ZERO, !is_set(reg_val, bit));
	}

	fn res(&self, pos: u8, reg: Register8) {
		todo!()
	}

	fn set(&self, pos: u8, reg: Register8) {
		todo!()
	}

	fn read_reg(&self, reg: Register8, memory: &Memory) -> u8 {
		match reg {
			Register8::A => self.registers.a,
			Register8::B => self.registers.b,
			Register8::C => self.registers.c,
			Register8::D => self.registers.d,
			Register8::E => self.registers.e,
			Register8::H => self.registers.h,
			Register8::L => self.registers.l,
			Register8::DerefHL => memory.read_byte(self.registers.get_hl()),
		}
	}

	fn write_reg(&mut self, reg: Register8, val: u8, memory: &mut Memory) {
		match reg {
			Register8::A => self.registers.a = val,
			Register8::B => self.registers.b = val,
			Register8::C => self.registers.c = val,
			Register8::D => self.registers.d = val,
			Register8::E => self.registers.e = val,
			Register8::H => self.registers.h = val,
			Register8::L => self.registers.l = val,
			Register8::DerefHL => memory.write_byte(self.registers.get_hl(), val),
		}
	}

	fn add(&mut self, n: u8) {
		let half_carry = ((self.registers.a & 0xF) + (n & 0xF)) & 0x10 == 0x10;
		let full_carry = (u16::from(self.registers.a) + u16::from(n)) & 0x100 == 0x100;

		self.registers.a = self.registers.a.wrapping_add(n);

		self.registers.f.set(Flag::ZERO, self.registers.a == 0);
		self.registers.f.remove(Flag::NEGATIVE);
		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.set(Flag::FULL_CARRY, full_carry);
	}

	fn adc(&mut self, n: u8) {
		let carry = if self.registers.f.contains(Flag::FULL_CARRY) {
			1u8
		} else {
			0u8
		};

		let half_carry = ((self.registers.a & 0xF) + (n & 0xF) + carry) & 0x10 == 0x10;
		let full_carry =
			(u16::from(self.registers.a) + u16::from(n) + u16::from(carry)) & 0x100 == 0x100;

		self.registers.a = self.registers.a.wrapping_add(n).wrapping_add(carry);

		self.registers.f.set(Flag::ZERO, self.registers.a == 0);
		self.registers.f.remove(Flag::NEGATIVE);
		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.set(Flag::FULL_CARRY, full_carry);
	}

	fn sbc(&mut self, n: u8) {
		let carry = if self.registers.f.contains(Flag::FULL_CARRY) {
			1u8
		} else {
			0u8
		};

		let result = i16::from(self.registers.a) - i16::from(n) - i16::from(carry);

		let half_carry =
			i16::from(self.registers.a & 0x0F) - i16::from(n & 0x0F) - (i16::from(carry)) < 0;
		let full_carry = result < 0;

		self.registers.a = self.registers.a.wrapping_sub(n).wrapping_sub(carry);

		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.set(Flag::FULL_CARRY, full_carry);
		self.registers.f.set(Flag::ZERO, self.registers.a == 0);
		self.registers.f.insert(Flag::NEGATIVE);
	}

	fn and(&mut self, n: u8) {
		self.registers.a &= n;

		self.registers.f.set(Flag::ZERO, self.registers.a == 0);
		self.registers.f.insert(Flag::HALF_CARRY);
		self.registers.f.remove(Flag::NEGATIVE | Flag::FULL_CARRY);
	}

	fn or(&mut self, n: u8) {
		self.registers.a |= n;

		self.registers.f.set(Flag::ZERO, self.registers.a == 0);
		self.registers
			.f
			.remove(Flag::NEGATIVE | Flag::FULL_CARRY | Flag::HALF_CARRY);
	}

	fn xor(&mut self, n: u8) {
		self.registers.a ^= n;

		self.registers.f.set(Flag::ZERO, self.registers.a == 0);
		self.registers
			.f
			.remove(Flag::NEGATIVE | Flag::HALF_CARRY | Flag::FULL_CARRY);
	}

	fn cp(&mut self, n: u8) {
		let half_carry = (self.registers.a & 0xF) < (n & 0xF);
		let overflow = self.registers.a < n;

		self.registers.f.set(Flag::FULL_CARRY, overflow);
		self.registers.f.set(Flag::ZERO, self.registers.a == n);
		self.registers.f.set(Flag::HALF_CARRY, half_carry);
		self.registers.f.insert(Flag::NEGATIVE);
	}
}
