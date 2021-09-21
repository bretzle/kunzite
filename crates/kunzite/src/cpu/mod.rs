//! The cpu

use crate::{
	cpu::instruction::{Flag, Instruction, Register16},
	memory::Memory,
	util::{lower, upper},
};

use self::{instruction::Register8, register::Registers};

mod decode;
pub mod instruction;
mod register;

/// The cpu
#[derive(Default)]
pub struct Cpu {
	/// Program counter
	pub pc: u16,
	/// Cpu registers
	pub registers: Registers,

	pub memory: Memory,

	pub tick: u8, // T-cycle
	pub halted: bool,
	ime: bool,
}

macro_rules! update_flags {
	(
		$self:ident,
		$( z: $z:expr, )?
		$( n: $n:expr, )?
		$( h: $h:expr, )?
		$( c: $c:expr, )?
	) => {
		$( $self.registers.set_flag(Flag::Z, $z as u8); )?
		$( $self.registers.set_flag(Flag::N, $n as u8); )?
		$( $self.registers.set_flag(Flag::H, $h as u8); )?
		$( $self.registers.set_flag(Flag::C, $c as u8); )?
	}
}

impl Cpu {
	fn trace(&self) {
		let a = self.read(Register8::A);
		let f = self.registers.flags();
		let b = self.read(Register8::B);
		let c = self.read(Register8::C);
		let d = self.read(Register8::D);
		let e = self.read(Register8::E);
		let h = self.read(Register8::H);
		let l = self.read(Register8::L);

		let sp = self.registers[Register16::SP];
		let pc = self.pc;

		let m0 = self.memory.read(pc + 0);
		let m1 = self.memory.read(pc + 1);
		let m2 = self.memory.read(pc + 2);
		let m3 = self.memory.read(pc + 3);

		println!(
			"A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: \
			 {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
			a, f, b, c, d, e, h, l, sp, pc, m0, m1, m2, m3
		)
	}

	/// Execute an instruction and increment pc
	pub fn step(&mut self) -> u8 {
		// self.trace();
		let mut total_tick = 0;

		self.tick = 0;

		if self.halted {
			self.tick += 4;
		} else if let Some(inst) = self.parse_instruction() {
			self.pc += inst.size();
			self.execute(inst);
		} else {
			panic!()
		}

		total_tick += self.tick;

		self.memory.update(self.tick);

		if self.ime {
			self.tick = 0;
			self.check_irqs();
			self.memory.update(self.tick);

			total_tick += self.tick;
		}

		total_tick
	}

	/// Checks IRQs and execute ISRs if requested.
	fn check_irqs(&mut self) {
		// Bit 0 has the highest priority
		for i in 0..5 {
			let irq = self.memory.int_flag & (1 << i) > 0;
			let ie = self.memory.int_enable & (1 << i) > 0;

			// If interrupt is requested and enabled
			if irq && ie {
				self.call_isr(i);
				break;
			}
		}
	}

	/// Calls requested interrupt service routine.
	fn call_isr(&mut self, id: u8) {
		// Reset corresponding bit in IF
		self.memory.int_flag &= !(1 << id);
		// Clear IME (disable any further interrupts)
		self.ime = false;
		self.halted = false;

		let isr = match id {
			0 => 0x40,
			1 => 0x48,
			2 => 0x50,
			3 => 0x80,
			4 => 0x70,
			_ => panic!("Invalid IRQ id {}", id),
		};

		self.tick += 8;

		self._call(isr);
	}

