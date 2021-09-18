use std::ops::{Index, IndexMut};

use super::{
	instruction::{Flag, Register16, Register8},
	Cpu,
};

#[repr(C)]
#[derive(Clone, Copy)]
struct Reg8s {
	f: u8,
	a: u8,
	c: u8,
	b: u8,
	e: u8,
	d: u8,
	l: u8,
	h: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Reg16s {
	af: u16,
	bc: u16,
	de: u16,
	hl: u16,
	sp: u16,
}

#[repr(C)]
pub union Registers {
	reg8: Reg8s,
	reg16: Reg16s,
}

impl Registers {
	fn read16(&self, reg: Register16) -> &u16 {
		unsafe {
			match reg {
				Register16::BC => &self.reg16.bc,
				Register16::DE => &self.reg16.de,
				Register16::HL => &self.reg16.hl,
				Register16::AF => &self.reg16.af,
				Register16::SP => &self.reg16.sp,
			}
		}
	}

	fn read16_mut(&mut self, reg: Register16) -> &mut u16 {
		unsafe {
			match reg {
				Register16::BC => &mut self.reg16.bc,
				Register16::DE => &mut self.reg16.de,
				Register16::HL => &mut self.reg16.hl,
				Register16::AF => &mut self.reg16.af,
				Register16::SP => &mut self.reg16.sp,
			}
		}
	}

	pub fn flags(&self) -> u8 {
		unsafe { self.reg8.f }
	}

	pub fn flag(&self, flag: Flag) -> bool {
		let f = unsafe { self.reg8.f };
		match flag {
			Flag::Z => f & 0x80 != 0,
			Flag::NZ => f & 0x80 == 0,
			Flag::N => f & 0x40 != 0,
			Flag::H => f & 0x20 != 0,
			Flag::C => f & 0x10 != 0,
			Flag::NC => f & 0x10 == 0,
		}
	}

	pub fn set_flag(&mut self, flag: Flag, val: u8) {
		debug_assert!(val == 0 || val == 1);

		let f = unsafe { &mut self.reg8.f };

		match flag {
			Flag::Z => {
				*f &= 0x7F;
				*f |= val << 7;
			}
			Flag::N => {
				*f &= 0xBF;
				*f |= val << 6;
			}
			Flag::H => {
				*f &= 0xDF;
				*f |= val << 5;
			}
			Flag::C => {
				*f &= 0xEF;
				*f |= val << 4;
			}
			_ => unreachable!(),
		}
	}

	pub fn carry(&self) -> bool {
		self.flag(Flag::C)
	}
}

// impl Index<Register8> for Registers {
// 	type Output = u8;

// 	fn index(&self, reg: Register8) -> &Self::Output {
// 		self.read(reg)
// 	}
// }

// impl IndexMut<Register8> for Registers {
// 	fn index_mut(&mut self, reg: Register8) -> &mut Self::Output {
// 		self.read_mut(reg)
// 	}
// }

impl Index<Register16> for Registers {
	type Output = u16;

	fn index(&self, reg: Register16) -> &Self::Output {
		self.read16(reg)
	}
}

impl IndexMut<Register16> for Registers {
	fn index_mut(&mut self, reg: Register16) -> &mut Self::Output {
		self.read16_mut(reg)
	}
}

impl Default for Registers {
	fn default() -> Self {
		Self {
			reg16: Reg16s::default(),
		}
	}
}

impl Cpu {
	pub fn read(&self, reg: Register8) -> u8 {
		unsafe {
			match reg {
				Register8::DerefHL => self.memory.read(self.registers[Register16::HL]),
				Register8::A => self.registers.reg8.a,
				Register8::B => self.registers.reg8.b,
				Register8::C => self.registers.reg8.c,
				Register8::D => self.registers.reg8.d,
				Register8::E => self.registers.reg8.e,
				Register8::H => self.registers.reg8.h,
				Register8::L => self.registers.reg8.l,
				Register8::F => self.registers.reg8.f,
			}
		}
	}

	pub fn write(&mut self, reg: Register8, val: u8) {
		match reg {
			Register8::DerefHL => self.memory.write(self.registers[Register16::HL], val),
			Register8::A => self.registers.reg8.a = val,
			Register8::B => self.registers.reg8.b = val,
			Register8::C => self.registers.reg8.c = val,
			Register8::D => self.registers.reg8.d = val,
			Register8::E => self.registers.reg8.e = val,
			Register8::H => self.registers.reg8.h = val,
			Register8::L => self.registers.reg8.l = val,
			Register8::F => self.registers.reg8.f = val,
		}
	}
}