	fn execute(&mut self, instruction: Instruction) {
		self.tick += instruction.ticks();
		match instruction {
			Instruction::Nop => {} // TODO: does this do anything?
			Instruction::Stop => todo!("{:?}", instruction),
			Instruction::Halt => self.halted = true,
			Instruction::StoreImm16(reg, val) => {
				self.registers[reg] = val;
			}
			Instruction::StoreImm8(reg, val) => self.write(reg, val),
			Instruction::StoreAToHlAddr(inc) => {
				let hl = self.registers[Register16::HL];
				let val = self.read(Register8::A);
				self.memory.write(hl, val);

				let hl = &mut self.registers[Register16::HL];
				if inc {
					*hl = hl.wrapping_add(1);
				} else {
					*hl = hl.wrapping_sub(1);
				}
			}
			Instruction::LoadAFromHlAddr(inc) => {
				let hl = self.registers[Register16::HL];
				self.write(Register8::A, self.memory.read(hl));

				let hl = &mut self.registers[Register16::HL];
				if inc {
					*hl += 1;
				} else {
					*hl -= 1;
				}
			}
			Instruction::StoreATo16(reg) => {
				let addr = self.registers[reg];
				let val = self.read(Register8::A);
				self.memory.write(addr, val);
			}
			Instruction::LoadAFromReg16Addr(reg) => {
				let addr = self.registers[reg];
				self.write(Register8::A, self.memory.read(addr));
			}
			Instruction::Mov8(dest, src) => self.write(dest, self.read(src)),
			Instruction::Jr(f, r) => match f {
				Some(flag) => {
					if self.registers.flag(flag) {
						self._jr(r);
					}
				}
				None => self._jr(r -2 ),
			},
			Instruction::Jp(f, addr) => match f {
				Some(flag) => {
					if self.registers.flag(flag) {
						self._jp(addr);
					}
				}
				None => self._jp(addr),
			},
			Instruction::Inc8(reg) => {
				let orig = self.read(reg);
				let new = orig.wrapping_add(1);
				self.write(reg, new);
				update_flags! {
					self,
					z: new == 0,
					n: 0,
					h: orig & 0xF == 0xF,
				};
			}
			Instruction::Dec8(reg) => {
				let orig = self.read(reg);
				let new = orig.wrapping_sub(1);
				self.write(reg, new);
				update_flags! {
					self,
					z: new == 0,
					n: 1,
					h: orig & 0xF == 0,
				}
			}
			Instruction::Inc16(reg) => self.registers[reg] = self.registers[reg].wrapping_add(1),
			Instruction::Dec16(reg) => self.registers[reg] = self.registers[reg].wrapping_sub(1),
			Instruction::Push(reg) => {
				let regs = reg.tear();
				self.push(self.read(regs.0));
				self.push(self.read(regs.1));
			}
			Instruction::Pop(reg) => {
				let regs = reg.tear();
				let a = self.pop();
				let b = self.pop();
				self.write(regs.1, a);
				self.write(regs.0, b);
			}
			Instruction::Add(reg) => self._add(self.read(reg)),
			Instruction::Adc(reg) => self._adc(self.read(reg)),
			Instruction::Sub(reg) => self._sub(self.read(reg)),
			Instruction::Sbc(reg) => self._sbc(self.read(reg)),
			Instruction::And(reg) => self._and(self.read(reg)),
			Instruction::Xor(reg) => self._xor(self.read(reg)),
			Instruction::Or(reg) => self._or(self.read(reg)),
			Instruction::Cp(reg) => self._cp(self.read(reg)),
			Instruction::Add8(val) => self._add(val),
			Instruction::Adc8(val) => self._adc(val),
			Instruction::Sub8(val) => self._sub(val),
			Instruction::Sbc8(val) => self._sbc(val),
			Instruction::And8(val) => self._and(val),
			Instruction::Xor8(val) => self._xor(val),
			Instruction::Or8(val) => self._or(val),
			Instruction::Cp8(val) => self._cp(val),
			Instruction::AddSp8(_) => todo!("{:?}", instruction),
			Instruction::Daa => todo!("{:?}", instruction),
			Instruction::Scf => todo!("{:?}", instruction),
			Instruction::Cpl => {
				self.write(Register8::A, !self.read(Register8::A));
				update_flags! {
					self,
					n: 1,
					h: 1,
				}
			}
			Instruction::Ccf => {
				update_flags! {
					self,
					n: false,
					h: false,
					c: !self.registers.flag(Flag::C),
				}
			}
			Instruction::Rlca => {
				self._rlc(Register8::A);
				update_flags! {
					self,
					z: false,
				}
			}
			Instruction::Rla => {
				self._rl(Register8::A);

				update_flags! {
					self,
					z: false,
				}
			}
			Instruction::Rrca => todo!("{:?}", instruction),
			Instruction::Rra => {
				self._rr(Register8::A);

				update_flags! {
					self,
					z: false,
				}
			}
			Instruction::StoreImm16AddrSp(_) => todo!("{:?}", instruction),
			Instruction::AddHl(reg) => {
				let hl = self.registers[Register16::HL];
				let val = self.registers[reg];

				let half_carry = (hl & 0xFFF) + (val & 0xFFF) > 0xFFF;
				let (res, carry) = hl.overflowing_add(val);
				self.registers[Register16::HL] = res;

				update_flags! {
					self,
					n: false,
					h: half_carry,
					c: carry,
				}
			}
			Instruction::Ret(f) => match f {
				Some(flag) => {
					if self.registers.flag(flag) {
						self._ret()
					}
				}
				None => self._ret(),
			},
			Instruction::Reti => {
				self.ime = false;
				self._ret()
			}
			Instruction::Di => self.ime = false,
			Instruction::Ei => self.ime = true,
			Instruction::Call(f, jump) => match f {
				Some(flag) => {
					if self.registers.flag(flag) {
						self._call(jump)
					}
				}
				None => self._call(jump),
			},
			Instruction::JpHl => {
				self.pc = self.registers[Register16::HL];
			}
			Instruction::Rst(val) => {
				self.push(upper(self.pc + 1));
				self.push(lower(self.pc + 1));
				self.pc = val as u16;
			}
			Instruction::LdHlSp8(_) => todo!("{:?}", instruction),
			Instruction::LdSpHl => todo!("{:?}", instruction),
			Instruction::StoreHA(offset) => {
				let addr = 0xFF00 + offset as u16;
				let val = self.read(Register8::A);
				self.memory.write(addr, val);
			}
			Instruction::LoadHA(offset) => {
				self.write(Register8::A, self.memory.read(0xFF00 + offset as u16));
			}
			Instruction::StoreCA => {
				let addr = 0xFF00 + self.read(Register8::C) as u16;
				let val = self.read(Register8::A);
				self.memory.write(addr, val);
			}
			Instruction::LoadCA => todo!("{:?}", instruction),
			Instruction::StoreAAtAddress(addr) => {
				let val = self.read(Register8::A);
				self.memory.write(addr, val);
			}
			Instruction::LoadAFromAddress(addr) => {
				self.write(Register8::A, self.memory.read(addr));
			}
			Instruction::Rlc(reg) => self._rlc(reg),
			Instruction::Rrc(reg) => self._rrc(reg),
			Instruction::Rr(reg) => self._rr(reg),
			Instruction::Rl(reg) => self._rl(reg),
			Instruction::Sla(_) => todo!("{:?}", instruction),
			Instruction::Sra(_) => todo!("{:?}", instruction),
			Instruction::Swap(reg) => {
				let orig = self.read(reg);

				let upper = orig & 0xF0;
				let lower = orig & 0x0F;
				let val = (lower << 4) | (upper >> 4);

				self.write(reg, val);

				update_flags! {
					self,
					z: val == 0,
					n: 0,
					h: 0,
					c: 0,
				}
			}
			Instruction::Srl(reg) => self.srl(reg),
			Instruction::Bit(bit, reg) => {
				let set = self.read(reg) & (1 << bit) != 0;
				update_flags! {
					self,
					z: !set,
					n: 0,
					h: 1,
				}
			}
			Instruction::Res(bit, reg) => {
				let orig = self.read(reg);
				let val = orig & !(1 << bit);
				self.write(reg, val);
			}
			Instruction::Set(bit, reg) => {
				let orig = self.read(reg);
				let val = orig | (1 << bit);
				self.write(reg, val);
			}
		}
	}

	fn push(&mut self, val: u8) {
		let sp = &mut self.registers[Register16::SP];

		*sp = sp.wrapping_sub(1);

		self.memory.write(*sp, val);
	}

	fn pop(&mut self) -> u8 {
		let sp = &mut self.registers[Register16::SP];

		let val = self.memory.read(*sp);

		*sp = sp.wrapping_add(1);

		val
	}
}

impl Cpu {
	fn _add(&mut self, val: u8) {
		let orig = self.read(Register8::A);
		let half_carry = (orig & 0xf) + (val & 0xf) > 0xf;
		let (res, carry) = orig.overflowing_add(val);

		self.write(Register8::A, res);

		update_flags! {
			self,
			z: res == 0,
			n: 0,
			h: half_carry,
			c: carry,
		}
	}

	fn _adc(&mut self, val: u8) {
		let orig = self.read(Register8::A);
		let c = if self.registers.flag(Flag::C) { 1 } else { 0 };

		let res = orig.wrapping_add(val).wrapping_add(c);
		let half_carry = (orig & 0xf) + (val & 0xf) + c > 0xf;
		let carry = (orig as u16) + (val as u16) + (c as u16) > 0xff;

		self.write(Register8::A, res);

		update_flags! {
			self,
			z: res == 0,
			n: 0,
			h: half_carry,
			c: carry,
		}
	}

	fn _sub(&mut self, val: u8) {
		let orig = self.read(Register8::A);

		let half_carry = (orig & 0xf) < (val & 0xf);
		let (res, carry) = orig.overflowing_sub(val);

		self.write(Register8::A, res);

		update_flags! {
			self,
			z: res == 0,
			n: 1,
			h: half_carry,
			c: carry,
		}
	}

	fn _sbc(&mut self, val: u8) {
		let c = if self.registers.flag(Flag::C) { 1 } else { 0 };
		let orig = self.read(Register8::A);

		let res = orig.wrapping_sub(val).wrapping_sub(c);
		let half_carry = (orig & 0xf) < (val & 0xf) + c;
		let carry = (orig as u16) < (val as u16) + (c as u16);

		self.write(Register8::A, res);

		update_flags! {
			self,
			z: res == 0,
			n: 1,
			h: half_carry,
			c: carry,
		}
	}

	fn _and(&mut self, val: u8) {
		let a = self.read(Register8::A);
		self.write(Register8::A, a & val);

		update_flags! {
			self,
			z: self.read(Register8::A) == 0,
			n: false,
			h: true,
			c: false,
		}
	}

	fn _xor(&mut self, val: u8) {
		let a = self.read(Register8::A);
		self.write(Register8::A, a ^ val);

		update_flags! {
			self,
			z: self.read(Register8::A) == 0,
			n: 0,
			h: 0,
			c: 0,
		}
	}

	fn _or(&mut self, val: u8) {
		let a = self.read(Register8::A);
		self.write(Register8::A, a | val);

		update_flags! {
			self,
			z: self.read(Register8::A) == 0,
			n: false,
			h: false,
			c: false,
		}
	}

	fn _cp(&mut self, val: u8) {
		let a = self.read(Register8::A);

		update_flags! {
			self,
			z: a == val,
			n: 1,
			h: a & 0xF < val & 0xF,
			c: a < val,
		}
	}

	fn _call(&mut self, jump: u16) {
		self.push(upper(self.pc));
		self.push(lower(self.pc));
		self.pc = jump;
		self.tick += 12;
	}

	fn _jr(&mut self, offset: i8) {
		self.tick += 4;
		self.pc = self.pc.wrapping_add(offset as u16);
	}

	fn _jp(&mut self, addr: u16) {
		self.tick += 4;
		self.pc = addr;
	}

	fn _rl(&mut self, reg: Register8) {
		let orig = self.read(reg);
		let carry = self.registers.carry();
		let res = (orig << 1) | (if carry { 1 } else { 0 });
		self.write(reg, res);

		update_flags! {
			self,
			z: res == 0,
			n: false,
			h: false,
			c: orig >> 7 & 1 == 1,
		}
	}

	fn _rr(&mut self, reg: Register8) {
		let orig = self.read(reg);
		let carry = self.registers.carry();
		let res = (orig >> 1) | (if carry { 1 } else { 0 } << 7);
		self.write(reg, res);

		update_flags! {
			self,
			z: res == 0,
			n: false,
			h: false,
			c: orig & 1 == 1,
		}
	}

	fn _rrc(&mut self, _reg: Register8) {
		todo!()
	}

	fn _rlc(&mut self, reg: Register8) {
		let orig = self.read(reg);
		let res = orig.rotate_left(1);
		self.write(reg, res);

		update_flags! {
			self,
			z: res == 0,
			n: false,
			h: false,
			c: orig >> 7 & 1 == 1,
		}
	}

	/// Shift right through carry
	fn srl(&mut self, reg: Register8) {
		let orig = self.read(reg);
		let res = orig >> 1;
		self.write(reg, res);

		update_flags! {
			self,
			z: res == 0,
			n: false,
			h: false,
			c: orig & 1 == 1,
		}
	}

	fn _ret(&mut self) {
		let lower = self.pop() as u16;
		let upper = self.pop() as u16;
		self.pc = (upper << 8) | lower;
		self.tick += 12;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! test_instructions {
		(
			$({
			setup = {
				$inst:expr;
				$( $reg:ident = $value:expr ),*
			}
			output = {
				$( $out_reg:ident == $out_value:expr ),*
			}
			flags = {
				$( z: $z:expr, )?
				$( n: $n:expr, )?
				$( h: $h:expr, )?
				$( c: $c:expr, )?
			}
			}),*
		) => {$({
			let mut cpu = Cpu::default();
			$(
				cpu.write($reg, $value);
			)*

			cpu.execute($inst);

			$( assert_eq!( cpu.read($out_reg), $out_value, "Register {:?} is wrong", $out_reg ); )*

			$( assert_eq!(cpu.registers.flag(Flag::Z), $z, "Zero flag is wrong"); )?
			$( assert_eq!(cpu.registers.flag(Flag::N), $n, "Subtract flag is wrong"); )?
			$( assert_eq!(cpu.registers.flag(Flag::H), $h, "Half-Carry flag is wrong"); )?
			$( assert_eq!(cpu.registers.flag(Flag::C), $c, "Carry flag is wrong"); )?
		})*}
	}

	const A: Register8 = Register8::A;
	const B: Register8 = Register8::B;
	const C: Register8 = Register8::C;
	const BC: Register16 = Register16::BC;

	#[test]
	fn test_inc() {
		test_instructions! [
			{
				setup = {
					Instruction::Inc8(A);
					A = 0x7
				}
				output = { A == 0x8 }
				flags = { z: false, n: false, h: false, c: false, }
			},
			{
				setup = {
					Instruction::Inc8(A);
					A = 0xF
				}
				output = { A == 0x10 }
				flags = { z: false, n: false, h: true, c: false, }
			},
			{
				setup = {
					Instruction::Inc8(A);
					A = 0xFF
				}
				output = { A == 0x00 }
				flags = { z: true, n: false, h: true, c: false, }
			},
			{
				setup = {
					Instruction::Inc16(BC);
					B = 0x00,
					C = 0xFF
				}
				output = { B == 0x01, C == 0x00 }
				flags = { z: false, n: false, h: false, c: false, }
			},
			{
				setup = {
					Instruction::Inc16(BC);
					B = 0xFF,
					C = 0xFF
				}
				output = { B == 0x00, C == 0x00 }
				flags = { z: false, n: false, h: false, c: false, }
			}
		];
	}

	#[test]
	fn test_dec() {
		test_instructions! [
			{
				setup = {
					Instruction::Dec8(A);
					A = 0x7
				}
				output = { A == 0x6 }
				flags = { z: false, n: true, h: false, c: false, }
			},
			{
				setup = {
					Instruction::Dec8(A);
					A = 0x80
				}
				output = { A == 0x7F }
				flags = { z: false, n: true, h: true, c: false, }
			},
			{
				setup = {
					Instruction::Dec8(A);
					A = 0x00
				}
				output = { A == 0xFF }
				flags = { z: false, n: true, h: true, c: false, }
			},
			{
				setup = {
					Instruction::Dec16(BC);
					B = 0x00,
					C = 0x00
				}
				output = { B == 0xFF, C == 0xFF }
				flags = { z: false, n: false, h: false, c: false, }
			}
		];
	}

	#[test]
	fn test_add() {
		test_instructions! [
			{
				setup = {
					Instruction::Add(A);
					A = 0x07
				}
				output = { A == 0x0E }
				flags = { z: false, n: false, h: false, c: false, }
			},
			{
				setup = {
					Instruction::Add(C);
					A = 0x07,
					C = 0x03
				}
				output = { A == 0x0A }
				flags = { z: false, n: false, h: false, c: false, }
			},
			{
				setup = {
					Instruction::Add(C);
					A = 0x07,
					C = 0x03
				}
				output = { A == 0x0A }
				flags = { z: false, n: false, h: false, c: false, }
			},
			{
				setup = {
					Instruction::Add(B);
					A = 0xFC,
					B = 0x09
				}
				output = { A == 0x05 }
				flags = { z: false, n: false, h: true, c: true, }
			}
		];
	}
}
